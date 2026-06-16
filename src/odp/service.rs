use core::iter::zip;

use embassy_sync::blocking_mutex::raw::RawMutex;
use embedded_hal_async::i2c::I2c;
use type_c_interface::port::event::PortEventBitfield;

use crate::asynchronous::embassy::interrupt;
use crate::registers::field_sets::IntEventBus1;
use crate::{debug, MAX_SUPPORTED_PORTS};

impl<'a, M: RawMutex, BUS: I2c> type_c_service::controller::event_receiver::InterruptReceiver<MAX_SUPPORTED_PORTS>
    for interrupt::InterruptReceiver<'a, M, BUS>
{
    async fn wait_interrupt(&mut self) -> [PortEventBitfield; MAX_SUPPORTED_PORTS] {
        let interrupts = self.wait_any(false).await;
        let mut port_events = [PortEventBitfield::none(); MAX_SUPPORTED_PORTS];
        for (interrupt, event) in zip(interrupts.iter(), port_events.iter_mut()) {
            if *interrupt == IntEventBus1::new_zero() {
                continue;
            }

            if interrupt.plug_event() {
                debug!("Event: Plug event");
                event.status.set_plug_inserted_or_removed(true);
            }
            if interrupt.source_caps_received() {
                debug!("Event: Source Caps received");
                event.status.set_source_caps_received(true);
            }

            if interrupt.sink_ready() {
                debug!("Event: Sink ready");
                event.status.set_sink_ready(true);
            }

            if interrupt.new_consumer_contract() {
                debug!("Event: New contract as consumer, PD controller act as Sink");
                // Port is consumer and power negotiation is complete
                event.status.set_new_power_contract_as_consumer(true);
            }

            if interrupt.new_provider_contract() {
                debug!("Event: New contract as provider, PD controller act as source");
                // Port is provider and power negotiation is complete
                event.status.set_new_power_contract_as_provider(true);
            }

            if interrupt.power_swap_completed() {
                debug!("Event: power swap completed");
                event.status.set_power_swap_completed(true);
            }

            if interrupt.data_swap_completed() {
                debug!("Event: data swap completed");
                event.status.set_data_swap_completed(true);
            }

            if interrupt.am_entered() {
                debug!("Event: alt mode entered");
                event.status.set_alt_mode_entered(true);
            }

            if interrupt.hard_reset() {
                debug!("Event: hard reset");
                event.status.set_pd_hard_reset(true);
            }

            if interrupt.crossbar_error() {
                debug!("Event: crossbar error");
                event.notification.set_usb_mux_error_recovery(true);
            }

            if interrupt.usvid_mode_entered() {
                debug!("Event: user svid mode entered");
                event.notification.set_custom_mode_entered(true);
            }

            if interrupt.usvid_mode_exited() {
                debug!("Event: usvid mode exited");
                event.notification.set_custom_mode_exited(true);
            }

            if interrupt.usvid_attention_vdm_received() {
                debug!("Event: user svid attention vdm received");
                event.notification.set_custom_mode_attention_received(true);
            }

            if interrupt.usvid_other_vdm_received() {
                debug!("Event: user svid other vdm received");
                event.notification.set_custom_mode_other_vdm_received(true);
            }

            if interrupt.discover_mode_completed() {
                debug!("Event: discover mode completed");
                event.notification.set_discover_mode_completed(true);
            }

            if interrupt.dp_sid_status_updated() {
                debug!("Event: dp sid status updated");
                event.notification.set_dp_status_update(true);
            }

            if interrupt.alert_message_received() {
                debug!("Event: alert message received");
                event.notification.set_alert(true);
            }
        }
        port_events
    }
}
