mod server;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;


#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder.
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
    let server = match server::spawn_server() {
        Ok(server) =>{ 
            tracing::info!("spawn server call is success") ;
            server
        },
        Err(err) => {
            tracing::error!("unable to spawn server {}", err);
            panic!("error spawning the server");
        }
    };

    let _= server.await ;
}
