use bitfield::bitfield;
use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_time::Delay;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::PdError;
use fw_update_interface::basic::{Error as BasicFwUpdateError, FwUpdate as BasicFwUpdate};

use crate::asynchronous::embassy as tps6699x_drv;
use crate::asynchronous::fw_update::{disable_all_interrupts, enable_port0_interrupts, BorrowedUpdater, UpdateTarget};
use crate::odp::driver::{FwUpdateState, Tps6699x};
use crate::{error, warn, MAX_SUPPORTED_PORTS};

/// Converts a PD error into a basic FW update error
pub fn basic_fw_update_error_from_pd_error(pd_error: PdError) -> BasicFwUpdateError {
    match pd_error {
        PdError::Busy => BasicFwUpdateError::Busy,
        _ => BasicFwUpdateError::Failed,
    }
}

bitfield! {
    /// Custom customer use format
    //#[derive(Clone, Copy)]
    //#[cfg_attr(feature = "defmt", derive(defmt::Format))]
    struct CustomerUse(u64);
    impl Debug;
    /// Custom FW version
    pub u32, custom_fw_version, set_custom_fw_version: 31, 0;
    /// TI FW version
    pub u32, ti_fw_version, set_ti_fw_version: 63, 32;
}

impl<'a, M: RawMutex, B: I2c> BasicFwUpdate for Tps6699x<'a, M, B> {
    async fn get_active_fw_version(&mut self) -> Result<u32, BasicFwUpdateError> {
        let customer_use = CustomerUse(
            self.tps6699x
                .get_customer_use()
                .await
                .map_err(|e| basic_fw_update_error_from_pd_error(self.log_error(e)))?,
        );
        Ok(customer_use.custom_fw_version())
    }

    async fn start_fw_update(&mut self) -> Result<(), BasicFwUpdateError> {
        let mut delay = Delay;
        let mut updater: BorrowedUpdater<tps6699x_drv::Tps6699x<'_, M, B>> =
            BorrowedUpdater::with_config(self.fw_update_config.clone());

        // Abandon any previous in-progress update
        if let Some(update) = self.update_state.take() {
            warn!("Abandoning in-progress update");
            update
                .updater
                .abort_fw_update(&mut [&mut self.tps6699x], &mut delay)
                .await;
        }

        let mut guards = [const { None }; MAX_SUPPORTED_PORTS];
        // Disable all interrupts on both ports, use guards[1] to ensure that this set of guards is dropped last
        disable_all_interrupts::<tps6699x_drv::Tps6699x<'_, M, B>>(&mut [&mut self.tps6699x], &mut guards[1..])
            .await
            .map_err(|e| basic_fw_update_error_from_pd_error(self.log_error(e)))?;
        let in_progress = updater
            .start_fw_update(&mut [&mut self.tps6699x], &mut delay)
            .await
            .map_err(|e| basic_fw_update_error_from_pd_error(self.log_error(e)))?;

        // Re-enable interrupts on port 0 only
        if let Err(e) =
            enable_port0_interrupts::<tps6699x_drv::Tps6699x<'_, M, B>>(&mut [&mut self.tps6699x], &mut guards[0..1])
                .await
                .map_err(|e| basic_fw_update_error_from_pd_error(self.log_error(e)))
        {
            error!("Failed to enable port 0 interrupts, aborting update: {:#?}", e);
            in_progress.abort_fw_update(&mut [&mut self.tps6699x], &mut delay).await;
            return Err(e);
        }

        self.update_state = Some(FwUpdateState {
            updater: in_progress,
            guards,
        });
        Ok(())
    }

    /// Aborts the firmware update in progress
    ///
    /// This can reset the controller
    async fn abort_fw_update(&mut self) -> Result<(), BasicFwUpdateError> {
        // Check if we're still in firmware update mode
        if self
            .tps6699x
            .get_mode()
            .await
            .map_err(|e| basic_fw_update_error_from_pd_error(self.log_error(e)))?
            == crate::Mode::F211
        {
            let mut delay = Delay;

            if let Some(update) = self.update_state.take() {
                // Attempt to abort the firmware update by consuming our update object
                update
                    .updater
                    .abort_fw_update(&mut [&mut self.tps6699x], &mut delay)
                    .await;
                Ok(())
            } else {
                // Bypass our update object since we've gotten into a state where we don't have one
                self.tps6699x
                    .fw_update_mode_exit(&mut delay)
                    .await
                    .map_err(|e| basic_fw_update_error_from_pd_error(self.log_error(e)))
            }
        } else {
            // Not in FW update mode, don't need to do anything
            Ok(())
        }
    }

    /// Finalize the firmware update
    ///
    /// This will reset the controller
    async fn finalize_fw_update(&mut self) -> Result<(), BasicFwUpdateError> {
        if let Some(update) = self.update_state.take() {
            let mut delay = Delay;
            update
                .updater
                .complete_fw_update(&mut [&mut self.tps6699x], &mut delay)
                .await
                .map_err(|e| basic_fw_update_error_from_pd_error(self.log_error(e)))
        } else {
            Err(BasicFwUpdateError::NeedsActiveUpdate)
        }
    }

    async fn write_fw_contents(&mut self, _offset: usize, data: &[u8]) -> Result<(), BasicFwUpdateError> {
        if let Some(update) = &mut self.update_state {
            let mut delay = Delay;
            update
                .updater
                .write_bytes(&mut [&mut self.tps6699x], &mut delay, data)
                .await
                .map_err(|e| basic_fw_update_error_from_pd_error(self.log_error(e)))?;
            Ok(())
        } else {
            Err(BasicFwUpdateError::NeedsActiveUpdate)
        }
    }
}
