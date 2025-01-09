//! High-level API that uses embassy_sync wakers
// This code holds refcells across await points but this is controlled within the code using scope
#![allow(clippy::await_holding_refcell_ref)]
use core::cell::RefCell;

use embassy_sync::blocking_mutex::raw::{NoopRawMutex, RawMutex};
use embassy_sync::mutex::{Mutex, MutexGuard};
use embassy_sync::signal::Signal;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::digital::Wait;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::asynchronous::controller::PdController;
use embedded_usb_pd::{Error, PortId};

use crate::asynchronous::internal;
use crate::command::{Command, ReturnValue};
use crate::registers::field_sets::IntEventBus1;

pub struct Controller<M: RawMutex, B: I2c, INT: Wait> {
    inner: Mutex<M, internal::Tps6699x<B>>,
    int: RefCell<INT>,
    interrupt_waker: Signal<NoopRawMutex, (IntEventBus1, IntEventBus1)>,
}

impl<M: RawMutex, B: I2c, INT: Wait> Controller<M, B, INT> {
    pub fn new(bus: B, int: INT, addr: [u8; 2]) -> Result<Self, Error<B::Error>> {
        Ok(Self {
            inner: Mutex::new(internal::Tps6699x::new(bus, addr)),
            int: RefCell::new(int),
            interrupt_waker: Signal::new(),
        })
    }

    pub fn make_parts(&mut self) -> (Tps6699x<'_, M, B, INT>, Interrupt<'_, M, B, INT>) {
        let tps = Tps6699x { controller: self };
        let interrupt = Interrupt { controller: self };
        (tps, interrupt)
    }
}

pub struct Tps6699x<'a, M: RawMutex, B: I2c, INT: Wait> {
    controller: &'a Controller<M, B, INT>,
}

impl<'a, M: RawMutex, B: I2c, INT: Wait> Tps6699x<'a, M, B, INT> {
    pub async fn lock_inner(&mut self) -> MutexGuard<'_, M, internal::Tps6699x<B>> {
        self.controller.inner.lock().await
    }

    /// Execute the given command
    pub async fn execute_command(
        &mut self,
        port: PortId,
        cmd: Command,
        outdata: Option<&mut [u8]>,
    ) -> Result<ReturnValue, Error<B::Error>> {
        {
            let mut inner = self.lock_inner().await;
            inner.send_command(port, cmd).await?;
        }

        self.wait_command_complete().await;
        {
            let mut inner = self.lock_inner().await;
            inner.read_command_result(port, outdata).await
        }
    }

    pub async fn wait_interrupt(&mut self) -> (IntEventBus1, IntEventBus1) {
        self.controller.interrupt_waker.wait().await
    }

    async fn wait_command_complete(&mut self) {
        loop {
            let (p0_flags, p1_flags) = self.wait_interrupt().await;

            if p0_flags.cmd_1_completed() || p1_flags.cmd_1_completed() {
                break;
            }
        }
    }
}

impl<'a, M: RawMutex, B: I2c, INT: Wait> PdController<B::Error> for Tps6699x<'a, M, B, INT> {
    async fn reset(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<B::Error>> {
        self.lock_inner().await.reset(delay).await
    }
}

pub struct Interrupt<'a, M: RawMutex, B: I2c, INT: Wait> {
    controller: &'a Controller<M, B, INT>,
}

impl<'a, M: RawMutex, B: I2c, INT: Wait> Interrupt<'a, M, B, INT> {
    async fn borrow_inner(&mut self) -> MutexGuard<'_, M, internal::Tps6699x<B>> {
        self.controller.inner.lock().await
    }

    pub async fn process_interrupt(&mut self) -> Result<(IntEventBus1, IntEventBus1), Error<B::Error>> {
        {
            let mut int = self.controller.int.borrow_mut();
            int.wait_for_low().await.unwrap();
        }

        let mut inner = self.borrow_inner().await;
        let p0_flags = inner.clear_interrupt(PortId(0)).await?;
        let p1_flags = inner.clear_interrupt(PortId(1)).await?;

        Ok((p0_flags, p1_flags))
    }
}
