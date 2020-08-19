// use crate::envoy_cds::CDS;
mod envoy;
mod envoy_cds;

use crate::envoy::envoy::service::cluster::v3::cluster_discovery_service_server::ClusterDiscoveryServiceServer;
use log::info;
use tonic::transport::Server;
use tonic::{Request, Status};

fn intercept(req: Request<()>) -> Result<Request<()>, Status> {
    println!("Intercepting request: {:?}", req);
    Ok(req)
}

fn initialize_config_loader() -> bool {
    return true;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // @TODO read env variable here:
    println!("Starting listening on 5000 port");
    let addr = "0.0.0.0:5000".parse().unwrap();

    //@TODO add a cache here, where the info can be set, and CDS read from that.
    initialize_config_loader();

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
