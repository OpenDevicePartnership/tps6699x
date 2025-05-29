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
use embassy_time::Delay;
use mimxrt600_fcb::FlexSPIFlashConfigurationBlock;
use static_cell::StaticCell;
use tps6699x::asynchronous::embassy as pd_controller;
use tps6699x::asynchronous::fw_update::perform_fw_update_borrowed;
use tps6699x::fw_update::UpdateConfig;
use tps6699x::ADDR0;
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

    let int_in = Input::new(p.PIO1_0, Pull::Up, Inverter::Disabled);
    static BUS: OnceLock<Mutex<NoopRawMutex, I2cMaster<'static, Async>>> = OnceLock::new();
    let bus = BUS.get_or_init(|| {
        Mutex::new(I2cMaster::new_async(p.FLEXCOMM2, p.PIO0_18, p.PIO0_17, Irqs, Speed::Standard, p.DMA0_CH5).unwrap())
    });

    let device = I2cDevice::new(bus);

    static CONTROLLER: StaticCell<Controller<'static>> = StaticCell::new();
    let controller = CONTROLLER.init(Controller::new_tps66994(device, ADDR0).unwrap());
    let (mut pd, interrupt) = controller.make_parts();

    let mut delay = Delay;
    info!("Resetting PD controller");
    pd.reset(&mut delay).await.unwrap();

    info!("Spawing PD interrupt task");
    spawner.must_spawn(interrupt_task(int_in, interrupt));

    let pd_fw_bytes = [0u8].as_slice(); //include_bytes!("../../fw.bin").as_slice();

    let mut controllers = [&mut pd];
    for (i, controller) in controllers.iter_mut().enumerate() {
        let customer_use = controller.get_customer_use().await.unwrap();
        info!("Controller {}: Customer use: {:#x}", i, customer_use);
    }

    info!("Performing PD FW update");
    let config = UpdateConfig::default();

    let mut controllers = [&mut pd];
    let mut guards = [const { None }; 2];
    perform_fw_update_borrowed(&mut controllers, &mut guards, &mut delay, config, pd_fw_bytes)
        .await
        .unwrap();

    let mut controllers = [&mut pd];
    for (i, controller) in controllers.iter_mut().enumerate() {
        let customer_use = controller.get_customer_use().await.unwrap();
        info!(
            "Controller {}: FW update complete, customer use: {:#x}",
            i, customer_use
        );
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
