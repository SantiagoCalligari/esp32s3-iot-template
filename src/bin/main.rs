#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
#![deny(clippy::large_stack_frames)]

use core::net::Ipv4Addr;

use embassy_executor::Spawner;
use embassy_net::{Ipv4Cidr, Runner, StackResources, StaticConfigV4};
use embassy_time::{Duration, Timer};
use esp_hal::clock::CpuClock;
use esp_hal::timer::timg::TimerGroup;
use esp_radio::{
    Controller,
    wifi::{AccessPointConfig, ModeConfig, WifiController, WifiDevice},
};
use leasehund::DhcpServer;
use rtt_target::rprintln;

#[macro_export]
macro_rules! mk_static {
    ($t:ty,$val:expr) => {{
        static STATIC_CELL: static_cell::StaticCell<$t> = static_cell::StaticCell::new();
        #[deny(unused_attributes)]
        let x = STATIC_CELL.uninit().write(($val));
        x
    }};
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;

esp_bootloader_esp_idf::esp_app_desc!();

#[allow(
    clippy::large_stack_frames,
    reason = "it's not unusual to allocate larger buffers etc. in main"
)]
#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    rtt_target::rtt_init_print!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    rprintln!("Embassy initialized!");

    let radio_init = &*mk_static!(
        Controller<'static>,
        esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller")
    );
    let (wifi_controller, interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi controller");

    let ip_address = Ipv4Cidr::new(Ipv4Addr::new(192, 168, 1, 1), 24);
    let net_config = embassy_net::Config::ipv4_static(StaticConfigV4 {
        address: ip_address,
        gateway: Some(Ipv4Addr::new(192, 168, 1, 1)),
        dns_servers: Default::default(),
    });

    let (stack, runner) = embassy_net::new(
        interfaces.ap,
        net_config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        1234,
    );

    spawner.spawn(net_task(runner)).ok();

    let dhcp_server: DhcpServer<32, 4> = DhcpServer::new_with_dns(
        Ipv4Addr::new(192, 168, 1, 1),
        Ipv4Addr::new(255, 255, 255, 0),
        Ipv4Addr::new(192, 168, 1, 1),
        Ipv4Addr::new(8, 8, 8, 8),
        Ipv4Addr::new(192, 168, 1, 100),
        Ipv4Addr::new(192, 168, 1, 200),
    );

    spawner.spawn(connection(wifi_controller)).ok();

    spawner.spawn(dhcp_task(stack, dhcp_server)).ok();

    let mut counter = 0;
    loop {
        rprintln!("Main loop ejecutando... {}", counter);
        counter += 1;
        Timer::after(Duration::from_secs(5)).await;
    }
}

#[embassy_executor::task]
async fn dhcp_task(stack: embassy_net::Stack<'static>, mut server: DhcpServer<32, 4>) {
    rprintln!("DHCP server iniciado en tarea separada");
    server.run(stack).await;
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    rprintln!("net_task iniciado");
    runner.run().await;
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    rprintln!("start connection task");
    rprintln!("Device capabilities: {:?}", controller.capabilities());

    let ap_config = ModeConfig::AccessPoint(
        AccessPointConfig::default()
            .with_ssid("esp32".into())
            .with_password("password".into())
            .with_auth_method(esp_radio::wifi::AuthMethod::Wpa2Personal),
    );
    controller.set_config(&ap_config).unwrap();
    rprintln!("Starting wifi AP");
    controller.start_async().await.unwrap();
    rprintln!("Wifi AP started en 192.168.1.1");

    loop {
        embassy_time::Timer::after(Duration::from_secs(3600)).await;
    }
}
