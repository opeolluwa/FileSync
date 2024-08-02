use axum::http::Method;

use tower_http::cors::Any;
use tower_http::cors::CorsLayer;
use tower_http::trace::DefaultMakeSpan;
use tower_http::trace::TraceLayer;
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use axum::extract::DefaultBodyLimit;

mod router;
mod routes;
/**
 * @function core_server
 * the application core responsible for handling file upload to client
 *  machine and file download to the host machine
 */
#[derive(Debug)]
/// the sever can be created with multiple instances
pub struct HttpServer;
impl HttpServer {
    pub async fn run(port: u64, ip_address: (u64, u64, u64, u64)) {
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new(
                std::env::var("RUST_LOG")
                    .unwrap_or_else(|_| "send_file_core=debug,tower_http=debug".into()),
            ))
            .with(tracing_subscriber::fmt::layer())
            .init();

        // define cors scope as any
        // change this later to only allow get and post http verbs
        // TODO: restrict this in the future to only sendfile proxy server for example http://sendfile/dhsdo
        let cors_layer = CorsLayer::new()
            .allow_headers(Any)
            .allow_methods([Method::GET, Method::POST]) // restrict methods
            .allow_origin(Any);

        // define file limit layer as 10GB
        // see information here <https://docs.rs/axum/0.6.2/axum/extract/struct.DefaultBodyLimit.html#%E2%80%A6>
        let file_size_limit = 10 * 1024 * 1024 * 1024;
        let file_limit = DefaultBodyLimit::max(file_size_limit);

        //  run the https server on localhost then feed off the connection using the wifi gateway, the same way Vite/Vue CLI would do the core server
        // this is currently achieved by binding the server to the device default ip address
        let my_local_ip = format!(
            "{}.{}.{}.{}",
            ip_address.0, ip_address.1, ip_address.2, ip_address.3
        )
        .parse::<std::net::Ipv4Addr>()
        .expect("Invalid IP Address V4 suspected");
        let ip_address = format!("{:?}:{:?}", my_local_ip, port);
        let ip_address = ip_address
            .parse::<std::net::SocketAddr>()
            .expect("invalid socket address");

        let app = router::app()
            .layer(file_limit)
            .layer(cors_layer)
            .layer(tower_http::trace::TraceLayer::new_for_http())
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::default().include_headers(true)),
            );

        // run it
        let listener = tokio::net::TcpListener::bind(&ip_address).await.unwrap();

        tracing::debug!(" the server port is http://{}", ip_address);

        axum::serve(listener, app).await.unwrap();
    }
}
