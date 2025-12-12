use core::future::Future;

use embedded_usb_pd::{Error, LocalPortId, PdError};

use crate::MAX_SUPPORTED_PORTS;

/// Trait for any type that restores the original interrupt state when dropped
#[allow(drop_bounds)]
pub trait InterruptGuard: Drop {}

/// Trait for any type that can enable/disable interrupts
pub trait InterruptController {
    type Guard: InterruptGuard;
    type BusError;

    fn interrupts_enabled(&self) -> impl Future<Output = Result<[bool; MAX_SUPPORTED_PORTS], Error<Self::BusError>>>;

    fn enable_interrupts_guarded(
        &mut self,
        enabled: [bool; MAX_SUPPORTED_PORTS],
    ) -> impl Future<Output = Result<Self::Guard, Error<Self::BusError>>>;

    /// Set the interrupt state for the given port for the lifetime of the returned guard
    #[allow(async_fn_in_trait)]
    async fn enable_interrupt_guarded(
        &mut self,
        port: LocalPortId,
        enabled: bool,
    ) -> Result<Self::Guard, Error<Self::BusError>> {
        let mut state = self.interrupts_enabled().await?;
        *state.get_mut(port.0 as usize).ok_or(PdError::InvalidPort)? = enabled;
        self.enable_interrupts_guarded(state).await
    }

    /// Disable all interrupts for the lifetime of the returned guard
    fn disable_all_interrupts_guarded(&mut self) -> impl Future<Output = Result<Self::Guard, Error<Self::BusError>>> {
        self.enable_interrupts_guarded([false; MAX_SUPPORTED_PORTS])
    }
}
