#![no_std]
#![no_main]
use core::default::Default;

use defmt::info;
use embassy_executor::Spawner;
use embassy_imxrt::gpio::{Input, Inverter, Pull};
use embassy_imxrt::i2c::master::{I2cMaster, Speed};
use embassy_imxrt::{self, bind_interrupts, peripherals};
use embedded_usb_pd::PortId;
use mimxrt600_fcb::FlexSPIFlashConfigurationBlock;
use tps6699x::asynchronous::internal::Tps6699x;
use tps6699x::registers::field_sets::IntEventBus1;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    FLEXCOMM2 => embassy_imxrt::i2c::InterruptHandler<peripherals::FLEXCOMM2>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let p = embassy_imxrt::init(Default::default());

    let mut int_in = Input::new(p.PIO1_0, Pull::Up, Inverter::Disabled);
    let device = I2cMaster::new_async(p.FLEXCOMM2, p.PIO0_18, p.PIO0_17, Irqs, Speed::Standard, p.DMA0_CH5).unwrap();

    let mut pd = Tps6699x::new(device, tps6699x::ADDR0);
    loop {
        info!("Wating for interrupt");
        int_in.wait_for_low().await;
        for port in 0..2 {
            let port = PortId(port as u8);
            let int = pd.clear_interrupt(port).await.unwrap();

            if int == IntEventBus1::new_zero() {
                continue;
            }

            info!("Got interrupt({}): {}", port, int);

            info!("Getting port status");
            let status = pd.get_port_status(port).await.unwrap();
            info!("Port status: {}", status);

            if !status.plug_present() {
                info!("Plug removed: {}", port.0);
                continue;
            }

            info!("Plug connected: {} ", port.0);

            let pdo = pd.get_active_pdo_contract(port).await.unwrap();
            info!("PDO: {}", pdo);

            let rdo = pd.get_active_rdo_contract(port).await.unwrap();
            info!("RDO: {}", rdo);
            info!("Done");
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
