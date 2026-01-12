use crate::rprintln;
use embassy_net::Stack;
use leasehund::DhcpServer;

#[embassy_executor::task]
pub async fn dhcp_task(stack: Stack<'static>, mut server: DhcpServer<32, 4>) {
    rprintln!("DHCP: Iniciando servidor...");
    server.run(stack).await;
}
