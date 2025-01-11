#![no_std]
#![no_main]
#![allow(clippy::await_holding_refcell_ref)]
use core::default::Default;

use defmt::info;
use embassy_executor::Spawner;
use embassy_imxrt::gpio::{Input, Inverter, Pull};
use embassy_imxrt::i2c::master::{I2cMaster, Speed};
use embassy_imxrt::i2c::Async;
use embassy_imxrt::{self, bind_interrupts, peripherals};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embedded_usb_pd::asynchronous::controller::PdController;
use embedded_usb_pd::PortId;
use mimxrt600_fcb::FlexSPIFlashConfigurationBlock;
use static_cell::StaticCell;
use tps6699x::asynchronous::embassy;
use tps6699x::registers::field_sets::IntEventBus1;
use tps6699x::ADDR0;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    FLEXCOMM2 => embassy_imxrt::i2c::InterruptHandler<peripherals::FLEXCOMM2>;
});

type Bus<'a> = I2cMaster<'a, Async>;
type Int<'a> = Input<'a>;
type Controller<'a> = embassy::Controller<NoopRawMutex, Bus<'a>, Int<'a>>;

type Interrupt<'a> = embassy::Interrupt<'a, NoopRawMutex, Bus<'a>, Int<'a>>;
type Tps6699x<'a> = embassy::Tps6699x<'a, NoopRawMutex, Bus<'a>, Int<'a>>;

#[embassy_executor::task]
async fn interrupt_task(mut interrupt: Interrupt<'static>) {
    loop {
        let _ = interrupt.process_interrupt().await.unwrap();
    }
}

#[embassy_executor::task]
async fn pd_task(mut pd: Tps6699x<'static>) {
    let mut delay = embassy_time::Delay;

    info!("Reseting PD controller");
    pd.reset(&mut delay).await.unwrap();
    info!("PD controller reset complete");

    {
        let mut inner = pd.lock_inner().await;
        let mode = inner.get_mode().await.unwrap();
        info!("Mode: {}", mode);
    }

    loop {
        let (p0_flags, p1_flags) = pd.wait_interrupt().await;

        let (port, flags) = if p0_flags != IntEventBus1::new_zero() {
            (PortId(0), p0_flags)
        } else if p1_flags != IntEventBus1::new_zero() {
            (PortId(1), p1_flags)
        } else {
            continue;
        };

        info!("Got interrupt({}): {}", port, flags);

        let mut inner = pd.lock_inner().await;
        info!("Getting port status");
        let status = inner.get_port_status(port).await.unwrap();
        info!("Port status: {}", status);

        if !status.plug_present() {
            info!("Plug removed: {}", port.0);
            continue;
        }

        info!("Plug connected: {} ", port.0);

        let pdo = inner.get_active_pdo_contract(port).await.unwrap();
        info!("PDO: {}", pdo);

        let rdo = inner.get_active_rdo_contract(port).await.unwrap();
        info!("RDO: {}", rdo);
        info!("Done");
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_imxrt::init(Default::default());

    let int_in = Input::new(p.PIO1_0, Pull::Up, Inverter::Disabled);
    let device = I2cMaster::new_async(p.FLEXCOMM2, p.PIO0_18, p.PIO0_17, Irqs, Speed::Standard, p.DMA0_CH5).unwrap();

    static CONTROLLER: StaticCell<Controller<'static>> = StaticCell::new();
    let controller = CONTROLLER.init(Controller::new(device, int_in, ADDR0).unwrap());
    let (pd, interrupt) = controller.make_parts();

    spawner.must_spawn(interrupt_task(interrupt));
    spawner.must_spawn(pd_task(pd));
}

#[link_section = ".otfad"]
#[used]
static OTFAD: [u8; 256] = [0; 256];

#[link_section = ".fcb"]
#[used]
static FCB: FlexSPIFlashConfigurationBlock = FlexSPIFlashConfigurationBlock::build();

#[link_section = ".biv"]
#[used]
static BOOT_IMAGE_VERSION: u32 = 0x01000000;

#[link_section = ".keystore"]
#[used]
static KEYSTORE: [u8; 2048] = [0; 2048];
