use embassy_sync::blocking_mutex::raw::RawMutex;
use embedded_hal::digital::InputPin;
use embedded_hal_async::digital::Wait;
use embedded_hal_async::i2c::I2c;

use super::Interrupt;
use crate::{error, warn};

/// Period to wait if we might be checking interrupts exessively
const INTERRUPT_BACKOFF_MS: u64 = 100;

/// Task to process all given interrupts
pub async fn interrupt_task<M: RawMutex, B: I2c, INT: Wait + InputPin>(
    int: &mut INT,
    interrupts: &mut [&mut Interrupt<'_, M, B>],
) {
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

        // If the interrupt line is still asserted then we either had an error or interrupts are currently disabled
        // Back off for a bit to allow other tasks to run, otherwise this task can end up continously scheduled
        // and starve other tasks
        match int.is_low() {
            Ok(true) => embassy_time::Timer::after_millis(INTERRUPT_BACKOFF_MS).await,
            Err(_) => {
                error!("Error post-checking interrupt line");
            }
            _ => {}
        }
    }
}
