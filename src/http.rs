use crate::{root::root, Args, HourlyWeather};
use std::{io::Error, net::SocketAddr};
use tracing::info;

pub async fn start(args: Args) -> Result<(), Error> {
    let state = HourlyWeather::new("weather.segment7.net");
    let app = root(&args).with_state(state);

    let tls_config = args.tls_config().await;

    let port = 8443;
    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    info!("binding {addr:?}");

    axum_server::bind_rustls(addr, tls_config)
        .serve(app.into_make_service_with_connect_info::<SocketAddr>())
        .await
}
