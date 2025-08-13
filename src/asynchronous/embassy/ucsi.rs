//! UCSI related functionality
use bincode::decode_from_slice_with_context;

use crate::registers::REG_DATA1_LEN;

use super::*;
use bincode::encode_into_slice;
use embedded_usb_pd::{
    ucsi::{lpm, CommandType},
    GlobalPortId,
};

impl<'a, M: RawMutex, B: I2c> Tps6699x<'a, M, B> {
    pub async fn execute_ucsi_command(&mut self, command: &lpm::Command) -> Result<lpm::Response, Error<B::Error>> {
        let mut indata = [0u8; REG_DATA1_LEN];
        //
        let mut outdata = [0u8; REG_DATA1_LEN - 1];
        let port = command.port;

        // Internally the controller uses 1-based port numbering
        let mut command = command.clone();
        command.port = GlobalPortId(command.port.0 + 1);

        encode_into_slice(
            command,
            &mut indata,
            bincode::config::standard().with_fixed_int_encoding(),
        )
        .map_err(|_| Error::Pd(PdError::Serialize))?;

        // TODO: embedded-usb-pd types to distinguish local vs global port id
        trace!("Encoded UCSI command: {:?}", indata);
        let ret = self
            .execute_command(PortId(port.0), Command::Ucsi, Some(&indata), Some(&mut outdata))
            .await?;

        if ret != ReturnValue::Success {
            error!("UCSI command failed with return value: {:?}", ret);
            return Err(PdError::Failed.into());
        }

        trace!("UCSI command response: {:?}", outdata);
        let (response_data, _) = decode_from_slice_with_context(
            &outdata,
            bincode::config::standard().with_fixed_int_encoding(),
            CommandType::GetConnectorStatus,
        )
        .map_err(|_| Error::Pd(PdError::Serialize))?;
        Ok(response_data)
    }
}
