#![no_std]
#![no_main]

use cortex_m::peripheral::NVIC;
use embassy_executor::Spawner;
use embassy_nrf::{interrupt, pac};
use embassy_time::Timer;
use nrf_modem::{ConnectionPreference, SystemMode};
use tinyrlibc::strncpy as _;
use {defmt_rtt as _, panic_probe as _};

#[interrupt]
#[allow(non_snake_case)]
fn IPC() {
    nrf_modem::ipc_irq_handler();
}

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut cp = cortex_m::Peripherals::take().unwrap();

    let mode = SystemMode {
        lte_support: true,
        nbiot_support: true,
        lte_psm_support: true,
        gnss_support: true,
        preference: ConnectionPreference::None,
    };
    nrf_modem::init(mode).await.unwrap();

    unsafe {
        NVIC::unmask(pac::Interrupt::IPC);
        cp.NVIC.set_priority(pac::Interrupt::IPC, 0 << 5);
    }

    loop {
        Timer::after_millis(1000).await;
    }
}
