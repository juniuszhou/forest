mod cli;
mod log;

use self::cli::cli;
use ferret_libp2p::service::NetworkEvent;
use futures::prelude::*;
use network::service::NetworkService;
use slog::info;
use tokio::runtime::Runtime;
use tokio::sync::mpsc;

fn main() {
    let log = log::setup_logging();
    info!(log, "Starting Ferret");

    // Capture CLI inputs
    let config = cli(&log).expect("CLI error");

    // Create the tokio runtime
    let rt = Runtime::new().unwrap();

    // Create the channel so we can receive messages from NetworkService
    let (tx, _rx) = mpsc::unbounded_channel::<NetworkEvent>();

    // Create the default libp2p config
    // Start the NetworkService. Returns net_tx so  you can pass messages in.
    let network_service = NetworkService::new(&config.network, &log, tx);

    // Start All Services
    network_service.start(&rt.executor());
    // Stop All Services
    // network_service.stop();

    // Shutdown tokio runtime
    rt.shutdown_on_idle().wait().unwrap();
    info!(log, "Ferret finish shutdown");
}
