#![no_std]
#![no_main]
use core::default::Default;

use defmt::*;
use embassy_embedded_hal::shared_bus::asynch::i2c::I2cDevice;
use embassy_executor::Spawner;
use embassy_imxrt::gpio::{Input, Inverter, Pull};
use embassy_imxrt::i2c::master::{I2cMaster, Speed};
use embassy_imxrt::i2c::Async;
use embassy_imxrt::{self, bind_interrupts, peripherals};
use embassy_sync::blocking_mutex::raw::NoopRawMutex;
use embassy_sync::mutex::Mutex;
use embassy_sync::once_lock::OnceLock;
use mimxrt600_fcb::FlexSPIFlashConfigurationBlock;
use static_cell::StaticCell;
use tps6699x::asynchronous::embassy;
use tps6699x::ADDR0;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    FLEXCOMM2 => embassy_imxrt::i2c::InterruptHandler<peripherals::FLEXCOMM2>;
});

type Bus<'a> = I2cDevice<'a, NoopRawMutex, I2cMaster<'a, Async>>;
type Controller<'a> = embassy::Controller<NoopRawMutex, Bus<'a>>;

type Interrupt<'a> = embassy::Interrupt<'a, NoopRawMutex, Bus<'a>>;
type Tps6699x<'a> = embassy::Tps6699x<'a, NoopRawMutex, Bus<'a>>;

#[embassy_executor::task]
async fn interrupt_task(mut int_in: Input<'static>, mut interrupt: Interrupt<'static>) {
    embassy::task::interrupt_task(&mut int_in, [&mut interrupt]).await;
}

#[embassy_executor::task]
async fn pd_task(mut pd: Tps6699x<'static>) {
    let fw = include_bytes!("../../TPS66994_Host.bin");

    let mut controllers = [&mut pd];

    let target_version = embassy::fw_update::get_customer_use_data(fw.as_slice()).unwrap();
    info!("Target FW Version: {:#x}", target_version);

    for (i, controller) in controllers.iter_mut().enumerate() {
        let version = controller.get_customer_use().await.unwrap();
        info!("Controller {}: Current FW Version: {:#x}", i, version);
    }

    info!("Performing PD FW update");
    embassy::fw_update::perform_fw_update(&mut controllers, fw.as_slice())
        .await
        .unwrap();

    for (i, controller) in controllers.iter_mut().enumerate() {
        let version = controller.get_customer_use().await.unwrap();
        if version != target_version {
            error!(
                "Controller {}: Failed to update FW, target version: {:#x}, current version: {:#x}",
                i, version, target_version
            );
        } else {
            info!("Controller {}: FW update complete", i);
        }
    }
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let p = embassy_imxrt::init(Default::default());

    let int_in = Input::new(p.PIO1_0, Pull::Up, Inverter::Disabled);
    static BUS: OnceLock<Mutex<NoopRawMutex, I2cMaster<'static, Async>>> = OnceLock::new();
    let bus = BUS.get_or_init(|| {
        Mutex::new(I2cMaster::new_async(p.FLEXCOMM2, p.PIO0_18, p.PIO0_17, Irqs, Speed::Standard, p.DMA0_CH5).unwrap())
    });

    let device = I2cDevice::new(bus);

    static CONTROLLER: StaticCell<Controller<'static>> = StaticCell::new();
    let controller = CONTROLLER.init(Controller::new(device, ADDR0).unwrap());
    let (pd, interrupt) = controller.make_parts();

    spawner.must_spawn(interrupt_task(int_in, interrupt));
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
