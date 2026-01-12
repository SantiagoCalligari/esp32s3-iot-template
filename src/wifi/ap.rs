use crate::rprintln;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use esp_radio::wifi::{AccessPointConfig, AuthMethod, ModeConfig, WifiController};

#[embassy_executor::task]
pub async fn connection(mut controller: WifiController<'static>) {
    rprintln!("WiFi: Iniciando tarea de conexión");
    rprintln!("WiFi: Capabilities = {:?}", controller.capabilities());

    let ap_config = ModeConfig::AccessPoint(
        AccessPointConfig::default()
            .with_ssid("esp32".into())
            .with_password("password".into())
            .with_auth_method(AuthMethod::Wpa2Personal),
    );

    controller.set_config(&ap_config).unwrap();
    rprintln!("WiFi: Iniciando AP...");
    controller.start_async().await.unwrap();
    rprintln!("WiFi: AP iniciado en 192.168.1.1");

    loop {
        Timer::after(Duration::from_secs(60)).await;
        rprintln!("WiFi: AP activo...");
    }
}
