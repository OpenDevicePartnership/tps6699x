//! `MuxR`: Repeat transactions on I2C3m under certain conditions.

use bitfield::bitfield;

bitfield! {
    /// The input data for the `MuxR` command.
    ///
    /// If all bits are zero when the command is executed, then the PD controller clears its history of the last events
    /// for target addresses 1-8. If `en_retry_on_target_addr_x` is asserted and no transaction has occurred on that
    /// I2C address because the history was cleared, then the PD controller will not execute any I2C transaction on
    /// that target address as there is no event to repeat.
    ///
    /// The contents of the last transaction on a given I2C target address is defined through the App Config loaded as
    /// part of the patch bundle. The `BINARYDATA_INDICES` register (`0x62`) contains the various events that can occur
    /// on each target address along with the payload for the transaction associated with each. When the `MuxR` task is
    /// executed, then for each `en_retry_on_target_addr*` that is asserted, it will be as if the last event for that
    /// target address has occurred again.
    #[derive(Clone, Copy)]
    #[cfg_attr(feature = "defmt", derive(defmt::Format))]
    pub struct Input(u16);
    impl Debug;

    /// If this bit is asserted, the PD controller will retry the last transaction on the I2C3m port that used
    /// `I2C_CNTLR_CONFIG.TargetAddr1` as the target address.
    pub bool, en_retry_on_target_addr_1, set_en_retry_on_target_addr_1: 0;

    /// If this bit is asserted, the PD controller will retry the last transaction on the I2C3m port that used
    /// `I2C_CNTLR_CONFIG.TargetAddr2` as the target address.
    pub bool, en_retry_on_target_addr_2, set_en_retry_on_target_addr_2: 1;

    /// If this bit is asserted, the PD controller will retry the last transaction on the I2C3m port that used
    /// `I2C_CNTLR_CONFIG.TargetAddr3` as the target address.
    pub bool, en_retry_on_target_addr_3, set_en_retry_on_target_addr_3: 2;

    /// If this bit is asserted, the PD controller will retry the last transaction on the I2C3m port that used
    /// `I2C_CNTLR_CONFIG.TargetAddr4` as the target address.
    pub bool, en_retry_on_target_addr_4, set_en_retry_on_target_addr_4: 3;

    /// If this bit is asserted, the PD controller will retry the last transaction on the I2C3m port that used
    /// `I2C_CNTLR_CONFIG.TargetAddr5` as the target address.
    pub bool, en_retry_on_target_addr_5, set_en_retry_on_target_addr_5: 4;

    /// If this bit is asserted, the PD controller will retry the last transaction on the I2C3m port that used
    /// `I2C_CNTLR_CONFIG.TargetAddr6` as the target address.
    pub bool, en_retry_on_target_addr_6, set_en_retry_on_target_addr_6: 5;

    /// If this bit is asserted, the PD controller will retry the last transaction on the I2C3m port that used
    /// `I2C_CNTLR_CONFIG.TargetAddr7` as the target address.
    pub bool, en_retry_on_target_addr_7, set_en_retry_on_target_addr_7: 6;

    /// If this bit is asserted, the PD controller will retry the last transaction on the I2C3m port that used
    /// `I2C_CNTLR_CONFIG.TargetAddr8` as the target address.
    pub bool, en_retry_on_target_addr_8, set_en_retry_on_target_addr_8: 7;

    /// If this bit is asserted, the PD controller will use I2C3m to write the `DATA_STATUS` register with
    /// `I2C_CNTLR_CONFIG.TargetAddrTbt1` as the target address. It will also repeat the write using
    /// `I2C_CNTLR_CONFIG.TargetAddrTbt2` as the target address.
    pub bool, en_retry_on_target_addr_tbt, set_en_retry_on_target_addr_tbt: 8;
}
