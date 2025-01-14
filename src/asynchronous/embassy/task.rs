use defmt::error;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embedded_hal::digital::InputPin;
use embedded_hal_async::{digital::Wait, i2c::I2c};

use super::Interrupt;

pub async fn interrupt_task<const N: usize, M: RawMutex, B: I2c, INT: Wait + InputPin>(
    int: &mut INT,
    mut interrupts: [&mut Interrupt<'_, M, B>; N],
) {
    loop {
        if let Err(_) = int.wait_for_low().await {
            error!("Error waiting for interrupt");
            continue;
        }

        for interrupt in &mut interrupts {
            if let Err(_) = interrupt.process_interrupt(int).await {
                error!("Error processing interrupt");
            }
        }
    }
}
