use crate::http::App;
use crate::rprintln;
use embassy_net::Stack;
use picoserve::{AppBuilder, AppRouter, Server, make_static};

#[embassy_executor::task(pool_size = super::HTTP_POOL_SIZE)]
pub async fn http_task(task_id: usize, stack: Stack<'static>) -> ! {
    rprintln!("HTTP task {} intialized", task_id);
    let mut tcp_rx_buffer = [0, 127];
    let mut tcp_tx_buffer = [0, 127];
    let mut http_buffer = [0, 255];

    let app = App;
    let router = make_static!(AppRouter<App>, app.build_app());
    let config = super::create_config();

    Server::new(router, config, &mut http_buffer)
        .listen_and_serve(task_id, stack, 80, &mut tcp_rx_buffer, &mut tcp_tx_buffer)
        .await
        .into_never()
}
