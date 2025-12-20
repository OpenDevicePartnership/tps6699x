//! UCSI related functionality
use bincode::{decode_from_slice_with_context, encode_into_slice};
use embedded_usb_pd::pdo::PDO_LEN;
use embedded_usb_pd::ucsi::lpm;
use embedded_usb_pd::PowerRole;

use super::*;
use crate::registers::REG_DATA1_LEN;

impl<'a, M: RawMutex, B: I2c> Tps6699x<'a, M, B> {
    async fn execute_get_alternate_modes(
        &mut self,
        port: LocalPortId,
        indata: &[u8],
        outdata: &mut [u8],
    ) -> Result<ReturnValue, Error<B::Error>> {
        // Controller UCSI system gets stuck if GET_ALT_MODES is sent to
        // recipient SOP when partner doesn't support an alt mode so workaround
        // is to first check for any alt mode before executing.
        let alt_mode = self.get_alt_mode_status(port).await?;
        if alt_mode != AltMode::none() {
            self.execute_command(port, Command::Ucsi, Some(indata), Some(outdata))
                .await
        } else {
            // No alt mode supported so return no data
            Ok(ReturnValue::Success)
        }
    }

    async fn execute_get_pdos(
        &mut self,
        port: LocalPortId,
        args: &lpm::get_pdos::Args,
        indata: &[u8],
        outdata: &mut [u8],
    ) -> Result<ReturnValue, Error<B::Error>> {
        if args.partner() {
            // Getting port partner PDOs can trigger a renegotiation so read the cached PDOs instead
            let caps: rx_caps::RxCaps<pdo::Pdo> = match args.role() {
                PowerRole::Source => self.get_rx_src_caps(port).await?.into(),
                PowerRole::Sink => self.get_rx_snk_caps(port).await?.into(),
            };

            let (raw_pdos, _) = outdata.as_chunks_mut::<PDO_LEN>();
            for (raw_pdo, pdo) in zip(
                raw_pdos,
                caps.spr_iter()
                    .chain(caps.epr_iter())
                    .skip(args.pdo_offset() as usize)
                    .take(args.num_pdos() as usize),
            ) {
                let encoded_pdo: u32 = (*pdo).into();
                *raw_pdo = encoded_pdo.to_le_bytes();
            }
        } else {
            self.execute_command(port, Command::Ucsi, Some(indata), Some(outdata))
                .await?;

            // First byte returned is number of bytes written, we don't use it so erase it
            outdata.rotate_left(1);
            if let Some(b) = outdata.last_mut() {
                *b = 0;
            }
        }

        Ok(ReturnValue::Success)
    }

    pub async fn execute_ucsi_command(
        &mut self,
        command: &lpm::LocalCommand,
    ) -> Result<Option<lpm::ResponseData>, Error<B::Error>> {
        let mut indata = [0u8; REG_DATA1_LEN];
        // -1 because the first byte is used for the standard TPS6699x command return value
        let mut outdata = [0u8; REG_DATA1_LEN - 1];
        let port = command.port();

        // Internally the controller uses 1-based port numbering
        let mut command = *command;
        command.set_port(LocalPortId(command.port().0 + 1));

        encode_into_slice(
            command,
            &mut indata,
            bincode::config::standard().with_fixed_int_encoding(),
        )
        .map_err(|_| Error::Pd(PdError::Serialize))?;

        let ret = match command.operation() {
            lpm::CommandData::GetAlternateModes(_) => {
                self.execute_get_alternate_modes(port, &indata, &mut outdata).await?
            }
            lpm::CommandData::GetPdos(args) => self.execute_get_pdos(port, &args, &indata, &mut outdata).await?,
            _rest => {
                self.execute_command(port, Command::Ucsi, Some(&indata), Some(&mut outdata))
                    .await?
            }
        };

        if ret != ReturnValue::Success {
            error!("UCSI command failed with return value: {:?}", ret);
            return Err(PdError::Failed.into());
        }

        trace!("UCSI command response: {:?}", outdata);
        if command.command_type().has_response() {
            let (response_data, _) = decode_from_slice_with_context(
                &outdata,
                bincode::config::standard().with_fixed_int_encoding(),
                command.command_type(),
            )
            .map_err(|_| Error::Pd(PdError::Serialize))?;
            Ok(Some(response_data))
        } else {
            Ok(None)
        }
    }
}
