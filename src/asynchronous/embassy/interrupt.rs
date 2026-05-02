//! Interrupt related code.
use core::array::from_fn;

use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_time::{with_timeout, Duration};
use embedded_hal::digital::InputPin;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::{Error, LocalPortId, PdError};
use itertools::izip;

use crate::asynchronous::embassy::controller::Controller;
use crate::registers::field_sets::IntEventBus1;
use crate::{error, trace, warn, MAX_SUPPORTED_PORTS};

/// Configuration for [`InterruptProcessor`]
#[non_exhaustive]
pub struct Config {
    pub interrupt_timeout: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            interrupt_timeout: Duration::from_millis(100),
        }
    }
}

/// Struct for processing interrupts from the TPS6699x.
pub struct InterruptProcessor<'a, M: RawMutex, B: I2c> {
    pub(super) controller: &'a Controller<M, B>,
}

impl<'a, M: RawMutex, B: I2c> InterruptProcessor<'a, M, B> {
    /// Process interrupts
    pub async fn process_interrupt(
        &mut self,
        int: &mut impl InputPin,
    ) -> Result<[IntEventBus1; MAX_SUPPORTED_PORTS], Error<B::Error>> {
        let timeout = self.controller.config.interrupt_processor_config.interrupt_timeout;
        let mut flags = self
            .controller
            .interrupt_waker
            .try_take()
            .unwrap_or([IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS]);

        let interrupts_enabled = self.controller.interrupts_enabled();
        let mut inner = self.controller.inner.lock().await;

        // Note: `interrupts_enabled` and `flags` are both of size MAX_SUPPORTED_PORTS and so
        // will always have a 1:1 mapping. If `num_ports` ever returns a value larger than
        // MAX_SUPPORTED_PORTS, `port` will simply be capped at MAX_SUPPORTED_PORTS.
        for (port, (interrupt_enabled, flag, command_complete)) in izip!(
            interrupts_enabled.iter(),
            flags.iter_mut(),
            self.controller.command_complete.iter()
        )
        .take(self.controller.num_ports)
        .enumerate()
        {
            let port_id = LocalPortId(port as u8);

            if !interrupt_enabled {
                trace!("{:?}: Interrupts disabled", port_id);
                continue;
            }

            match int.is_high() {
                Ok(true) => {
                    // Early exit if checking the last port cleared the interrupt
                    trace!("Interrupt line is high, exiting");
                    break;
                }
                Err(_) => {
                    error!("Failed to read interrupt line");
                    return PdError::Failed.into();
                }
                _ => {}
            }

            match with_timeout(timeout, inner.clear_interrupt(port_id)).await {
                Ok(res) => match res {
                    Ok(event) => {
                        *flag |= event;
                        if event.cmd_1_completed() {
                            command_complete.signal(());
                        }
                    }
                    Err(_) => {
                        error!("{:?}: clear_interrupt failed", port_id);
                        continue;
                    }
                },
                Err(_) => {
                    error!("{:?}: clear_interrupt timeout", port_id);
                    continue;
                }
            }
        }

        self.controller.interrupt_waker.signal(flags);
        Ok(flags)
    }
}

/// Restores the original interrupt state when dropped
pub struct InterruptGuard<'a, M: RawMutex, B: I2c> {
    target_state: [bool; MAX_SUPPORTED_PORTS],
    controller: &'a Controller<M, B>,
}

impl<'a, M: RawMutex, B: I2c> InterruptGuard<'a, M, B> {
    pub(super) fn new(controller: &'a Controller<M, B>, enabled: [bool; MAX_SUPPORTED_PORTS]) -> Self {
        let target_state = controller.interrupts_enabled();
        controller.enable_interrupts(enabled);
        Self {
            target_state,
            controller,
        }
    }
}

impl<M: RawMutex, B: I2c> Drop for InterruptGuard<'_, M, B> {
    fn drop(&mut self) {
        self.controller.enable_interrupts(self.target_state);
    }
}

impl<M: RawMutex, B: I2c> crate::asynchronous::interrupt::InterruptGuard for InterruptGuard<'_, M, B> {}

/// Struct to ensure drop-safety of [`InterruptReceiver::wait_any_masked`]
///
/// This struct re-signals any unhandled interrupts on drop.
struct AccumulatedFlagsAny<'a, M: RawMutex, B: I2c> {
    controller: &'a Controller<M, B>,
    accumulated_flags: [IntEventBus1; MAX_SUPPORTED_PORTS],
    masks: [IntEventBus1; MAX_SUPPORTED_PORTS],
}

impl<'a, M: RawMutex, B: I2c> AccumulatedFlagsAny<'a, M, B> {
    fn new(controller: &'a Controller<M, B>, masks: [IntEventBus1; MAX_SUPPORTED_PORTS]) -> Self {
        AccumulatedFlagsAny {
            controller,
            accumulated_flags: [IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS],
            masks,
        }
    }

    fn accumulate(
        &mut self,
        flags: [IntEventBus1; MAX_SUPPORTED_PORTS],
    ) -> Option<[IntEventBus1; MAX_SUPPORTED_PORTS]> {
        let mut done = false;
        for (&flags, &mask, accumulated) in izip!(flags.iter(), self.masks.iter(), self.accumulated_flags.iter_mut(),) {
            *accumulated |= flags;
            let consumed_flags = flags & mask;
            if consumed_flags != IntEventBus1::new_zero() {
                done = true;
            }
        }

        if done {
            // Panic safety: the return type, `accumulated_flags`, and `mask` are all of size MAX_SUPPORTED_PORTS
            // so this will never index out of bounds
            #[allow(clippy::indexing_slicing)]
            let handled = from_fn(|i| self.accumulated_flags[i] & self.masks[i]);
            // Put unhandled flags back for signaling in `drop()`
            self.accumulated_flags = from_fn(|i| self.accumulated_flags[i] & !self.masks[i]);
            Some(handled)
        } else {
            None
        }
    }
}

impl<M: RawMutex, B: I2c> Drop for AccumulatedFlagsAny<'_, M, B> {
    fn drop(&mut self) {
        // Catch any flags that may have happened since the last accumulate.
        let new = self
            .controller
            .interrupt_waker
            .try_take()
            .unwrap_or([IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS]);
        // Panic safety: `unhandled`, `accumulated_flags`, and `mask` are all of size MAX_SUPPORTED_PORTS
        // so this will never index out of bounds
        #[allow(clippy::indexing_slicing)]
        let unhandled = from_fn(|i| self.accumulated_flags[i] | new[i]);

        // Put back any unhandled interrupt flags for future processing
        if unhandled.iter().any(|&f| f != IntEventBus1::new_zero()) {
            // If there are unhandled flags, signal them for future processing
            trace!("Signaling unhandled interrupt flags: {:?}", unhandled);
            self.controller.interrupt_waker.signal(unhandled);
        }
    }
}

/// Struct used to receive interrupts from the TPS6699x.
///
///
pub struct InterruptReceiver<'a, M: RawMutex, B: I2c> {
    pub(super) controller: &'a Controller<M, B>,
}

impl<'a, M: RawMutex, B: I2c> InterruptReceiver<'a, M, B> {
    /// Wait for an interrupt to occur.
    ///
    /// Drop safety: Safe, unhandled interrupts will be re-signaled.
    pub async fn wait_any(&mut self, clean_current: bool) -> [IntEventBus1; MAX_SUPPORTED_PORTS] {
        self.wait_any_masked(clean_current, [IntEventBus1::all(); MAX_SUPPORTED_PORTS])
            .await
    }

    /// Wait for an interrupt to occur that matches any bits in the given mask.
    pub async fn wait_any_masked(
        &mut self,
        clear_current: bool,
        mask: [IntEventBus1; MAX_SUPPORTED_PORTS],
    ) -> [IntEventBus1; MAX_SUPPORTED_PORTS] {
        // No interrupts set, return immediately because there is nothing to wait for
        // Also log a warning because this likely isn't what the user intended
        if mask == [IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS] {
            warn!("Interrupt masks are empty, returning immediately");
            return [IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS];
        }

        if clear_current {
            self.controller.interrupt_waker.reset();
        }

        let mut accumulated_flags = AccumulatedFlagsAny::new(self.controller, mask);
        loop {
            let flags = self.controller.interrupt_waker.wait().await;
            if let Some(flags) = accumulated_flags.accumulate(flags) {
                return flags;
            }
        }
    }
}

#[cfg(test)]
mod test {
    use embassy_sync::blocking_mutex::raw::NoopRawMutex;
    use embassy_time::{with_timeout, Duration, TimeoutError};
    use embedded_hal_mock::eh1::i2c::Mock;
    use static_cell::StaticCell;

    use super::*;
    use crate::asynchronous::embassy::controller::Controller;
    use crate::ADDR0;

    /// Tests `wait_any_masked` with a mask for both ports.
    #[tokio::test]
    async fn test_wait_any_masked_both() {
        static CONTROLLER: StaticCell<Controller<NoopRawMutex, Mock>> = StaticCell::new();
        let controller = CONTROLLER.init(Controller::new_tps66994(Mock::new(&[]), Default::default(), ADDR0).unwrap());
        let (pd, _processor, mut receiver) = controller.make_parts();

        let mut port0 = IntEventBus1::new_zero();
        port0.set_new_consumer_contract(true);
        port0.set_sink_ready(true);
        port0.set_cmd_1_completed(true);

        let mut port1 = IntEventBus1::new_zero();
        port1.set_plug_event(true);
        port1.set_alert_message_received(true);

        pd.controller.interrupt_waker.signal([port0, port1]);

        let mut mask0 = IntEventBus1::new_zero();
        mask0.set_cmd_1_completed(true);

        let mut mask1 = IntEventBus1::new_zero();
        mask1.set_plug_event(true);
        mask1.set_alert_message_received(true);

        let flags = receiver.wait_any_masked(false, [mask0, mask1]).await;
        assert_eq!(flags, [mask0, mask1]);

        let mut unhandled0 = IntEventBus1::new_zero();
        unhandled0.set_new_consumer_contract(true);
        unhandled0.set_sink_ready(true);

        let unhandled1 = IntEventBus1::new_zero();

        // Should already be signaled
        assert_eq!(
            pd.controller.interrupt_waker.try_take().unwrap(),
            [unhandled0, unhandled1]
        );
    }

    /// Tests `wait_any_masked` with a mask for a single port.
    #[tokio::test]
    async fn test_wait_any_masked_single() {
        static CONTROLLER: StaticCell<Controller<NoopRawMutex, Mock>> = StaticCell::new();
        let controller = CONTROLLER.init(Controller::new_tps66994(Mock::new(&[]), Default::default(), ADDR0).unwrap());
        let (pd, _processor, mut receiver) = controller.make_parts();

        let mut port0 = IntEventBus1::new_zero();
        port0.set_new_consumer_contract(true);
        port0.set_sink_ready(true);
        port0.set_cmd_1_completed(true);

        let mut port1 = IntEventBus1::new_zero();
        port1.set_plug_event(true);
        port1.set_alert_message_received(true);

        pd.controller.interrupt_waker.signal([port0, port1]);

        let mut mask0 = IntEventBus1::new_zero();
        mask0.set_cmd_1_completed(true);

        let mask1 = IntEventBus1::new_zero();

        let flags = receiver.wait_any_masked(false, [mask0, mask1]).await;
        assert_eq!(flags, [mask0, mask1]);

        let mut unhandled0 = IntEventBus1::new_zero();
        unhandled0.set_new_consumer_contract(true);
        unhandled0.set_sink_ready(true);

        let unhandled1 = port1;

        // Should already be signaled
        assert_eq!(
            pd.controller.interrupt_waker.try_take().unwrap(),
            [unhandled0, unhandled1]
        );
    }

    /// Tests `wait_any_masked` with both masks set to zero.
    #[tokio::test]
    async fn test_wait_any_masked_zero_masks() {
        static CONTROLLER: StaticCell<Controller<NoopRawMutex, Mock>> = StaticCell::new();
        let controller = CONTROLLER.init(Controller::new_tps66994(Mock::new(&[]), Default::default(), ADDR0).unwrap());
        let (pd, _processor, mut receiver) = controller.make_parts();

        let mut port0 = IntEventBus1::new_zero();
        port0.set_new_consumer_contract(true);
        port0.set_sink_ready(true);
        port0.set_cmd_1_completed(true);

        let mut port1 = IntEventBus1::new_zero();
        port1.set_plug_event(true);
        port1.set_alert_message_received(true);

        pd.controller.interrupt_waker.signal([port0, port1]);

        let mask0 = IntEventBus1::new_zero();
        let mask1 = IntEventBus1::new_zero();
        let flags = receiver.wait_any_masked(false, [mask0, mask1]).await;
        assert_eq!(flags, [mask0, mask1]);

        // Should already be signaled with nothing changed
        assert_eq!(pd.controller.interrupt_waker.try_take().unwrap(), [port0, port1]);
    }

    #[tokio::test]
    async fn test_wait_any_masked_timeout() {
        // Port0 mocked pending interrupts
        let mut port0 = IntEventBus1::new_zero();
        port0.set_new_consumer_contract(true);

        // Port1 mocked pending interrupts
        let mut port1 = IntEventBus1::new_zero();
        port1.set_plug_event(true);

        static CONTROLLER: StaticCell<Controller<NoopRawMutex, Mock>> = StaticCell::new();
        let controller = CONTROLLER.init(Controller::new_tps66994(Mock::new(&[]), Default::default(), ADDR0).unwrap());
        let (pd, _processor, mut receiver) = controller.make_parts();

        pd.controller.interrupt_waker.signal([port0, port1]);

        // The mask doesn't match the pending interrupts, so we should get a timeout
        let mut mask0 = IntEventBus1::new_zero();
        mask0.set_cmd_1_completed(true);

        let mut mask1 = IntEventBus1::new_zero();
        mask1.set_new_provider_contract(true);

        assert_eq!(
            with_timeout(
                Duration::from_millis(10),
                receiver.wait_any_masked(false, [mask0, mask1])
            )
            .await,
            Err(TimeoutError)
        );

        // Use all mask to get leftover interrupts
        let mut leftover0 = IntEventBus1::new_zero();
        leftover0.set_new_consumer_contract(true);

        let mut leftover1 = IntEventBus1::new_zero();
        leftover1.set_plug_event(true);

        let leftover_flags = with_timeout(
            Duration::from_millis(10),
            receiver.wait_any_masked(false, [IntEventBus1::all(), IntEventBus1::all()]),
        )
        .await
        .unwrap();
        assert_eq!(leftover_flags[0], leftover0);
        assert_eq!(leftover_flags[1], leftover1);
    }

    /// Tests `wait_any`.
    #[tokio::test]
    async fn test_wait_any() {
        static CONTROLLER: StaticCell<Controller<NoopRawMutex, Mock>> = StaticCell::new();
        let controller = CONTROLLER.init(Controller::new_tps66994(Mock::new(&[]), Default::default(), ADDR0).unwrap());
        let (pd, _processor, mut receiver) = controller.make_parts();

        let mut port0 = IntEventBus1::new_zero();
        port0.set_new_consumer_contract(true);
        port0.set_sink_ready(true);
        port0.set_cmd_1_completed(true);

        let mut port1 = IntEventBus1::new_zero();
        port1.set_plug_event(true);
        port1.set_alert_message_received(true);

        pd.controller.interrupt_waker.signal([port0, port1]);

        let mut flags0 = IntEventBus1::new_zero();
        flags0.set_new_consumer_contract(true);
        flags0.set_sink_ready(true);
        flags0.set_cmd_1_completed(true);

        let mut flags1 = IntEventBus1::new_zero();
        flags1.set_plug_event(true);
        flags1.set_alert_message_received(true);

        let flags = receiver.wait_any(false).await;
        assert_eq!(flags, [flags0, flags1]);

        // This should timeout because there are no leftover interrupts
        let leftover_flags = with_timeout(
            Duration::from_millis(10),
            receiver.wait_any_masked(false, [IntEventBus1::all(), IntEventBus1::all()]),
        )
        .await;
        assert_eq!(leftover_flags, Err(TimeoutError));
    }
}
