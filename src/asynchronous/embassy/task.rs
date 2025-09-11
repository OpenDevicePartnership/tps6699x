use embassy_sync::blocking_mutex::raw::RawMutex;
use embedded_hal::digital::InputPin;
use embedded_hal_async::digital::Wait;
use embedded_hal_async::i2c::I2c;

use super::Interrupt;
use crate::{error, trace, warn};

/// Task to process all given interrupts
pub async fn interrupt_task<M: RawMutex, B: I2c, INT: Wait + InputPin>(
    int: &mut INT,
    interrupts: &mut [&mut Interrupt<'_, M, B>],
) {
    let mut retry_strategy = retry_strategy::ExponentialBackoff::default();
    loop {
        if int.wait_for_low().await.is_err() {
            error!("Error waiting for interrupt");
            continue;
        }

        for interrupt in &mut *interrupts {
            if interrupt.process_interrupt(int).await.is_err() {
                warn!("Error processing interrupt");
            }

            if let Ok(true) = int.is_high() {
                // Done handling pending interrupts
                break;
            }
        }

        // If interrupt line is still asserted, retry following backoff strategy
        match int.is_low() {
            Ok(true) => {
                match retry_strategy.next() {
                    None => {
                        trace!("Interrupt line still asserted, retrying immediately");
                    }
                    Some(backoff) => {
                        // If this was not the first try, back off
                        trace!("Interrupt line still asserted, backing off for {:?}", backoff);
                        embassy_time::Timer::after(backoff).await;
                    }
                }
            }
            Ok(false) => {
                // Interrupt line is no longer asserted, reset backoff
                retry_strategy = Default::default();
            }
            Err(_) => {
                error!("Error post-checking interrupt line");
            }
        }
    }
}

mod retry_strategy {
    use embassy_time::Duration;

    /// How to back off when the interrupt line remains asserted after processing all interrupts.
    ///
    /// The interrupt line may remain asserted after processing all interrupts due to an error (possibly transient),
    /// interrupts being disabled, or a new interrupt that came in very recently.
    /// When this happens, back off for a bit to allow other tasks to run, otherwise [`super::interrupt_task`] may
    /// starve other tasks.
    ///
    /// This implements an exponential backoff strategy so transient errors are retried quickly.
    /// However, the first retry is immediate in case there was a new interrupt that came in very recently.
    pub struct ExponentialBackoff {
        /// Whether [`Self::next`] should return immediately with [`None`].
        ///
        /// This is `true` initially, and set to `false` after the first call to [`Self::next`].
        instant: bool,

        /// The next backoff duration to use.
        next_backoff: Duration,

        /// The maximum backoff duration to use.
        max_backoff: Duration,
    }

    impl Default for ExponentialBackoff {
        fn default() -> Self {
            Self {
                instant: true,
                next_backoff: Duration::from_millis(1),
                max_backoff: Duration::from_millis(100),
            }
        }
    }

    impl ExponentialBackoff {
        /// Get the next backoff duration.
        pub fn next(&mut self) -> Option<Duration> {
            if core::mem::take(&mut self.instant) {
                None
            } else {
                let next_backoff = self
                    .next_backoff
                    .checked_mul(2)
                    .unwrap_or(Duration::MAX)
                    .min(self.max_backoff);

                Some(core::mem::replace(&mut self.next_backoff, next_backoff))
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// The default state should use an instant first try and a positive non-zero backoff.
        #[test]
        fn default_is_instant_positive() {
            let strategy = ExponentialBackoff::default();
            assert!(strategy.instant, "First try should be true");
            assert!(
                strategy.next_backoff > Duration::MIN,
                "Next backoff should be positive non-zero (got {:?})",
                strategy.next_backoff
            );
        }

        #[test]
        fn instant_first_try() {
            let mut strategy = ExponentialBackoff {
                instant: true,
                next_backoff: Duration::from_millis(1),
                max_backoff: Duration::from_millis(100),
            };

            assert_eq!(strategy.next(), None, "First try should be instant");
            assert_ne!(strategy.next(), None, "Second try should not be instant");
        }

        #[test]
        fn exponential_backoff() {
            let first = Duration::from_millis(1);
            let mut strategy = ExponentialBackoff {
                instant: false,
                next_backoff: first,
                max_backoff: Duration::from_millis(100),
            };

            assert_eq!(strategy.next(), Some(first * 1));
            assert_eq!(strategy.next(), Some(first * 2));
            assert_eq!(strategy.next(), Some(first * 4));
            assert_eq!(strategy.next(), Some(first * 8));
            assert_eq!(strategy.next(), Some(first * 16));
            assert_eq!(strategy.next(), Some(first * 32));
            assert_eq!(strategy.next(), Some(first * 64));
            // Next would be 128ms, but capped at 100ms, and that's another test case.
        }

        #[test]
        fn max_backoff() {
            let first = Duration::from_millis(99);
            let mut strategy = ExponentialBackoff {
                instant: false,
                next_backoff: first,
                max_backoff: first + Duration::from_millis(1),
            };

            assert_eq!(strategy.next(), Some(first));
            assert_eq!(
                strategy.next(),
                Some(strategy.max_backoff),
                "Backoff should cap at max_backoff"
            );
            assert_eq!(
                strategy.next(),
                Some(strategy.max_backoff),
                "Backoff should still cap at max_backoff"
            );
        }

        /// No panic should occur on overflow, and backoff should cap at `Duration::MAX.min(max_backoff)`.
        #[test]
        fn overflow() {
            let first = Duration::MAX / 2 + Duration::from_millis(1);
            let mut strategy = ExponentialBackoff {
                instant: false,
                next_backoff: first,
                max_backoff: first + Duration::from_millis(2),
            };

            assert_eq!(strategy.next(), Some(first));
            assert_eq!(
                strategy.next(),
                Some(strategy.max_backoff),
                "Backoff should cap at max_backoff on overflow"
            );
        }
    }
}
