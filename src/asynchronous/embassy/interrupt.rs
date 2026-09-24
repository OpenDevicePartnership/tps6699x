//! Interrupt related code.
use core::array::from_fn;

use embassy_sync::blocking_mutex::raw::RawMutex;
use embassy_time::{Duration, with_timeout};
use embedded_hal::digital::InputPin;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::{Error, LocalPortId, PdError};
use itertools::izip;

use crate::asynchronous::embassy::controller::Controller;
use crate::registers::field_sets::IntEventBus1;
use crate::{MAX_SUPPORTED_PORTS, error, trace, warn};

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
        let i2c_timeout = self.controller.config.interrupt_processor_config.interrupt_timeout;
        let mut flags = [IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS];

        let interrupts_enabled = self.controller.interrupts_enabled();
        let mut inner = self.controller.inner.lock().await;

        // Note: `interrupts_enabled` and `flags` are both of size MAX_SUPPORTED_PORTS and so
        // will always have a 1:1 mapping. If `num_ports` ever returns a value larger than
        // MAX_SUPPORTED_PORTS, `port` will simply be capped at MAX_SUPPORTED_PORTS.
        for (port, (interrupt_enabled, flag, command_complete)) in izip!(
            interrupts_enabled.iter(),
            flags.iter_mut(),
            self.controller.per_port.iter().map(|p| &p.command_complete)
        )
        .take(self.controller.num_ports)
        .enumerate()
        {
            let port_id = LocalPortId(port as u8);

            if !interrupt_enabled {
                trace!("{:?}: Interrupts disabled", port_id);
                continue;
            }

            let interrupt_asserted = match int.is_high() {
                Ok(true) => {
                    trace!("Interrupt line is high");
                    false
                }
                Err(_) => {
                    error!("Failed to read interrupt line");
                    return PdError::Failed.into();
                }
                _ => true,
            };

            if interrupt_asserted {
                match with_timeout(i2c_timeout, inner.read_interrupt(port_id)).await {
                    Ok(Ok(event)) => {
                        *flag |= event;
                        if event.cmd_1_completed() {
                            command_complete.signal(());
                        }
                        // Publish before the destructive W1C write. No await may be
                        // introduced between this publication and the clear below.
                        self.controller.publish_interrupt(port_id, event);
                    }
                    Ok(Err(_)) => {
                        error!("{:?}: read_interrupt failed", port_id);
                    }
                    Err(_) => {
                        warn!("{:?}: read_interrupt timeout", port_id);
                    }
                }
            }

            if !interrupt_asserted && !inner.has_pending_interrupt_clear(port_id)? {
                // Interrupt line is deasserted and there is nothing pending to clear on this port.
                continue;
            }

            match with_timeout(i2c_timeout, inner.clear_pending_interrupts(port_id)).await {
                Ok(res) => match res {
                    Ok(()) => {}
                    Err(_) => {
                        error!("{:?}: clear_pending_interrupts failed", port_id);
                        continue;
                    }
                },
                Err(_) => {
                    error!("{:?}: clear_pending_interrupts timeout", port_id);
                    continue;
                }
            }
        }

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
            let handled = from_fn(
                #[allow(clippy::indexing_slicing)]
                |i| self.accumulated_flags[i] & self.masks[i],
            );
            // Put unhandled flags back for signaling in `drop()`
            self.accumulated_flags = from_fn(
                #[allow(clippy::indexing_slicing)]
                |i| self.accumulated_flags[i] & !self.masks[i],
            );
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
        let unhandled = from_fn(
            #[allow(clippy::indexing_slicing)]
            |i| self.accumulated_flags[i] | new[i],
        );

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
    use core::convert::Infallible;
    use core::future::pending;

    use embassy_sync::blocking_mutex::raw::NoopRawMutex;
    use embassy_time::{Duration, TimeoutError, with_timeout};
    use embedded_hal::digital::{ErrorType as DigitalErrorType, InputPin};
    use embedded_hal::i2c::{ErrorKind, ErrorType as I2cErrorType, Operation};
    use embedded_hal_async::i2c::I2c as AsyncI2c;
    use embedded_hal_mock::eh1::i2c::Mock;
    use static_cell::StaticCell;

    use super::*;
    use crate::asynchronous::embassy::controller::Controller;
    use crate::test::PORT0_ADDR0;
    use crate::{ADDR0, PORT0};

    const INTERRUPT_BYTES: usize = 11;

    #[derive(Clone, Copy)]
    enum ClearBehavior {
        Succeed,
        FailOnce,
        PendingOnce,
    }

    struct InterruptTestBus {
        event: [u8; INTERRUPT_BYTES],
        clear_behavior: ClearBehavior,
        read_count: usize,
        clear_attempts: usize,
    }

    impl InterruptTestBus {
        fn new(event: IntEventBus1, clear_behavior: ClearBehavior) -> Self {
            Self {
                event: event.into(),
                clear_behavior,
                read_count: 0,
                clear_attempts: 0,
            }
        }
    }

    impl I2cErrorType for InterruptTestBus {
        type Error = ErrorKind;
    }

    impl AsyncI2c for InterruptTestBus {
        async fn read(&mut self, _address: u8, _read: &mut [u8]) -> Result<(), Self::Error> {
            Err(ErrorKind::Other)
        }

        async fn write(&mut self, _address: u8, write: &[u8]) -> Result<(), Self::Error> {
            if write.first() != Some(&0x18) {
                return Err(ErrorKind::Other);
            }

            self.clear_attempts += 1;
            match self.clear_behavior {
                ClearBehavior::Succeed => Ok(()),
                ClearBehavior::FailOnce => {
                    self.clear_behavior = ClearBehavior::Succeed;
                    Err(ErrorKind::Other)
                }
                ClearBehavior::PendingOnce => {
                    self.clear_behavior = ClearBehavior::Succeed;
                    pending().await
                }
            }
        }

        async fn write_read(&mut self, _address: u8, write: &[u8], read: &mut [u8]) -> Result<(), Self::Error> {
            if write != [0x14] {
                return Err(ErrorKind::Other);
            }

            let (length, data) = read.split_first_mut().ok_or(ErrorKind::Other)?;
            if data.len() != self.event.len() {
                return Err(ErrorKind::Other);
            }

            *length = self.event.len() as u8;
            data.copy_from_slice(&self.event);
            self.read_count += 1;
            Ok(())
        }

        async fn transaction(&mut self, address: u8, operations: &mut [Operation<'_>]) -> Result<(), Self::Error> {
            for operation in operations {
                match operation {
                    Operation::Read(read) => self.read(address, read).await?,
                    Operation::Write(write) => self.write(address, write).await?,
                }
            }
            Ok(())
        }
    }

    struct LowPin;

    impl DigitalErrorType for LowPin {
        type Error = Infallible;
    }

    impl InputPin for LowPin {
        fn is_high(&mut self) -> Result<bool, Self::Error> {
            Ok(false)
        }

        fn is_low(&mut self) -> Result<bool, Self::Error> {
            Ok(true)
        }
    }

    fn interrupt_event() -> IntEventBus1 {
        let mut event = IntEventBus1::new_zero();
        event.set_plug_event(true);
        event.set_new_consumer_contract(true);
        event.set_cmd_1_completed(true);
        event
    }

    #[test]
    fn test_interrupt_publish_invalid_port_preserves_pending_flags() {
        let mut controller: Controller<NoopRawMutex, _> =
            Controller::new_tps66994(Mock::new(&[]), Default::default(), ADDR0).unwrap();
        let pending = [interrupt_event(), IntEventBus1::new_zero()];
        controller.interrupt_waker.signal(pending);

        controller.publish_interrupt(LocalPortId(MAX_SUPPORTED_PORTS as u8), interrupt_event());

        assert_eq!(controller.interrupt_waker.try_take(), Some(pending));
        controller.inner.get_mut().bus.done();
    }

    #[tokio::test]
    async fn test_interrupt_is_published_before_cancelled_clear() {
        let event = interrupt_event();
        let bus = InterruptTestBus::new(event, ClearBehavior::PendingOnce);
        let mut controller: Controller<NoopRawMutex, _> =
            Controller::new_tps66993(bus, Default::default(), PORT0_ADDR0).unwrap();
        let (_pd, mut interrupt, _receiver) = controller.make_parts();
        let mut int = LowPin;

        assert_eq!(
            with_timeout(Duration::from_millis(10), interrupt.process_interrupt(&mut int)).await,
            Err(TimeoutError)
        );

        let pending = [event, IntEventBus1::new_zero()];
        assert_eq!(interrupt.controller.interrupt_waker.try_take(), Some(pending));
        interrupt.controller.interrupt_waker.signal(pending);
        {
            let inner = interrupt.controller.inner.lock().await;
            assert!(inner.has_pending_interrupt_clear(PORT0).unwrap());
            assert_eq!(inner.bus.read_count, 1);
            assert_eq!(inner.bus.clear_attempts, 1);
        }

        assert_eq!(
            interrupt.process_interrupt(&mut int).await.unwrap(),
            [IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS]
        );
        assert_eq!(interrupt.controller.interrupt_waker.try_take(), Some(pending));
        let inner = interrupt.controller.inner.lock().await;
        assert!(!inner.has_pending_interrupt_clear(PORT0).unwrap());
        assert_eq!(inner.bus.read_count, 2);
        assert_eq!(inner.bus.clear_attempts, 2);
    }

    #[tokio::test]
    async fn test_interrupt_clear_failure_is_retryable_without_duplicate_publication() {
        let event = interrupt_event();
        let bus = InterruptTestBus::new(event, ClearBehavior::FailOnce);
        let mut controller: Controller<NoopRawMutex, _> =
            Controller::new_tps66993(bus, Default::default(), PORT0_ADDR0).unwrap();
        let (_pd, mut interrupt, _receiver) = controller.make_parts();
        let mut int = LowPin;

        assert_eq!(
            interrupt.process_interrupt(&mut int).await.unwrap(),
            [event, IntEventBus1::new_zero()]
        );
        {
            let inner = interrupt.controller.inner.lock().await;
            assert!(inner.has_pending_interrupt_clear(PORT0).unwrap());
        }

        assert_eq!(
            interrupt.process_interrupt(&mut int).await.unwrap(),
            [IntEventBus1::new_zero(); MAX_SUPPORTED_PORTS]
        );
        assert_eq!(
            interrupt.controller.interrupt_waker.try_take(),
            Some([event, IntEventBus1::new_zero()])
        );
        let inner = interrupt.controller.inner.lock().await;
        assert!(!inner.has_pending_interrupt_clear(PORT0).unwrap());
        assert_eq!(inner.bus.read_count, 2);
        assert_eq!(inner.bus.clear_attempts, 2);
    }

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
