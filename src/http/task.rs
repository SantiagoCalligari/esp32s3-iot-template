use crate::http::App;
use crate::rprintln;
use crate::utils::mk_static;
use embassy_net::Stack;
use picoserve::{AppBuilder, AppRouter, Server};

#[embassy_executor::task(pool_size = super::HTTP_POOL_SIZE)]
pub async fn http_task(task_id: usize, stack: Stack<'static>) -> ! {
    rprintln!("HTTP task {} initialized", task_id);

    // Mover buffers a memoria estática para evitar stack overflow
    let tcp_rx_buffer = mk_static!([u8; 2048], [0; 2048]);
    let tcp_tx_buffer = mk_static!([u8; 2048], [0; 2048]);
    let http_buffer = mk_static!([u8; 4096], [0; 4096]);

    let app = App;
    let router = mk_static!(AppRouter<App>, app.build_app());
    let config = super::create_config();

    Server::new(router, config, http_buffer)
        .listen_and_serve(task_id, stack, 80, tcp_rx_buffer, tcp_tx_buffer)
        .await
        .into_never()
}
