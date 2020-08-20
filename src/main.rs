// use crate::envoy_cds::CDS;
mod cache;
mod configuration;
mod envoy;
mod envoy_cds;
mod service;

use crate::envoy::envoy::service::cluster::v3::cluster_discovery_service_server::ClusterDiscoveryServiceServer;
use log::info;
use tonic::transport::Server;
use tonic::{Request, Status};

fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    println!("Intercepting request: {:?}", req);
    Ok(req)
}

fn initialize_config_loader(mut cache: cache::LocalCache<std::string::String>) -> bool {
    tokio::task::spawn_blocking(move || loop {
        let mut config = configuration::Config::parse_config("./log.json");
        println!("Config-->{:?}", config);
        // @TODO maybe something golang ticker here? should be a better way to do this.
        // cache.add();
        println!("Running {:?}", std::time::Instant::now());
        std::thread::sleep(std::time::Duration::from_secs(30));
        // println!("Finished, cache={:?}", cache);
    });
    return true;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // @TODO read env variable here:
    println!("Starting listening on 5000 port");
    let addr = "0.0.0.0:5000".parse().unwrap();

    // configuration::Config::parse_config("./log.json");
    //@TODO add a cache here, where the info can be set, and CDS read from that.
    //
    let config_cache = cache::LocalCache::<std::string::String>::new();
    initialize_config_loader(config_cache);

    let cds = envoy_cds::CDS::default();
    info!("CDS service listening on {}", addr);
    info!("CDS service listening on {:?}", cds);

    Server::builder()
        .add_service(ClusterDiscoveryServiceServer::with_interceptor(
            cds, intercept,
        ))
        .serve(addr)
        .await?;

    Ok(())
}
