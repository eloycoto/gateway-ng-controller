use prost_types::Duration;
use serde::{Deserialize, Serialize};

use crate::envoy::envoy::config::cluster::v3::Cluster;
use crate::envoy::envoy::config::endpoint::v3::lb_endpoint::HostIdentifier;
use crate::envoy::envoy::config::endpoint::v3::ClusterLoadAssignment;
use crate::envoy::envoy::config::endpoint::v3::Endpoint;
use crate::envoy::envoy::config::endpoint::v3::LbEndpoint;
use crate::envoy::envoy::config::endpoint::v3::LocalityLbEndpoints;

// @TODO target domain connect_timeout
// @TODO optional fields
#[derive(Serialize, Deserialize, Debug)]
pub struct Service {
    pub id: u32,
    pub hosts: Vec<std::string::String>,
    pub policies: Vec<std::string::String>,
    pub target_domain: std::string::String,
}

impl Service {
    // pub fn export_to_envoy(&self) {
    //     println!("Export to envoy {:?}", self);
    // }

    pub fn export_clusters(&self) -> Cluster {
        return Cluster {
            name: self.target_domain.to_string(),
            connect_timeout: Some(Duration {
                seconds: 1,
                nanos: 0,
            }),
            load_assignment: Some(ClusterLoadAssignment {
                cluster_name: self.target_domain.to_string(),
                endpoints: vec![LocalityLbEndpoints {
                    lb_endpoints: vec![LbEndpoint {
                        host_identifier: Some(HostIdentifier::Endpoint(Endpoint {
                            hostname: self.target_domain.to_string(),
                            ..Default::default()
                        })),
                        ..Default::default()
                    }],
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };
    }

    // pub fn export_listener(&self) {
    //     print!("To be implemented");
    // }
}
