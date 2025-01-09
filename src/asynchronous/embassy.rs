//! High-level API that uses embassy_sync wakers
// This code holds refcells across await points but this is controlled within the code using scope
#![allow(clippy::await_holding_refcell_ref)]
use core::cell::{RefCell, RefMut};
use core::future::poll_fn;

use embassy_sync::waitqueue::AtomicWaker;
use embedded_hal_async::delay::DelayNs;
use embedded_hal_async::digital::Wait;
use embedded_hal_async::i2c::I2c;
use embedded_usb_pd::asynchronous::controller::PdController;
use embedded_usb_pd::{Error, PortId};

use crate::asynchronous::internal;
use crate::command::{Command, ReturnValue};
use crate::registers::field_sets::IntEventBus1;

pub struct Controller<B: I2c, INT: Wait> {
    inner: RefCell<internal::Tps6699x<B>>,
    int: RefCell<INT>,
    command_waker: AtomicWaker,
}

impl<B: I2c, INT: Wait> Controller<B, INT> {
    pub fn new(bus: B, int: INT, addr: [u8; 2]) -> Result<Self, Error<B::Error>> {
        Ok(Self {
            inner: RefCell::new(internal::Tps6699x::new(bus, addr)),
            int: RefCell::new(int),
            command_waker: AtomicWaker::new(),
        })
    }

    pub fn make_parts(&mut self) -> (Tps6699x<'_, B, INT>, Interrupt<'_, B, INT>) {
        let tps = Tps6699x { controller: self };
        let interrupt = Interrupt { controller: self };
        (tps, interrupt)
    }
}

pub struct Tps6699x<'a, B: I2c, INT: Wait> {
    controller: &'a Controller<B, INT>,
}

impl<'a, B: I2c, INT: Wait> Tps6699x<'a, B, INT> {
    async fn wait_command_complete(&mut self) {
        poll_fn(|cx| {
            self.controller.command_waker.register(cx.waker());
            core::task::Poll::<()>::Pending
        })
        .await;
    }

    pub fn borrow_inner(&mut self) -> RefMut<'_, internal::Tps6699x<B>> {
        self.controller.inner.borrow_mut()
    }

    /// Execute the given command
    pub async fn execute_command(
        &mut self,
        port: PortId,
        cmd: Command,
        outdata: Option<&mut [u8]>,
    ) -> Result<ReturnValue, Error<B::Error>> {
        {
            let mut inner = self.borrow_inner();
            inner.send_command(port, cmd).await?;
        }

        self.wait_command_complete().await;
        {
            let mut inner = self.borrow_inner();
            inner.read_command_result(port, outdata).await
        }
    }
}

impl<'a, B: I2c, INT: Wait> PdController<B::Error> for Tps6699x<'a, B, INT> {
    async fn reset(&mut self, delay: &mut impl DelayNs) -> Result<(), Error<B::Error>> {
        self.borrow_inner().reset(delay).await
    }
}

pub struct Interrupt<'a, B: I2c, INT: Wait> {
    controller: &'a Controller<B, INT>,
}

impl<'a, B: I2c, INT: Wait> Interrupt<'a, B, INT> {
    fn borrow_inner(&mut self) -> RefMut<'_, internal::Tps6699x<B>> {
        self.controller.inner.borrow_mut()
    }

    async fn check_interrupt(&mut self, port: PortId) -> Result<IntEventBus1, Error<B::Error>> {
        let flags = {
            let mut inner = self.borrow_inner();
            inner.clear_interrupt(port).await?
        };

        if flags.cmd_1_completed() {
            // Each port can execute commands, but this implementation is single threaded
            // so only one command can be in progress at a time
            self.controller.command_waker.wake();
        }

        Ok(flags)
    }

    pub async fn process_interrupt(&mut self) -> Result<(IntEventBus1, IntEventBus1), Error<B::Error>> {
        {
            let mut int = self.controller.int.borrow_mut();
            int.wait_for_low().await.unwrap();
        }

        let p0_flags = self.check_interrupt(PortId(0)).await?;
        let p1_flags = self.check_interrupt(PortId(1)).await?;

        Ok((p0_flags, p1_flags))
    }
}
