mod server;
mod utils ;
mod api ;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;
mod schema;
mod open_api;


#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()

        .with_max_level(Level::TRACE)
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
