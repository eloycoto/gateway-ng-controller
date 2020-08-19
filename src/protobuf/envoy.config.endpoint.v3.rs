// [#protodoc-title: Endpoints]

/// Upstream host identifier.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Endpoint {
    /// The upstream host address.
    ///
    /// .. attention::
    ///
    ///   The form of host address depends on the given cluster type. For STATIC or EDS,
    ///   it is expected to be a direct IP address (or something resolvable by the
    ///   specified :ref:`resolver <envoy_api_field_config.core.v3.SocketAddress.resolver_name>`
    ///   in the Address). For LOGICAL or STRICT DNS, it is expected to be hostname,
    ///   and will be resolved via DNS.
    #[prost(message, optional, tag = "1")]
    pub address: ::std::option::Option<super::super::core::v3::Address>,
    /// The optional health check configuration is used as configuration for the
    /// health checker to contact the health checked host.
    ///
    /// .. attention::
    ///
    ///   This takes into effect only for upstream clusters with
    ///   :ref:`active health checking <arch_overview_health_checking>` enabled.
    #[prost(message, optional, tag = "2")]
    pub health_check_config: ::std::option::Option<endpoint::HealthCheckConfig>,
    /// The hostname associated with this endpoint. This hostname is not used for routing or address
    /// resolution. If provided, it will be associated with the endpoint, and can be used for features
    /// that require a hostname, like
    /// :ref:`auto_host_rewrite <envoy_api_field_config.route.v3.RouteAction.auto_host_rewrite>`.
    #[prost(string, tag = "3")]
    pub hostname: std::string::String,
}
pub mod endpoint {
    /// The optional health check configuration.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct HealthCheckConfig {
        /// Optional alternative health check port value.
        ///
        /// By default the health check address port of an upstream host is the same
        /// as the host's serving address port. This provides an alternative health
        /// check port. Setting this with a non-zero value allows an upstream host
        /// to have different health check address port.
        #[prost(uint32, tag = "1")]
        pub port_value: u32,
        /// By default, the host header for L7 health checks is controlled by cluster level configuration
        /// (see: :ref:`host <envoy_api_field_config.core.v3.HealthCheck.HttpHealthCheck.host>` and
        /// :ref:`authority <envoy_api_field_config.core.v3.HealthCheck.GrpcHealthCheck.authority>`). Setting this
        /// to a non-empty value allows overriding the cluster level configuration for a specific
        /// endpoint.
        #[prost(string, tag = "2")]
        pub hostname: std::string::String,
    }
}
/// An Endpoint that Envoy can route traffic to.
/// [#next-free-field: 6]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LbEndpoint {
    /// Optional health status when known and supplied by EDS server.
    #[prost(enumeration = "super::super::core::v3::HealthStatus", tag = "2")]
    pub health_status: i32,
    /// The endpoint metadata specifies values that may be used by the load
    /// balancer to select endpoints in a cluster for a given request. The filter
    /// name should be specified as *envoy.lb*. An example boolean key-value pair
    /// is *canary*, providing the optional canary status of the upstream host.
    /// This may be matched against in a route's
    /// :ref:`RouteAction <envoy_api_msg_config.route.v3.RouteAction>` metadata_match field
    /// to subset the endpoints considered in cluster load balancing.
    #[prost(message, optional, tag = "3")]
    pub metadata: ::std::option::Option<super::super::core::v3::Metadata>,
    /// The optional load balancing weight of the upstream host; at least 1.
    /// Envoy uses the load balancing weight in some of the built in load
    /// balancers. The load balancing weight for an endpoint is divided by the sum
    /// of the weights of all endpoints in the endpoint's locality to produce a
    /// percentage of traffic for the endpoint. This percentage is then further
    /// weighted by the endpoint's locality's load balancing weight from
    /// LocalityLbEndpoints. If unspecified, each host is presumed to have equal
    /// weight in a locality. The sum of the weights of all endpoints in the
    /// endpoint's locality must not exceed uint32_t maximal value (4294967295).
    #[prost(message, optional, tag = "4")]
    pub load_balancing_weight: ::std::option::Option<u32>,
    /// Upstream host identifier or a named reference.
    #[prost(oneof = "lb_endpoint::HostIdentifier", tags = "1, 5")]
    pub host_identifier: ::std::option::Option<lb_endpoint::HostIdentifier>,
}
pub mod lb_endpoint {
    /// Upstream host identifier or a named reference.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum HostIdentifier {
        #[prost(message, tag = "1")]
        Endpoint(super::Endpoint),
        /// [#not-implemented-hide:]
        #[prost(string, tag = "5")]
        EndpointName(std::string::String),
    }
}
/// A group of endpoints belonging to a Locality.
/// One can have multiple LocalityLbEndpoints for a locality, but this is
/// generally only done if the different groups need to have different load
/// balancing weights or different priorities.
/// [#next-free-field: 7]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LocalityLbEndpoints {
    /// Identifies location of where the upstream hosts run.
    #[prost(message, optional, tag = "1")]
    pub locality: ::std::option::Option<super::super::core::v3::Locality>,
    /// The group of endpoints belonging to the locality specified.
    #[prost(message, repeated, tag = "2")]
    pub lb_endpoints: ::std::vec::Vec<LbEndpoint>,
    /// Optional: Per priority/region/zone/sub_zone weight; at least 1. The load
    /// balancing weight for a locality is divided by the sum of the weights of all
    /// localities  at the same priority level to produce the effective percentage
    /// of traffic for the locality. The sum of the weights of all localities at
    /// the same priority level must not exceed uint32_t maximal value (4294967295).
    ///
    /// Locality weights are only considered when :ref:`locality weighted load
    /// balancing <arch_overview_load_balancing_locality_weighted_lb>` is
    /// configured. These weights are ignored otherwise. If no weights are
    /// specified when locality weighted load balancing is enabled, the locality is
    /// assigned no load.
    #[prost(message, optional, tag = "3")]
    pub load_balancing_weight: ::std::option::Option<u32>,
    /// Optional: the priority for this LocalityLbEndpoints. If unspecified this will
    /// default to the highest priority (0).
    ///
    /// Under usual circumstances, Envoy will only select endpoints for the highest
    /// priority (0). In the event all endpoints for a particular priority are
    /// unavailable/unhealthy, Envoy will fail over to selecting endpoints for the
    /// next highest priority group.
    ///
    /// Priorities should range from 0 (highest) to N (lowest) without skipping.
    #[prost(uint32, tag = "5")]
    pub priority: u32,
    /// Optional: Per locality proximity value which indicates how close this
    /// locality is from the source locality. This value only provides ordering
    /// information (lower the value, closer it is to the source locality).
    /// This will be consumed by load balancing schemes that need proximity order
    /// to determine where to route the requests.
    /// [#not-implemented-hide:]
    #[prost(message, optional, tag = "6")]
    pub proximity: ::std::option::Option<u32>,
}
// [#protodoc-title: Endpoint configuration]
// Endpoint discovery :ref:`architecture overview <arch_overview_service_discovery_types_eds>`

/// Each route from RDS will map to a single cluster or traffic split across
/// clusters using weights expressed in the RDS WeightedCluster.
///
/// With EDS, each cluster is treated independently from a LB perspective, with
/// LB taking place between the Localities within a cluster and at a finer
/// granularity between the hosts within a locality. The percentage of traffic
/// for each endpoint is determined by both its load_balancing_weight, and the
/// load_balancing_weight of its locality. First, a locality will be selected,
/// then an endpoint within that locality will be chose based on its weight.
/// [#next-free-field: 6]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterLoadAssignment {
    /// Name of the cluster. This will be the :ref:`service_name
    /// <envoy_api_field_config.cluster.v3.Cluster.EdsClusterConfig.service_name>` value if specified
    /// in the cluster :ref:`EdsClusterConfig
    /// <envoy_api_msg_config.cluster.v3.Cluster.EdsClusterConfig>`.
    #[prost(string, tag = "1")]
    pub cluster_name: std::string::String,
    /// List of endpoints to load balance to.
    #[prost(message, repeated, tag = "2")]
    pub endpoints: ::std::vec::Vec<LocalityLbEndpoints>,
    /// Map of named endpoints that can be referenced in LocalityLbEndpoints.
    /// [#not-implemented-hide:]
    #[prost(map = "string, message", tag = "5")]
    pub named_endpoints: ::std::collections::HashMap<std::string::String, Endpoint>,
    /// Load balancing policy settings.
    #[prost(message, optional, tag = "4")]
    pub policy: ::std::option::Option<cluster_load_assignment::Policy>,
}
pub mod cluster_load_assignment {
    /// Load balancing policy settings.
    /// [#next-free-field: 6]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Policy {
        /// Action to trim the overall incoming traffic to protect the upstream
        /// hosts. This action allows protection in case the hosts are unable to
        /// recover from an outage, or unable to autoscale or unable to handle
        /// incoming traffic volume for any reason.
        ///
        /// At the client each category is applied one after the other to generate
        /// the 'actual' drop percentage on all outgoing traffic. For example:
        ///
        /// .. code-block:: json
        ///
        ///  { "drop_overloads": [
        ///      { "category": "throttle", "drop_percentage": 60 }
        ///      { "category": "lb", "drop_percentage": 50 }
        ///  ]}
        ///
        /// The actual drop percentages applied to the traffic at the clients will be
        ///    "throttle"_drop = 60%
        ///    "lb"_drop = 20%  // 50% of the remaining 'actual' load, which is 40%.
        ///    actual_outgoing_load = 20% // remaining after applying all categories.
        /// [#not-implemented-hide:]
        #[prost(message, repeated, tag = "2")]
        pub drop_overloads: ::std::vec::Vec<policy::DropOverload>,
        /// Priority levels and localities are considered overprovisioned with this
        /// factor (in percentage). This means that we don't consider a priority
        /// level or locality unhealthy until the fraction of healthy hosts
        /// multiplied by the overprovisioning factor drops below 100.
        /// With the default value 140(1.4), Envoy doesn't consider a priority level
        /// or a locality unhealthy until their percentage of healthy hosts drops
        /// below 72%. For example:
        ///
        /// .. code-block:: json
        ///
        ///  { "overprovisioning_factor": 100 }
        ///
        /// Read more at :ref:`priority levels <arch_overview_load_balancing_priority_levels>` and
        /// :ref:`localities <arch_overview_load_balancing_locality_weighted_lb>`.
        #[prost(message, optional, tag = "3")]
        pub overprovisioning_factor: ::std::option::Option<u32>,
        /// The max time until which the endpoints from this assignment can be used.
        /// If no new assignments are received before this time expires the endpoints
        /// are considered stale and should be marked unhealthy.
        /// Defaults to 0 which means endpoints never go stale.
        #[prost(message, optional, tag = "4")]
        pub endpoint_stale_after: ::std::option::Option<::prost_types::Duration>,
    }
    pub mod policy {
        /// [#not-implemented-hide:]
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct DropOverload {
            /// Identifier for the policy specifying the drop.
            #[prost(string, tag = "1")]
            pub category: std::string::String,
            /// Percentage of traffic that should be dropped for the category.
            #[prost(message, optional, tag = "2")]
            pub drop_percentage: ::std::option::Option<
                super::super::super::super::super::r#type::v3::FractionalPercent,
            >,
        }
    }
}
