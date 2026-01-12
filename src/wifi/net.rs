use crate::rprintln;
use embassy_net::Runner;
use esp_radio::wifi::WifiDevice;

#[embassy_executor::task]
pub async fn net_task(mut runner: Runner<'static, WifiDevice<'static>>) {
    rprintln!("Net: Iniciando network stack runner");
    runner.run().await;
}
