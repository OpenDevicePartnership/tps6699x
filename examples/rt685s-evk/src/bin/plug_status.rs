#![no_std]
#![no_main]
use core::default::Default;

use defmt::info;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::Spawner;
use embassy_imxrt::gpio::{Input, Inverter, Pull};
use embassy_imxrt::i2c::master::I2cMaster;
use embassy_imxrt::i2c::Async;
use embassy_imxrt::{self, bind_interrupts, peripherals};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embedded_usb_pd::LocalPortId;
use mimxrt600_fcb::FlexSPIFlashConfigurationBlock;
use static_cell::StaticCell;
use tps6699x::asynchronous::embassy as pd_controller;
use tps6699x::registers::field_sets::IntEventBus1;
use tps6699x::{ADDR0, MAX_SUPPORTED_PORTS};
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    FLEXCOMM2 => embassy_imxrt::i2c::InterruptHandler<peripherals::FLEXCOMM2>;
});

type Bus<'a> = I2cDevice<'a, NoopRawMutex, I2cMaster<'a, Async>>;
type Controller<'a> = pd_controller::controller::Controller<NoopRawMutex, Bus<'a>>;

type Interrupt<'a> = pd_controller::Interrupt<'a, NoopRawMutex, Bus<'a>>;

#[embassy_executor::task]
async fn interrupt_task(mut int_in: Input<'static>, mut interrupt: Interrupt<'static>) {
    pd_controller::task::interrupt_task(&mut int_in, [&mut interrupt].as_mut_slice()).await;
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_imxrt::init(Default::default());

    let int_in = Input::new(p.PIO1_7, Pull::Up, Inverter::Disabled);
    static BUS: StaticCell<Mutex<NoopRawMutex, I2cMaster<'static, Async>>> = StaticCell::new();
    let bus = BUS.init(Mutex::new(
        I2cMaster::new_async(p.FLEXCOMM2, p.PIO0_18, p.PIO0_17, Irqs, Default::default(), p.DMA0_CH5).unwrap(),
    ));

    let device = I2cDevice::new(bus);

    static CONTROLLER: StaticCell<Controller<'static>> = StaticCell::new();
    let controller = CONTROLLER.init(Controller::new_tps66994(device, ADDR0).unwrap());
    let (mut pd, interrupt) = controller.make_parts();

    info!("Spawing PD interrupt task");
    spawner.must_spawn(interrupt_task(int_in, interrupt));

    loop {
        let mut plug_event_mask = IntEventBus1::new_zero();
        plug_event_mask.set_plug_event(true);
        let flags = pd
            .wait_interrupt_any(false, [plug_event_mask; MAX_SUPPORTED_PORTS])
            .await;

        for (i, flag) in flags.iter().enumerate().take(pd.num_ports()) {
            if *flag == IntEventBus1::new_zero() {
                continue;
            }

            let port = LocalPortId(i as u8);
            info!("{:?}: plug event", port);
            let result = pd.get_port_status(port).await;
            if let Err(e) = result {
                info!("{:?}: error getting port status: {:?}", port, e);
                continue;
            }

            let status = result.unwrap();
            if status.plug_present() {
                info!("{:?}: plug present", port);
                if status.plug_orientation() {
                    info!("{:?}: flipped ", port);
                } else {
                    info!("{:?}: not flipped", port);
                }

                info!("{:?}: connection state {:?}", port, status.connection_state());
            } else {
                info!("{:?}: plug removed", port);
            }
        }
    }
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
