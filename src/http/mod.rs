pub mod task;
use embassy_time::Duration;
use picoserve::{
    AppBuilder, Router, make_static,
    response::IntoResponse,
    routing::{PathRouter, get},
};

pub struct App;

impl AppBuilder for App {
    type PathRouter = impl PathRouter;

    fn build_app(self) -> Router<Self::PathRouter> {
        Router::new().route("/", get(root_handler))
    }
}

async fn root_handler() -> impl IntoResponse {
    "Hola Mundo desde ESP32 AP!"
}

pub const HTTP_POOL_SIZE: usize = 2;

pub fn create_config() -> &'static picoserve::Config<Duration> {
    make_static!(
        picoserve::Config<Duration>,
        picoserve::Config::new(picoserve::Timeouts {
            start_read_request: Some(Duration::from_secs(5)),
            persistent_start_read_request: Some(Duration::from_secs(1)),
            read_request: Some(Duration::from_secs(1)),
            write: Some(Duration::from_secs(1)),
        })
        .keep_connection_alive()
    )
}
