#![no_std]
#![no_main]
#![deny(clippy::mem_forget, clippy::large_stack_frames)]

use core::net::Ipv4Addr;
use embassy_executor::Spawner;
use embassy_net::{self, Ipv4Cidr, StackResources, StaticConfigV4};
use embassy_time::Timer;
use esp_hal::{clock::CpuClock, timer::timg::TimerGroup};
use esp_radio::Controller; // ✓ SIN el módulo wifi aquí
use leasehund::DhcpServer; // ✓ Importar DhcpServer directamente
use wifi_template::{dhcp, rprintln, utils::mk_static, wifi}; // ✓ Solo una vez wifi

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    loop {}
}

extern crate alloc;
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_rtos::main]
async fn main(spawner: Spawner) -> ! {
    rtt_target::rtt_init_print!();

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(#[esp_hal::ram(reclaimed)] size: 73744);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0);

    rprintln!("========================================");
    rprintln!("ESP32 Wi-Fi AP + DHCP Server");
    rprintln!("========================================");

    let radio_init = &*mk_static!(
        Controller<'static>,
        esp_radio::init().expect("Failed to initialize Wi-Fi/BLE controller")
    );

    let (wifi_controller, interfaces) =
        esp_radio::wifi::new(&radio_init, peripherals.WIFI, Default::default())
            .expect("Failed to initialize Wi-Fi");

    let net_config = embassy_net::Config::ipv4_static(StaticConfigV4 {
        address: Ipv4Cidr::new(Ipv4Addr::new(192, 168, 1, 1), 24),
        gateway: Some(Ipv4Addr::new(192, 168, 1, 1)),
        dns_servers: Default::default(),
    });

    let (stack, runner) = embassy_net::new(
        interfaces.ap,
        net_config,
        mk_static!(StackResources<3>, StackResources::<3>::new()),
        1234,
    );

    spawner.spawn(wifi::net_task(runner)).ok();
    spawner.spawn(wifi::connection(wifi_controller)).ok();

    let dhcp_server: leasehund::DhcpServer<32, 4> = DhcpServer::new_with_dns(
        Ipv4Addr::new(192, 168, 1, 1),
        Ipv4Addr::new(255, 255, 255, 0),
        Ipv4Addr::new(192, 168, 1, 1),
        Ipv4Addr::new(8, 8, 8, 8),
        Ipv4Addr::new(192, 168, 1, 100),
        Ipv4Addr::new(192, 168, 1, 200),
    );
    spawner.spawn(dhcp::dhcp_task(stack, dhcp_server)).ok();

    rprintln!("Todas las tareas iniciadas correctamente");
    let mut counter = 0;
    loop {
        rprintln!("Main: Sistema operativo {}", counter);
        counter += 1;
        Timer::after_secs(10).await;
    }
}
