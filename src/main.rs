// use crate::envoy_cds::CDS;
mod cache;
mod configuration;
mod envoy;
mod envoy_cds;
mod envoy_helpers;
mod service;

use crate::envoy::envoy::service::cluster::v3::cluster_discovery_service_server::ClusterDiscoveryServiceServer;
use log::info;
use tonic::transport::Server;
use tonic::{Request, Status};

fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    println!("Intercepting request: {:?}", req);
    Ok(req)
}

fn initialize_config_loader(cache: cache::LocalCache) -> bool {
    tokio::task::spawn_blocking(move || loop {
        let config = configuration::Config::parse_config("./log.json");
        //@TODO Check md5 here
        cache.add_multiple(&mut config.export_config_to_envoy());
        // @TODO mybe something golang ticker here? should be a better way to do this.
        println!("Running {:?}", std::time::Instant::now());
        std::thread::sleep(std::time::Duration::from_secs(5));
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
    let config_cache = cache::LocalCache::new();

    let cds = envoy_cds::CDS::new(config_cache);

    initialize_config_loader(config_cache);

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
