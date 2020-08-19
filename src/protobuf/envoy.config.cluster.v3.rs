// [#protodoc-title: Circuit breakers]

/// :ref:`Circuit breaking<arch_overview_circuit_break>` settings can be
/// specified individually for each defined priority.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CircuitBreakers {
    /// If multiple :ref:`Thresholds<envoy_api_msg_config.cluster.v3.CircuitBreakers.Thresholds>`
    /// are defined with the same :ref:`RoutingPriority<envoy_api_enum_config.core.v3.RoutingPriority>`,
    /// the first one in the list is used. If no Thresholds is defined for a given
    /// :ref:`RoutingPriority<envoy_api_enum_config.core.v3.RoutingPriority>`, the default values
    /// are used.
    #[prost(message, repeated, tag = "1")]
    pub thresholds: ::std::vec::Vec<circuit_breakers::Thresholds>,
}
pub mod circuit_breakers {
    /// A Thresholds defines CircuitBreaker settings for a
    /// :ref:`RoutingPriority<envoy_api_enum_config.core.v3.RoutingPriority>`.
    /// [#next-free-field: 9]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Thresholds {
        /// The :ref:`RoutingPriority<envoy_api_enum_config.core.v3.RoutingPriority>`
        /// the specified CircuitBreaker settings apply to.
        #[prost(
            enumeration = "super::super::super::core::v3::RoutingPriority",
            tag = "1"
        )]
        pub priority: i32,
        /// The maximum number of connections that Envoy will make to the upstream
        /// cluster. If not specified, the default is 1024.
        #[prost(message, optional, tag = "2")]
        pub max_connections: ::std::option::Option<u32>,
        /// The maximum number of pending requests that Envoy will allow to the
        /// upstream cluster. If not specified, the default is 1024.
        #[prost(message, optional, tag = "3")]
        pub max_pending_requests: ::std::option::Option<u32>,
        /// The maximum number of parallel requests that Envoy will make to the
        /// upstream cluster. If not specified, the default is 1024.
        #[prost(message, optional, tag = "4")]
        pub max_requests: ::std::option::Option<u32>,
        /// The maximum number of parallel retries that Envoy will allow to the
        /// upstream cluster. If not specified, the default is 3.
        #[prost(message, optional, tag = "5")]
        pub max_retries: ::std::option::Option<u32>,
        /// Specifies a limit on concurrent retries in relation to the number of active requests. This
        /// parameter is optional.
        ///
        /// .. note::
        ///
        ///    If this field is set, the retry budget will override any configured retry circuit
        ///    breaker.
        #[prost(message, optional, tag = "8")]
        pub retry_budget: ::std::option::Option<thresholds::RetryBudget>,
        /// If track_remaining is true, then stats will be published that expose
        /// the number of resources remaining until the circuit breakers open. If
        /// not specified, the default is false.
        ///
        /// .. note::
        ///
        ///    If a retry budget is used in lieu of the max_retries circuit breaker,
        ///    the remaining retry resources remaining will not be tracked.
        #[prost(bool, tag = "6")]
        pub track_remaining: bool,
        /// The maximum number of connection pools per cluster that Envoy will concurrently support at
        /// once. If not specified, the default is unlimited. Set this for clusters which create a
        /// large number of connection pools. See
        /// :ref:`Circuit Breaking <arch_overview_circuit_break_cluster_maximum_connection_pools>` for
        /// more details.
        #[prost(message, optional, tag = "7")]
        pub max_connection_pools: ::std::option::Option<u32>,
    }
    pub mod thresholds {
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct RetryBudget {
            /// Specifies the limit on concurrent retries as a percentage of the sum of active requests and
            /// active pending requests. For example, if there are 100 active requests and the
            /// budget_percent is set to 25, there may be 25 active retries.
            ///
            /// This parameter is optional. Defaults to 20%.
            #[prost(message, optional, tag = "1")]
            pub budget_percent:
                ::std::option::Option<super::super::super::super::super::r#type::v3::Percent>,
            /// Specifies the minimum retry concurrency allowed for the retry budget. The limit on the
            /// number of active retries may never go below this number.
            ///
            /// This parameter is optional. Defaults to 3.
            #[prost(message, optional, tag = "2")]
            pub min_retry_concurrency: ::std::option::Option<u32>,
        }
    }
}
// [#protodoc-title: Upstream filters]
// Upstream filters apply to the connections to the upstream cluster hosts.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// The name of the filter to instantiate. The name must match a
    /// :ref:`supported filter <config_network_filters>`.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// Filter specific configuration which depends on the filter being
    /// instantiated. See the supported filters for further documentation.
    #[prost(message, optional, tag = "2")]
    pub typed_config: ::std::option::Option<::prost_types::Any>,
}
// [#protodoc-title: Outlier detection]

/// See the :ref:`architecture overview <arch_overview_outlier_detection>` for
/// more information on outlier detection.
/// [#next-free-field: 21]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OutlierDetection {
    /// The number of consecutive 5xx responses or local origin errors that are mapped
    /// to 5xx error codes before a consecutive 5xx ejection
    /// occurs. Defaults to 5.
    #[prost(message, optional, tag = "1")]
    pub consecutive_5xx: ::std::option::Option<u32>,
    /// The time interval between ejection analysis sweeps. This can result in
    /// both new ejections as well as hosts being returned to service. Defaults
    /// to 10000ms or 10s.
    #[prost(message, optional, tag = "2")]
    pub interval: ::std::option::Option<::prost_types::Duration>,
    /// The base time that a host is ejected for. The real time is equal to the
    /// base time multiplied by the number of times the host has been ejected.
    /// Defaults to 30000ms or 30s.
    #[prost(message, optional, tag = "3")]
    pub base_ejection_time: ::std::option::Option<::prost_types::Duration>,
    /// The maximum % of an upstream cluster that can be ejected due to outlier
    /// detection. Defaults to 10% but will eject at least one host regardless of the value.
    #[prost(message, optional, tag = "4")]
    pub max_ejection_percent: ::std::option::Option<u32>,
    /// The % chance that a host will be actually ejected when an outlier status
    /// is detected through consecutive 5xx. This setting can be used to disable
    /// ejection or to ramp it up slowly. Defaults to 100.
    #[prost(message, optional, tag = "5")]
    pub enforcing_consecutive_5xx: ::std::option::Option<u32>,
    /// The % chance that a host will be actually ejected when an outlier status
    /// is detected through success rate statistics. This setting can be used to
    /// disable ejection or to ramp it up slowly. Defaults to 100.
    #[prost(message, optional, tag = "6")]
    pub enforcing_success_rate: ::std::option::Option<u32>,
    /// The number of hosts in a cluster that must have enough request volume to
    /// detect success rate outliers. If the number of hosts is less than this
    /// setting, outlier detection via success rate statistics is not performed
    /// for any host in the cluster. Defaults to 5.
    #[prost(message, optional, tag = "7")]
    pub success_rate_minimum_hosts: ::std::option::Option<u32>,
    /// The minimum number of total requests that must be collected in one
    /// interval (as defined by the interval duration above) to include this host
    /// in success rate based outlier detection. If the volume is lower than this
    /// setting, outlier detection via success rate statistics is not performed
    /// for that host. Defaults to 100.
    #[prost(message, optional, tag = "8")]
    pub success_rate_request_volume: ::std::option::Option<u32>,
    /// This factor is used to determine the ejection threshold for success rate
    /// outlier ejection. The ejection threshold is the difference between the
    /// mean success rate, and the product of this factor and the standard
    /// deviation of the mean success rate: mean - (stdev *
    /// success_rate_stdev_factor). This factor is divided by a thousand to get a
    /// double. That is, if the desired factor is 1.9, the runtime value should
    /// be 1900. Defaults to 1900.
    #[prost(message, optional, tag = "9")]
    pub success_rate_stdev_factor: ::std::option::Option<u32>,
    /// The number of consecutive gateway failures (502, 503, 504 status codes)
    /// before a consecutive gateway failure ejection occurs. Defaults to 5.
    #[prost(message, optional, tag = "10")]
    pub consecutive_gateway_failure: ::std::option::Option<u32>,
    /// The % chance that a host will be actually ejected when an outlier status
    /// is detected through consecutive gateway failures. This setting can be
    /// used to disable ejection or to ramp it up slowly. Defaults to 0.
    #[prost(message, optional, tag = "11")]
    pub enforcing_consecutive_gateway_failure: ::std::option::Option<u32>,
    /// Determines whether to distinguish local origin failures from external errors. If set to true
    /// the following configuration parameters are taken into account:
    /// :ref:`consecutive_local_origin_failure<envoy_api_field_config.cluster.v3.OutlierDetection.consecutive_local_origin_failure>`,
    /// :ref:`enforcing_consecutive_local_origin_failure<envoy_api_field_config.cluster.v3.OutlierDetection.enforcing_consecutive_local_origin_failure>`
    /// and
    /// :ref:`enforcing_local_origin_success_rate<envoy_api_field_config.cluster.v3.OutlierDetection.enforcing_local_origin_success_rate>`.
    /// Defaults to false.
    #[prost(bool, tag = "12")]
    pub split_external_local_origin_errors: bool,
    /// The number of consecutive locally originated failures before ejection
    /// occurs. Defaults to 5. Parameter takes effect only when
    /// :ref:`split_external_local_origin_errors<envoy_api_field_config.cluster.v3.OutlierDetection.split_external_local_origin_errors>`
    /// is set to true.
    #[prost(message, optional, tag = "13")]
    pub consecutive_local_origin_failure: ::std::option::Option<u32>,
    /// The % chance that a host will be actually ejected when an outlier status
    /// is detected through consecutive locally originated failures. This setting can be
    /// used to disable ejection or to ramp it up slowly. Defaults to 100.
    /// Parameter takes effect only when
    /// :ref:`split_external_local_origin_errors<envoy_api_field_config.cluster.v3.OutlierDetection.split_external_local_origin_errors>`
    /// is set to true.
    #[prost(message, optional, tag = "14")]
    pub enforcing_consecutive_local_origin_failure: ::std::option::Option<u32>,
    /// The % chance that a host will be actually ejected when an outlier status
    /// is detected through success rate statistics for locally originated errors.
    /// This setting can be used to disable ejection or to ramp it up slowly. Defaults to 100.
    /// Parameter takes effect only when
    /// :ref:`split_external_local_origin_errors<envoy_api_field_config.cluster.v3.OutlierDetection.split_external_local_origin_errors>`
    /// is set to true.
    #[prost(message, optional, tag = "15")]
    pub enforcing_local_origin_success_rate: ::std::option::Option<u32>,
    /// The failure percentage to use when determining failure percentage-based outlier detection. If
    /// the failure percentage of a given host is greater than or equal to this value, it will be
    /// ejected. Defaults to 85.
    #[prost(message, optional, tag = "16")]
    pub failure_percentage_threshold: ::std::option::Option<u32>,
    /// The % chance that a host will be actually ejected when an outlier status is detected through
    /// failure percentage statistics. This setting can be used to disable ejection or to ramp it up
    /// slowly. Defaults to 0.
    ///
    /// [#next-major-version: setting this without setting failure_percentage_threshold should be
    /// invalid in v4.]
    #[prost(message, optional, tag = "17")]
    pub enforcing_failure_percentage: ::std::option::Option<u32>,
    /// The % chance that a host will be actually ejected when an outlier status is detected through
    /// local-origin failure percentage statistics. This setting can be used to disable ejection or to
    /// ramp it up slowly. Defaults to 0.
    #[prost(message, optional, tag = "18")]
    pub enforcing_failure_percentage_local_origin: ::std::option::Option<u32>,
    /// The minimum number of hosts in a cluster in order to perform failure percentage-based ejection.
    /// If the total number of hosts in the cluster is less than this value, failure percentage-based
    /// ejection will not be performed. Defaults to 5.
    #[prost(message, optional, tag = "19")]
    pub failure_percentage_minimum_hosts: ::std::option::Option<u32>,
    /// The minimum number of total requests that must be collected in one interval (as defined by the
    /// interval duration above) to perform failure percentage-based ejection for this host. If the
    /// volume is lower than this setting, failure percentage-based ejection will not be performed for
    /// this host. Defaults to 50.
    #[prost(message, optional, tag = "20")]
    pub failure_percentage_request_volume: ::std::option::Option<u32>,
}
// [#protodoc-title: Cluster configuration]

/// Cluster list collections. Entries are *Cluster* resources or references.
/// [#not-implemented-hide:]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClusterCollection {
    #[prost(message, optional, tag = "1")]
    pub entries: ::std::option::Option<super::super::super::super::udpa::core::v1::CollectionEntry>,
}
/// Configuration for a single upstream cluster.
/// [#next-free-field: 51]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cluster {
    /// Configuration to use different transport sockets for different endpoints.
    /// The entry of *envoy.transport_socket_match* in the
    /// :ref:`LbEndpoint.Metadata <envoy_api_field_config.endpoint.v3.LbEndpoint.metadata>`
    /// is used to match against the transport sockets as they appear in the list. The first
    /// :ref:`match <envoy_api_msg_config.cluster.v3.Cluster.TransportSocketMatch>` is used.
    /// For example, with the following match
    ///
    /// .. code-block:: yaml
    ///
    ///  transport_socket_matches:
    ///  - name: "enableMTLS"
    ///    match:
    ///      acceptMTLS: true
    ///    transport_socket:
    ///      name: envoy.transport_sockets.tls
    ///      config: { ... } # tls socket configuration
    ///  - name: "defaultToPlaintext"
    ///    match: {}
    ///    transport_socket:
    ///      name: envoy.transport_sockets.raw_buffer
    ///
    /// Connections to the endpoints whose metadata value under *envoy.transport_socket_match*
    /// having "acceptMTLS"/"true" key/value pair use the "enableMTLS" socket configuration.
    ///
    /// If a :ref:`socket match <envoy_api_msg_config.cluster.v3.Cluster.TransportSocketMatch>` with empty match
    /// criteria is provided, that always match any endpoint. For example, the "defaultToPlaintext"
    /// socket match in case above.
    ///
    /// If an endpoint metadata's value under *envoy.transport_socket_match* does not match any
    /// *TransportSocketMatch*, socket configuration fallbacks to use the *tls_context* or
    /// *transport_socket* specified in this cluster.
    ///
    /// This field allows gradual and flexible transport socket configuration changes.
    ///
    /// The metadata of endpoints in EDS can indicate transport socket capabilities. For example,
    /// an endpoint's metadata can have two key value pairs as "acceptMTLS": "true",
    /// "acceptPlaintext": "true". While some other endpoints, only accepting plaintext traffic
    /// has "acceptPlaintext": "true" metadata information.
    ///
    /// Then the xDS server can configure the CDS to a client, Envoy A, to send mutual TLS
    /// traffic for endpoints with "acceptMTLS": "true", by adding a corresponding
    /// *TransportSocketMatch* in this field. Other client Envoys receive CDS without
    /// *transport_socket_match* set, and still send plain text traffic to the same cluster.
    ///
    /// This field can be used to specify custom transport socket configurations for health
    /// checks by adding matching key/value pairs in a health check's
    /// :ref:`transport socket match criteria <envoy_api_field_config.core.v3.HealthCheck.transport_socket_match_criteria>` field.
    ///
    /// [#comment:TODO(incfly): add a detailed architecture doc on intended usage.]
    #[prost(message, repeated, tag = "43")]
    pub transport_socket_matches: ::std::vec::Vec<cluster::TransportSocketMatch>,
    /// Supplies the name of the cluster which must be unique across all clusters.
    /// The cluster name is used when emitting
    /// :ref:`statistics <config_cluster_manager_cluster_stats>` if :ref:`alt_stat_name
    /// <envoy_api_field_config.cluster.v3.Cluster.alt_stat_name>` is not provided.
    /// Any ``:`` in the cluster name will be converted to ``_`` when emitting statistics.
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    /// An optional alternative to the cluster name to be used while emitting stats.
    /// Any ``:`` in the name will be converted to ``_`` when emitting statistics. This should not be
    /// confused with :ref:`Router Filter Header
    /// <config_http_filters_router_x-envoy-upstream-alt-stat-name>`.
    #[prost(string, tag = "28")]
    pub alt_stat_name: std::string::String,
    /// Configuration to use for EDS updates for the Cluster.
    #[prost(message, optional, tag = "3")]
    pub eds_cluster_config: ::std::option::Option<cluster::EdsClusterConfig>,
    /// The timeout for new network connections to hosts in the cluster.
    #[prost(message, optional, tag = "4")]
    pub connect_timeout: ::std::option::Option<::prost_types::Duration>,
    /// Soft limit on size of the cluster’s connections read and write buffers. If
    /// unspecified, an implementation defined default is applied (1MiB).
    #[prost(message, optional, tag = "5")]
    pub per_connection_buffer_limit_bytes: ::std::option::Option<u32>,
    /// The :ref:`load balancer type <arch_overview_load_balancing_types>` to use
    /// when picking a host in the cluster.
    /// [#comment:TODO: Remove enum constraint :ref:`LOAD_BALANCING_POLICY_CONFIG<envoy_api_enum_value_config.cluster.v3.Cluster.LbPolicy.LOAD_BALANCING_POLICY_CONFIG>` when implemented.]
    #[prost(enumeration = "cluster::LbPolicy", tag = "6")]
    pub lb_policy: i32,
    /// Setting this is required for specifying members of
    /// :ref:`STATIC<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.STATIC>`,
    /// :ref:`STRICT_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.STRICT_DNS>`
    /// or :ref:`LOGICAL_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.LOGICAL_DNS>` clusters.
    /// This field supersedes the *hosts* field in the v2 API.
    ///
    /// .. attention::
    ///
    ///   Setting this allows non-EDS cluster types to contain embedded EDS equivalent
    ///   :ref:`endpoint assignments<envoy_api_msg_config.endpoint.v3.ClusterLoadAssignment>`.
    ///
    #[prost(message, optional, tag = "33")]
    pub load_assignment: ::std::option::Option<super::super::endpoint::v3::ClusterLoadAssignment>,
    /// Optional :ref:`active health checking <arch_overview_health_checking>`
    /// configuration for the cluster. If no
    /// configuration is specified no health checking will be done and all cluster
    /// members will be considered healthy at all times.
    #[prost(message, repeated, tag = "8")]
    pub health_checks: ::std::vec::Vec<super::super::core::v3::HealthCheck>,
    /// Optional maximum requests for a single upstream connection. This parameter
    /// is respected by both the HTTP/1.1 and HTTP/2 connection pool
    /// implementations. If not specified, there is no limit. Setting this
    /// parameter to 1 will effectively disable keep alive.
    #[prost(message, optional, tag = "9")]
    pub max_requests_per_connection: ::std::option::Option<u32>,
    /// Optional :ref:`circuit breaking <arch_overview_circuit_break>` for the cluster.
    #[prost(message, optional, tag = "10")]
    pub circuit_breakers: ::std::option::Option<CircuitBreakers>,
    /// HTTP protocol options that are applied only to upstream HTTP connections.
    /// These options apply to all HTTP versions.
    #[prost(message, optional, tag = "46")]
    pub upstream_http_protocol_options:
        ::std::option::Option<super::super::core::v3::UpstreamHttpProtocolOptions>,
    /// Additional options when handling HTTP requests upstream. These options will be applicable to
    /// both HTTP1 and HTTP2 requests.
    #[prost(message, optional, tag = "29")]
    pub common_http_protocol_options:
        ::std::option::Option<super::super::core::v3::HttpProtocolOptions>,
    /// Additional options when handling HTTP1 requests.
    #[prost(message, optional, tag = "13")]
    pub http_protocol_options: ::std::option::Option<super::super::core::v3::Http1ProtocolOptions>,
    /// Even if default HTTP2 protocol options are desired, this field must be
    /// set so that Envoy will assume that the upstream supports HTTP/2 when
    /// making new HTTP connection pool connections. Currently, Envoy only
    /// supports prior knowledge for upstream connections. Even if TLS is used
    /// with ALPN, `http2_protocol_options` must be specified. As an aside this allows HTTP/2
    /// connections to happen over plain text.
    #[prost(message, optional, tag = "14")]
    pub http2_protocol_options: ::std::option::Option<super::super::core::v3::Http2ProtocolOptions>,
    /// The extension_protocol_options field is used to provide extension-specific protocol options
    /// for upstream connections. The key should match the extension filter name, such as
    /// "envoy.filters.network.thrift_proxy". See the extension's documentation for details on
    /// specific options.
    #[prost(map = "string, message", tag = "36")]
    pub typed_extension_protocol_options:
        ::std::collections::HashMap<std::string::String, ::prost_types::Any>,
    /// If the DNS refresh rate is specified and the cluster type is either
    /// :ref:`STRICT_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.STRICT_DNS>`,
    /// or :ref:`LOGICAL_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.LOGICAL_DNS>`,
    /// this value is used as the cluster’s DNS refresh
    /// rate. The value configured must be at least 1ms. If this setting is not specified, the
    /// value defaults to 5000ms. For cluster types other than
    /// :ref:`STRICT_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.STRICT_DNS>`
    /// and :ref:`LOGICAL_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.LOGICAL_DNS>`
    /// this setting is ignored.
    #[prost(message, optional, tag = "16")]
    pub dns_refresh_rate: ::std::option::Option<::prost_types::Duration>,
    /// If the DNS failure refresh rate is specified and the cluster type is either
    /// :ref:`STRICT_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.STRICT_DNS>`,
    /// or :ref:`LOGICAL_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.LOGICAL_DNS>`,
    /// this is used as the cluster’s DNS refresh rate when requests are failing. If this setting is
    /// not specified, the failure refresh rate defaults to the DNS refresh rate. For cluster types
    /// other than :ref:`STRICT_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.STRICT_DNS>` and
    /// :ref:`LOGICAL_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.LOGICAL_DNS>` this setting is
    /// ignored.
    #[prost(message, optional, tag = "44")]
    pub dns_failure_refresh_rate: ::std::option::Option<cluster::RefreshRate>,
    /// Optional configuration for setting cluster's DNS refresh rate. If the value is set to true,
    /// cluster's DNS refresh rate will be set to resource record's TTL which comes from DNS
    /// resolution.
    #[prost(bool, tag = "39")]
    pub respect_dns_ttl: bool,
    /// The DNS IP address resolution policy. If this setting is not specified, the
    /// value defaults to
    /// :ref:`AUTO<envoy_api_enum_value_config.cluster.v3.Cluster.DnsLookupFamily.AUTO>`.
    #[prost(enumeration = "cluster::DnsLookupFamily", tag = "17")]
    pub dns_lookup_family: i32,
    /// If DNS resolvers are specified and the cluster type is either
    /// :ref:`STRICT_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.STRICT_DNS>`,
    /// or :ref:`LOGICAL_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.LOGICAL_DNS>`,
    /// this value is used to specify the cluster’s dns resolvers.
    /// If this setting is not specified, the value defaults to the default
    /// resolver, which uses /etc/resolv.conf for configuration. For cluster types
    /// other than
    /// :ref:`STRICT_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.STRICT_DNS>`
    /// and :ref:`LOGICAL_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.LOGICAL_DNS>`
    /// this setting is ignored.
    #[prost(message, repeated, tag = "18")]
    pub dns_resolvers: ::std::vec::Vec<super::super::core::v3::Address>,
    /// [#next-major-version: Reconcile DNS options in a single message.]
    /// Always use TCP queries instead of UDP queries for DNS lookups.
    #[prost(bool, tag = "45")]
    pub use_tcp_for_dns_lookups: bool,
    /// If specified, outlier detection will be enabled for this upstream cluster.
    /// Each of the configuration values can be overridden via
    /// :ref:`runtime values <config_cluster_manager_cluster_runtime_outlier_detection>`.
    #[prost(message, optional, tag = "19")]
    pub outlier_detection: ::std::option::Option<OutlierDetection>,
    /// The interval for removing stale hosts from a cluster type
    /// :ref:`ORIGINAL_DST<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.ORIGINAL_DST>`.
    /// Hosts are considered stale if they have not been used
    /// as upstream destinations during this interval. New hosts are added
    /// to original destination clusters on demand as new connections are
    /// redirected to Envoy, causing the number of hosts in the cluster to
    /// grow over time. Hosts that are not stale (they are actively used as
    /// destinations) are kept in the cluster, which allows connections to
    /// them remain open, saving the latency that would otherwise be spent
    /// on opening new connections. If this setting is not specified, the
    /// value defaults to 5000ms. For cluster types other than
    /// :ref:`ORIGINAL_DST<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.ORIGINAL_DST>`
    /// this setting is ignored.
    #[prost(message, optional, tag = "20")]
    pub cleanup_interval: ::std::option::Option<::prost_types::Duration>,
    /// Optional configuration used to bind newly established upstream connections.
    /// This overrides any bind_config specified in the bootstrap proto.
    /// If the address and port are empty, no bind will be performed.
    #[prost(message, optional, tag = "21")]
    pub upstream_bind_config: ::std::option::Option<super::super::core::v3::BindConfig>,
    /// Configuration for load balancing subsetting.
    #[prost(message, optional, tag = "22")]
    pub lb_subset_config: ::std::option::Option<cluster::LbSubsetConfig>,
    /// Common configuration for all load balancer implementations.
    #[prost(message, optional, tag = "27")]
    pub common_lb_config: ::std::option::Option<cluster::CommonLbConfig>,
    /// Optional custom transport socket implementation to use for upstream connections.
    /// To setup TLS, set a transport socket with name `tls` and
    /// :ref:`UpstreamTlsContexts <envoy_api_msg_extensions.transport_sockets.tls.v3.UpstreamTlsContext>` in the `typed_config`.
    /// If no transport socket configuration is specified, new connections
    /// will be set up with plaintext.
    #[prost(message, optional, tag = "24")]
    pub transport_socket: ::std::option::Option<super::super::core::v3::TransportSocket>,
    /// The Metadata field can be used to provide additional information about the
    /// cluster. It can be used for stats, logging, and varying filter behavior.
    /// Fields should use reverse DNS notation to denote which entity within Envoy
    /// will need the information. For instance, if the metadata is intended for
    /// the Router filter, the filter name should be specified as *envoy.filters.http.router*.
    #[prost(message, optional, tag = "25")]
    pub metadata: ::std::option::Option<super::super::core::v3::Metadata>,
    /// Determines how Envoy selects the protocol used to speak to upstream hosts.
    #[prost(enumeration = "cluster::ClusterProtocolSelection", tag = "26")]
    pub protocol_selection: i32,
    /// Optional options for upstream connections.
    #[prost(message, optional, tag = "30")]
    pub upstream_connection_options: ::std::option::Option<UpstreamConnectionOptions>,
    /// If an upstream host becomes unhealthy (as determined by the configured health checks
    /// or outlier detection), immediately close all connections to the failed host.
    ///
    /// .. note::
    ///
    ///   This is currently only supported for connections created by tcp_proxy.
    ///
    /// .. note::
    ///
    ///   The current implementation of this feature closes all connections immediately when
    ///   the unhealthy status is detected. If there are a large number of connections open
    ///   to an upstream host that becomes unhealthy, Envoy may spend a substantial amount of
    ///   time exclusively closing these connections, and not processing any other traffic.
    #[prost(bool, tag = "31")]
    pub close_connections_on_host_health_failure: bool,
    /// If set to true, Envoy will ignore the health value of a host when processing its removal
    /// from service discovery. This means that if active health checking is used, Envoy will *not*
    /// wait for the endpoint to go unhealthy before removing it.
    #[prost(bool, tag = "32")]
    pub ignore_health_on_host_removal: bool,
    /// An (optional) network filter chain, listed in the order the filters should be applied.
    /// The chain will be applied to all outgoing connections that Envoy makes to the upstream
    /// servers of this cluster.
    #[prost(message, repeated, tag = "40")]
    pub filters: ::std::vec::Vec<Filter>,
    /// [#not-implemented-hide:] New mechanism for LB policy configuration. Used only if the
    /// :ref:`lb_policy<envoy_api_field_config.cluster.v3.Cluster.lb_policy>` field has the value
    /// :ref:`LOAD_BALANCING_POLICY_CONFIG<envoy_api_enum_value_config.cluster.v3.Cluster.LbPolicy.LOAD_BALANCING_POLICY_CONFIG>`.
    #[prost(message, optional, tag = "41")]
    pub load_balancing_policy: ::std::option::Option<LoadBalancingPolicy>,
    /// [#not-implemented-hide:]
    /// If present, tells the client where to send load reports via LRS. If not present, the
    /// client will fall back to a client-side default, which may be either (a) don't send any
    /// load reports or (b) send load reports for all clusters to a single default server
    /// (which may be configured in the bootstrap file).
    ///
    /// Note that if multiple clusters point to the same LRS server, the client may choose to
    /// create a separate stream for each cluster or it may choose to coalesce the data for
    /// multiple clusters onto a single stream. Either way, the client must make sure to send
    /// the data for any given cluster on no more than one stream.
    ///
    /// [#next-major-version: In the v3 API, we should consider restructuring this somehow,
    /// maybe by allowing LRS to go on the ADS stream, or maybe by moving some of the negotiation
    /// from the LRS stream here.]
    #[prost(message, optional, tag = "42")]
    pub lrs_server: ::std::option::Option<super::super::core::v3::ConfigSource>,
    /// If track_timeout_budgets is true, the :ref:`timeout budget histograms
    /// <config_cluster_manager_cluster_stats_timeout_budgets>` will be published for each
    /// request. These show what percentage of a request's per try and global timeout was used. A value
    /// of 0 would indicate that none of the timeout was used or that the timeout was infinite. A value
    /// of 100 would indicate that the request took the entirety of the timeout given to it.
    ///
    /// .. attention::
    ///
    ///   This field has been deprecated in favor of `timeout_budgets`, part of
    ///   :ref:`track_cluster_stats <envoy_api_field_config.cluster.v3.Cluster.track_cluster_stats>`.
    #[prost(bool, tag = "47")]
    pub track_timeout_budgets: bool,
    /// Optional customization and configuration of upstream connection pool, and upstream type.
    ///
    /// Currently this field only applies for HTTP traffic but is designed for eventual use for custom
    /// TCP upstreams.
    ///
    /// For HTTP traffic, Envoy will generally take downstream HTTP and send it upstream as upstream
    /// HTTP, using the http connection pool and the codec from `http2_protocol_options`
    ///
    /// For routes where CONNECT termination is configured, Envoy will take downstream CONNECT
    /// requests and forward the CONNECT payload upstream over raw TCP using the tcp connection pool.
    ///
    /// The default pool used is the generic connection pool which creates the HTTP upstream for most
    /// HTTP requests, and the TCP upstream if CONNECT termination is configured.
    ///
    /// If users desire custom connection pool or upstream behavior, for example terminating
    /// CONNECT only if a custom filter indicates it is appropriate, the custom factories
    /// can be registered and configured here.
    #[prost(message, optional, tag = "48")]
    pub upstream_config: ::std::option::Option<super::super::core::v3::TypedExtensionConfig>,
    /// Configuration to track optional cluster stats.
    #[prost(message, optional, tag = "49")]
    pub track_cluster_stats: ::std::option::Option<TrackClusterStats>,
    /// [#not-implemented-hide:]
    /// Prefetch configuration for this cluster.
    #[prost(message, optional, tag = "50")]
    pub prefetch_policy: ::std::option::Option<cluster::PrefetchPolicy>,
    #[prost(oneof = "cluster::ClusterDiscoveryType", tags = "2, 38")]
    pub cluster_discovery_type: ::std::option::Option<cluster::ClusterDiscoveryType>,
    /// Optional configuration for the load balancing algorithm selected by
    /// LbPolicy. Currently only
    /// :ref:`RING_HASH<envoy_api_enum_value_config.cluster.v3.Cluster.LbPolicy.RING_HASH>` and
    /// :ref:`LEAST_REQUEST<envoy_api_enum_value_config.cluster.v3.Cluster.LbPolicy.LEAST_REQUEST>`
    /// has additional configuration options.
    /// Specifying ring_hash_lb_config or least_request_lb_config without setting the corresponding
    /// LbPolicy will generate an error at runtime.
    #[prost(oneof = "cluster::LbConfig", tags = "23, 34, 37")]
    pub lb_config: ::std::option::Option<cluster::LbConfig>,
}
pub mod cluster {
    /// TransportSocketMatch specifies what transport socket config will be used
    /// when the match conditions are satisfied.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct TransportSocketMatch {
        /// The name of the match, used in stats generation.
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        /// Optional endpoint metadata match criteria.
        /// The connection to the endpoint with metadata matching what is set in this field
        /// will use the transport socket configuration specified here.
        /// The endpoint's metadata entry in *envoy.transport_socket_match* is used to match
        /// against the values specified in this field.
        #[prost(message, optional, tag = "2")]
        pub r#match: ::std::option::Option<::prost_types::Struct>,
        /// The configuration of the transport socket.
        #[prost(message, optional, tag = "3")]
        pub transport_socket: ::std::option::Option<super::super::super::core::v3::TransportSocket>,
    }
    /// Extended cluster type.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CustomClusterType {
        /// The type of the cluster to instantiate. The name must match a supported cluster type.
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        /// Cluster specific configuration which depends on the cluster being instantiated.
        /// See the supported cluster for further documentation.
        #[prost(message, optional, tag = "2")]
        pub typed_config: ::std::option::Option<::prost_types::Any>,
    }
    /// Only valid when discovery type is EDS.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct EdsClusterConfig {
        /// Configuration for the source of EDS updates for this Cluster.
        #[prost(message, optional, tag = "1")]
        pub eds_config: ::std::option::Option<super::super::super::core::v3::ConfigSource>,
        /// Optional alternative to cluster name to present to EDS. This does not
        /// have the same restrictions as cluster name, i.e. it may be arbitrary
        /// length.
        #[prost(string, tag = "2")]
        pub service_name: std::string::String,
        /// Resource locator for EDS. This is mutually exclusive to *service_name*.
        /// [#not-implemented-hide:]
        #[prost(message, optional, tag = "3")]
        pub eds_resource_locator: ::std::option::Option<
            super::super::super::super::super::udpa::core::v1::ResourceLocator,
        >,
    }
    /// Optionally divide the endpoints in this cluster into subsets defined by
    /// endpoint metadata and selected by route and weighted cluster metadata.
    /// [#next-free-field: 8]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LbSubsetConfig {
        /// The behavior used when no endpoint subset matches the selected route's
        /// metadata. The value defaults to
        /// :ref:`NO_FALLBACK<envoy_api_enum_value_config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetFallbackPolicy.NO_FALLBACK>`.
        #[prost(enumeration = "lb_subset_config::LbSubsetFallbackPolicy", tag = "1")]
        pub fallback_policy: i32,
        /// Specifies the default subset of endpoints used during fallback if
        /// fallback_policy is
        /// :ref:`DEFAULT_SUBSET<envoy_api_enum_value_config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetFallbackPolicy.DEFAULT_SUBSET>`.
        /// Each field in default_subset is
        /// compared to the matching LbEndpoint.Metadata under the *envoy.lb*
        /// namespace. It is valid for no hosts to match, in which case the behavior
        /// is the same as a fallback_policy of
        /// :ref:`NO_FALLBACK<envoy_api_enum_value_config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetFallbackPolicy.NO_FALLBACK>`.
        #[prost(message, optional, tag = "2")]
        pub default_subset: ::std::option::Option<::prost_types::Struct>,
        /// For each entry, LbEndpoint.Metadata's
        /// *envoy.lb* namespace is traversed and a subset is created for each unique
        /// combination of key and value. For example:
        ///
        /// .. code-block:: json
        ///
        ///   { "subset_selectors": [
        ///       { "keys": [ "version" ] },
        ///       { "keys": [ "stage", "hardware_type" ] }
        ///   ]}
        ///
        /// A subset is matched when the metadata from the selected route and
        /// weighted cluster contains the same keys and values as the subset's
        /// metadata. The same host may appear in multiple subsets.
        #[prost(message, repeated, tag = "3")]
        pub subset_selectors: ::std::vec::Vec<lb_subset_config::LbSubsetSelector>,
        /// If true, routing to subsets will take into account the localities and locality weights of the
        /// endpoints when making the routing decision.
        ///
        /// There are some potential pitfalls associated with enabling this feature, as the resulting
        /// traffic split after applying both a subset match and locality weights might be undesirable.
        ///
        /// Consider for example a situation in which you have 50/50 split across two localities X/Y
        /// which have 100 hosts each without subsetting. If the subset LB results in X having only 1
        /// host selected but Y having 100, then a lot more load is being dumped on the single host in X
        /// than originally anticipated in the load balancing assignment delivered via EDS.
        #[prost(bool, tag = "4")]
        pub locality_weight_aware: bool,
        /// When used with locality_weight_aware, scales the weight of each locality by the ratio
        /// of hosts in the subset vs hosts in the original subset. This aims to even out the load
        /// going to an individual locality if said locality is disproportionately affected by the
        /// subset predicate.
        #[prost(bool, tag = "5")]
        pub scale_locality_weight: bool,
        /// If true, when a fallback policy is configured and its corresponding subset fails to find
        /// a host this will cause any host to be selected instead.
        ///
        /// This is useful when using the default subset as the fallback policy, given the default
        /// subset might become empty. With this option enabled, if that happens the LB will attempt
        /// to select a host from the entire cluster.
        #[prost(bool, tag = "6")]
        pub panic_mode_any: bool,
        /// If true, metadata specified for a metadata key will be matched against the corresponding
        /// endpoint metadata if the endpoint metadata matches the value exactly OR it is a list value
        /// and any of the elements in the list matches the criteria.
        #[prost(bool, tag = "7")]
        pub list_as_any: bool,
    }
    pub mod lb_subset_config {
        /// Specifications for subsets.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LbSubsetSelector {
            /// List of keys to match with the weighted cluster metadata.
            #[prost(string, repeated, tag = "1")]
            pub keys: ::std::vec::Vec<std::string::String>,
            /// The behavior used when no endpoint subset matches the selected route's
            /// metadata.
            #[prost(
                enumeration = "lb_subset_selector::LbSubsetSelectorFallbackPolicy",
                tag = "2"
            )]
            pub fallback_policy: i32,
            /// Subset of
            /// :ref:`keys<envoy_api_field_config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetSelector.keys>` used by
            /// :ref:`KEYS_SUBSET<envoy_api_enum_value_config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetSelector.LbSubsetSelectorFallbackPolicy.KEYS_SUBSET>`
            /// fallback policy.
            /// It has to be a non empty list if KEYS_SUBSET fallback policy is selected.
            /// For any other fallback policy the parameter is not used and should not be set.
            /// Only values also present in
            /// :ref:`keys<envoy_api_field_config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetSelector.keys>` are allowed, but
            /// `fallback_keys_subset` cannot be equal to `keys`.
            #[prost(string, repeated, tag = "3")]
            pub fallback_keys_subset: ::std::vec::Vec<std::string::String>,
        }
        pub mod lb_subset_selector {
            /// Allows to override top level fallback policy per selector.
            #[derive(
                Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
            )]
            #[repr(i32)]
            pub enum LbSubsetSelectorFallbackPolicy {
                /// If NOT_DEFINED top level config fallback policy is used instead.
                NotDefined = 0,
                /// If NO_FALLBACK is selected, a result equivalent to no healthy hosts is reported.
                NoFallback = 1,
                /// If ANY_ENDPOINT is selected, any cluster endpoint may be returned
                /// (subject to policy, health checks, etc).
                AnyEndpoint = 2,
                /// If DEFAULT_SUBSET is selected, load balancing is performed over the
                /// endpoints matching the values from the default_subset field.
                DefaultSubset = 3,
                /// If KEYS_SUBSET is selected, subset selector matching is performed again with metadata
                /// keys reduced to
                /// :ref:`fallback_keys_subset<envoy_api_field_config.cluster.v3.Cluster.LbSubsetConfig.LbSubsetSelector.fallback_keys_subset>`.
                /// It allows for a fallback to a different, less specific selector if some of the keys of
                /// the selector are considered optional.
                KeysSubset = 4,
            }
        }
        /// If NO_FALLBACK is selected, a result
        /// equivalent to no healthy hosts is reported. If ANY_ENDPOINT is selected,
        /// any cluster endpoint may be returned (subject to policy, health checks,
        /// etc). If DEFAULT_SUBSET is selected, load balancing is performed over the
        /// endpoints matching the values from the default_subset field.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum LbSubsetFallbackPolicy {
            NoFallback = 0,
            AnyEndpoint = 1,
            DefaultSubset = 2,
        }
    }
    /// Specific configuration for the LeastRequest load balancing policy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LeastRequestLbConfig {
        /// The number of random healthy hosts from which the host with the fewest active requests will
        /// be chosen. Defaults to 2 so that we perform two-choice selection if the field is not set.
        #[prost(message, optional, tag = "1")]
        pub choice_count: ::std::option::Option<u32>,
        /// The following formula is used to calculate the dynamic weights when hosts have different load
        /// balancing weights:
        ///
        /// `weight = load_balancing_weight / (active_requests + 1)^active_request_bias`
        ///
        /// The larger the active request bias is, the more aggressively active requests will lower the
        /// effective weight when all host weights are not equal.
        ///
        /// `active_request_bias` must be greater than or equal to 0.0.
        ///
        /// When `active_request_bias == 0.0` the Least Request Load Balancer doesn't consider the number
        /// of active requests at the time it picks a host and behaves like the Round Robin Load
        /// Balancer.
        ///
        /// When `active_request_bias > 0.0` the Least Request Load Balancer scales the load balancing
        /// weight by the number of active requests at the time it does a pick.
        ///
        /// The value is cached for performance reasons and refreshed whenever one of the Load Balancer's
        /// host sets changes, e.g., whenever there is a host membership update or a host load balancing
        /// weight change.
        ///
        /// .. note::
        ///   This setting only takes effect if all host weights are not equal.
        #[prost(message, optional, tag = "2")]
        pub active_request_bias:
            ::std::option::Option<super::super::super::core::v3::RuntimeDouble>,
    }
    /// Specific configuration for the :ref:`RingHash<arch_overview_load_balancing_types_ring_hash>`
    /// load balancing policy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RingHashLbConfig {
        /// Minimum hash ring size. The larger the ring is (that is, the more hashes there are for each
        /// provided host) the better the request distribution will reflect the desired weights. Defaults
        /// to 1024 entries, and limited to 8M entries. See also
        /// :ref:`maximum_ring_size<envoy_api_field_config.cluster.v3.Cluster.RingHashLbConfig.maximum_ring_size>`.
        #[prost(message, optional, tag = "1")]
        pub minimum_ring_size: ::std::option::Option<u64>,
        /// The hash function used to hash hosts onto the ketama ring. The value defaults to
        /// :ref:`XX_HASH<envoy_api_enum_value_config.cluster.v3.Cluster.RingHashLbConfig.HashFunction.XX_HASH>`.
        #[prost(enumeration = "ring_hash_lb_config::HashFunction", tag = "3")]
        pub hash_function: i32,
        /// Maximum hash ring size. Defaults to 8M entries, and limited to 8M entries, but can be lowered
        /// to further constrain resource use. See also
        /// :ref:`minimum_ring_size<envoy_api_field_config.cluster.v3.Cluster.RingHashLbConfig.minimum_ring_size>`.
        #[prost(message, optional, tag = "4")]
        pub maximum_ring_size: ::std::option::Option<u64>,
    }
    pub mod ring_hash_lb_config {
        /// The hash function used to hash hosts onto the ketama ring.
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
        #[repr(i32)]
        pub enum HashFunction {
            /// Use `xxHash <https://github.com/Cyan4973/xxHash>`_, this is the default hash function.
            XxHash = 0,
            /// Use `MurmurHash2 <https://sites.google.com/site/murmurhash/>`_, this is compatible with
            /// std:hash<string> in GNU libstdc++ 3.4.20 or above. This is typically the case when compiled
            /// on Linux and not macOS.
            MurmurHash2 = 1,
        }
    }
    /// Specific configuration for the
    /// :ref:`Original Destination <arch_overview_load_balancing_types_original_destination>`
    /// load balancing policy.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct OriginalDstLbConfig {
        /// When true, :ref:`x-envoy-original-dst-host
        /// <config_http_conn_man_headers_x-envoy-original-dst-host>` can be used to override destination
        /// address.
        ///
        /// .. attention::
        ///
        ///   This header isn't sanitized by default, so enabling this feature allows HTTP clients to
        ///   route traffic to arbitrary hosts and/or ports, which may have serious security
        ///   consequences.
        #[prost(bool, tag = "1")]
        pub use_http_header: bool,
    }
    /// Common configuration for all load balancer implementations.
    /// [#next-free-field: 8]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CommonLbConfig {
        /// Configures the :ref:`healthy panic threshold <arch_overview_load_balancing_panic_threshold>`.
        /// If not specified, the default is 50%.
        /// To disable panic mode, set to 0%.
        ///
        /// .. note::
        ///   The specified percent will be truncated to the nearest 1%.
        #[prost(message, optional, tag = "1")]
        pub healthy_panic_threshold:
            ::std::option::Option<super::super::super::super::r#type::v3::Percent>,
        /// If set, all health check/weight/metadata updates that happen within this duration will be
        /// merged and delivered in one shot when the duration expires. The start of the duration is when
        /// the first update happens. This is useful for big clusters, with potentially noisy deploys
        /// that might trigger excessive CPU usage due to a constant stream of healthcheck state changes
        /// or metadata updates. The first set of updates to be seen apply immediately (e.g.: a new
        /// cluster). Please always keep in mind that the use of sandbox technologies may change this
        /// behavior.
        ///
        /// If this is not set, we default to a merge window of 1000ms. To disable it, set the merge
        /// window to 0.
        ///
        /// Note: merging does not apply to cluster membership changes (e.g.: adds/removes); this is
        /// because merging those updates isn't currently safe. See
        /// https://github.com/envoyproxy/envoy/pull/3941.
        #[prost(message, optional, tag = "4")]
        pub update_merge_window: ::std::option::Option<::prost_types::Duration>,
        /// If set to true, Envoy will not consider new hosts when computing load balancing weights until
        /// they have been health checked for the first time. This will have no effect unless
        /// active health checking is also configured.
        ///
        /// Ignoring a host means that for any load balancing calculations that adjust weights based
        /// on the ratio of eligible hosts and total hosts (priority spillover, locality weighting and
        /// panic mode) Envoy will exclude these hosts in the denominator.
        ///
        /// For example, with hosts in two priorities P0 and P1, where P0 looks like
        /// {healthy, unhealthy (new), unhealthy (new)}
        /// and where P1 looks like
        /// {healthy, healthy}
        /// all traffic will still hit P0, as 1 / (3 - 2) = 1.
        ///
        /// Enabling this will allow scaling up the number of hosts for a given cluster without entering
        /// panic mode or triggering priority spillover, assuming the hosts pass the first health check.
        ///
        /// If panic mode is triggered, new hosts are still eligible for traffic; they simply do not
        /// contribute to the calculation when deciding whether panic mode is enabled or not.
        #[prost(bool, tag = "5")]
        pub ignore_new_hosts_until_first_hc: bool,
        /// If set to `true`, the cluster manager will drain all existing
        /// connections to upstream hosts whenever hosts are added or removed from the cluster.
        #[prost(bool, tag = "6")]
        pub close_connections_on_host_set_change: bool,
        ///Common Configuration for all consistent hashing load balancers (MaglevLb, RingHashLb, etc.)
        #[prost(message, optional, tag = "7")]
        pub consistent_hashing_lb_config:
            ::std::option::Option<common_lb_config::ConsistentHashingLbConfig>,
        #[prost(oneof = "common_lb_config::LocalityConfigSpecifier", tags = "2, 3")]
        pub locality_config_specifier:
            ::std::option::Option<common_lb_config::LocalityConfigSpecifier>,
    }
    pub mod common_lb_config {
        /// Configuration for :ref:`zone aware routing
        /// <arch_overview_load_balancing_zone_aware_routing>`.
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ZoneAwareLbConfig {
            /// Configures percentage of requests that will be considered for zone aware routing
            /// if zone aware routing is configured. If not specified, the default is 100%.
            /// * :ref:`runtime values <config_cluster_manager_cluster_runtime_zone_routing>`.
            /// * :ref:`Zone aware routing support <arch_overview_load_balancing_zone_aware_routing>`.
            #[prost(message, optional, tag = "1")]
            pub routing_enabled:
                ::std::option::Option<super::super::super::super::super::r#type::v3::Percent>,
            /// Configures minimum upstream cluster size required for zone aware routing
            /// If upstream cluster size is less than specified, zone aware routing is not performed
            /// even if zone aware routing is configured. If not specified, the default is 6.
            /// * :ref:`runtime values <config_cluster_manager_cluster_runtime_zone_routing>`.
            /// * :ref:`Zone aware routing support <arch_overview_load_balancing_zone_aware_routing>`.
            #[prost(message, optional, tag = "2")]
            pub min_cluster_size: ::std::option::Option<u64>,
            /// If set to true, Envoy will not consider any hosts when the cluster is in :ref:`panic
            /// mode<arch_overview_load_balancing_panic_threshold>`. Instead, the cluster will fail all
            /// requests as if all hosts are unhealthy. This can help avoid potentially overwhelming a
            /// failing service.
            #[prost(bool, tag = "3")]
            pub fail_traffic_on_panic: bool,
        }
        /// Configuration for :ref:`locality weighted load balancing
        /// <arch_overview_load_balancing_locality_weighted_lb>`
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct LocalityWeightedLbConfig {}
        /// Common Configuration for all consistent hashing load balancers (MaglevLb, RingHashLb, etc.)
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct ConsistentHashingLbConfig {
            /// If set to `true`, the cluster will use hostname instead of the resolved
            /// address as the key to consistently hash to an upstream host. Only valid for StrictDNS clusters with hostnames which resolve to a single IP address.
            #[prost(bool, tag = "1")]
            pub use_hostname_for_hashing: bool,
        }
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum LocalityConfigSpecifier {
            #[prost(message, tag = "2")]
            ZoneAwareLbConfig(ZoneAwareLbConfig),
            #[prost(message, tag = "3")]
            LocalityWeightedLbConfig(LocalityWeightedLbConfig),
        }
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RefreshRate {
        /// Specifies the base interval between refreshes. This parameter is required and must be greater
        /// than zero and less than
        /// :ref:`max_interval <envoy_api_field_config.cluster.v3.Cluster.RefreshRate.max_interval>`.
        #[prost(message, optional, tag = "1")]
        pub base_interval: ::std::option::Option<::prost_types::Duration>,
        /// Specifies the maximum interval between refreshes. This parameter is optional, but must be
        /// greater than or equal to the
        /// :ref:`base_interval <envoy_api_field_config.cluster.v3.Cluster.RefreshRate.base_interval>`  if set. The default
        /// is 10 times the :ref:`base_interval <envoy_api_field_config.cluster.v3.Cluster.RefreshRate.base_interval>`.
        #[prost(message, optional, tag = "2")]
        pub max_interval: ::std::option::Option<::prost_types::Duration>,
    }
    /// [#not-implemented-hide:]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct PrefetchPolicy {
        /// Indicates how many many streams (rounded up) can be anticipated per-upstream for each
        /// stream, useful for high-QPS or latency-sensitive services.
        ///
        /// For example if this is 2, for an incoming HTTP/1.1 stream, 2 connections will be
        /// established, one for the new incoming stream, and one for a presumed follow-up stream. For
        /// HTTP/2, only one connection would be established by default as one connection can
        /// serve both the original and presumed follow-up stream.
        ///
        /// In steady state for non-multiplexed connections a value of 1.5 would mean if there were 100
        /// active streams, there would be 100 connections in use, and 50 connections prefetched.
        /// This might be a useful value for something like short lived single-use connections,
        /// for example proxying HTTP/1.1 if keep-alive were false and each stream resulted in connection
        /// termination. It would likely be overkill for long lived connections, such as TCP proxying SMTP
        /// or regular HTTP/1.1 with keep-alive. For long lived traffic, a value of 1.05 would be more
        /// reasonable, where for every 100 connections, 5 prefetched connections would be in the queue
        /// in case of unexpected disconnects where the connection could not be reused.
        ///
        /// If this value is not set, or set explicitly to one, Envoy will fetch as many connections
        /// as needed to serve streams in flight. This means in steady state if a connection is torn down,
        /// a subsequent streams will pay an upstream-rtt latency penalty waiting for streams to be
        /// prefetched.
        ///
        /// This is limited somewhat arbitrarily to 3 because prefetching connections too aggressively can
        /// harm latency more than the prefetching helps.
        #[prost(message, optional, tag = "1")]
        pub prefetch_ratio: ::std::option::Option<f64>,
    }
    /// Refer to :ref:`service discovery type <arch_overview_service_discovery_types>`
    /// for an explanation on each type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DiscoveryType {
        /// Refer to the :ref:`static discovery type<arch_overview_service_discovery_types_static>`
        /// for an explanation.
        Static = 0,
        /// Refer to the :ref:`strict DNS discovery
        /// type<arch_overview_service_discovery_types_strict_dns>`
        /// for an explanation.
        StrictDns = 1,
        /// Refer to the :ref:`logical DNS discovery
        /// type<arch_overview_service_discovery_types_logical_dns>`
        /// for an explanation.
        LogicalDns = 2,
        /// Refer to the :ref:`service discovery type<arch_overview_service_discovery_types_eds>`
        /// for an explanation.
        Eds = 3,
        /// Refer to the :ref:`original destination discovery
        /// type<arch_overview_service_discovery_types_original_destination>`
        /// for an explanation.
        OriginalDst = 4,
    }
    /// Refer to :ref:`load balancer type <arch_overview_load_balancing_types>` architecture
    /// overview section for information on each type.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum LbPolicy {
        /// Refer to the :ref:`round robin load balancing
        /// policy<arch_overview_load_balancing_types_round_robin>`
        /// for an explanation.
        RoundRobin = 0,
        /// Refer to the :ref:`least request load balancing
        /// policy<arch_overview_load_balancing_types_least_request>`
        /// for an explanation.
        LeastRequest = 1,
        /// Refer to the :ref:`ring hash load balancing
        /// policy<arch_overview_load_balancing_types_ring_hash>`
        /// for an explanation.
        RingHash = 2,
        /// Refer to the :ref:`random load balancing
        /// policy<arch_overview_load_balancing_types_random>`
        /// for an explanation.
        Random = 3,
        /// Refer to the :ref:`Maglev load balancing policy<arch_overview_load_balancing_types_maglev>`
        /// for an explanation.
        Maglev = 5,
        /// This load balancer type must be specified if the configured cluster provides a cluster
        /// specific load balancer. Consult the configured cluster's documentation for whether to set
        /// this option or not.
        ClusterProvided = 6,
        /// [#not-implemented-hide:] Use the new :ref:`load_balancing_policy
        /// <envoy_api_field_config.cluster.v3.Cluster.load_balancing_policy>` field to determine the LB policy.
        /// [#next-major-version: In the v3 API, we should consider deprecating the lb_policy field
        /// and instead using the new load_balancing_policy field as the one and only mechanism for
        /// configuring this.]
        LoadBalancingPolicyConfig = 7,
    }
    /// When V4_ONLY is selected, the DNS resolver will only perform a lookup for
    /// addresses in the IPv4 family. If V6_ONLY is selected, the DNS resolver will
    /// only perform a lookup for addresses in the IPv6 family. If AUTO is
    /// specified, the DNS resolver will first perform a lookup for addresses in
    /// the IPv6 family and fallback to a lookup for addresses in the IPv4 family.
    /// For cluster types other than
    /// :ref:`STRICT_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.STRICT_DNS>` and
    /// :ref:`LOGICAL_DNS<envoy_api_enum_value_config.cluster.v3.Cluster.DiscoveryType.LOGICAL_DNS>`,
    /// this setting is
    /// ignored.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum DnsLookupFamily {
        Auto = 0,
        V4Only = 1,
        V6Only = 2,
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ClusterProtocolSelection {
        /// Cluster can only operate on one of the possible upstream protocols (HTTP1.1, HTTP2).
        /// If :ref:`http2_protocol_options <envoy_api_field_config.cluster.v3.Cluster.http2_protocol_options>` are
        /// present, HTTP2 will be used, otherwise HTTP1.1 will be used.
        UseConfiguredProtocol = 0,
        /// Use HTTP1.1 or HTTP2, depending on which one is used on the downstream connection.
        UseDownstreamProtocol = 1,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ClusterDiscoveryType {
        /// The :ref:`service discovery type <arch_overview_service_discovery_types>`
        /// to use for resolving the cluster.
        #[prost(enumeration = "DiscoveryType", tag = "2")]
        Type(i32),
        /// The custom cluster type.
        #[prost(message, tag = "38")]
        ClusterType(CustomClusterType),
    }
    /// Optional configuration for the load balancing algorithm selected by
    /// LbPolicy. Currently only
    /// :ref:`RING_HASH<envoy_api_enum_value_config.cluster.v3.Cluster.LbPolicy.RING_HASH>` and
    /// :ref:`LEAST_REQUEST<envoy_api_enum_value_config.cluster.v3.Cluster.LbPolicy.LEAST_REQUEST>`
    /// has additional configuration options.
    /// Specifying ring_hash_lb_config or least_request_lb_config without setting the corresponding
    /// LbPolicy will generate an error at runtime.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum LbConfig {
        /// Optional configuration for the Ring Hash load balancing policy.
        #[prost(message, tag = "23")]
        RingHashLbConfig(RingHashLbConfig),
        /// Optional configuration for the Original Destination load balancing policy.
        #[prost(message, tag = "34")]
        OriginalDstLbConfig(OriginalDstLbConfig),
        /// Optional configuration for the LeastRequest load balancing policy.
        #[prost(message, tag = "37")]
        LeastRequestLbConfig(LeastRequestLbConfig),
    }
}
/// [#not-implemented-hide:] Extensible load balancing policy configuration.
///
/// Every LB policy defined via this mechanism will be identified via a unique name using reverse
/// DNS notation. If the policy needs configuration parameters, it must define a message for its
/// own configuration, which will be stored in the config field. The name of the policy will tell
/// clients which type of message they should expect to see in the config field.
///
/// Note that there are cases where it is useful to be able to independently select LB policies
/// for choosing a locality and for choosing an endpoint within that locality. For example, a
/// given deployment may always use the same policy to choose the locality, but for choosing the
/// endpoint within the locality, some clusters may use weighted-round-robin, while others may
/// use some sort of session-based balancing.
///
/// This can be accomplished via hierarchical LB policies, where the parent LB policy creates a
/// child LB policy for each locality. For each request, the parent chooses the locality and then
/// delegates to the child policy for that locality to choose the endpoint within the locality.
///
/// To facilitate this, the config message for the top-level LB policy may include a field of
/// type LoadBalancingPolicy that specifies the child policy.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LoadBalancingPolicy {
    /// Each client will iterate over the list in order and stop at the first policy that it
    /// supports. This provides a mechanism for starting to use new LB policies that are not yet
    /// supported by all clients.
    #[prost(message, repeated, tag = "1")]
    pub policies: ::std::vec::Vec<load_balancing_policy::Policy>,
}
pub mod load_balancing_policy {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Policy {
        /// Required. The name of the LB policy.
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        #[prost(message, optional, tag = "3")]
        pub typed_config: ::std::option::Option<::prost_types::Any>,
    }
}
/// An extensible structure containing the address Envoy should bind to when
/// establishing upstream connections.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamBindConfig {
    /// The address Envoy should bind to when establishing upstream connections.
    #[prost(message, optional, tag = "1")]
    pub source_address: ::std::option::Option<super::super::core::v3::Address>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpstreamConnectionOptions {
    /// If set then set SO_KEEPALIVE on the socket to enable TCP Keepalives.
    #[prost(message, optional, tag = "1")]
    pub tcp_keepalive: ::std::option::Option<super::super::core::v3::TcpKeepalive>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TrackClusterStats {
    /// If timeout_budgets is true, the :ref:`timeout budget histograms
    /// <config_cluster_manager_cluster_stats_timeout_budgets>` will be published for each
    /// request. These show what percentage of a request's per try and global timeout was used. A value
    /// of 0 would indicate that none of the timeout was used or that the timeout was infinite. A value
    /// of 100 would indicate that the request took the entirety of the timeout given to it.
    #[prost(bool, tag = "1")]
    pub timeout_budgets: bool,
    /// If request_response_sizes is true, then the :ref:`histograms
    /// <config_cluster_manager_cluster_stats_request_response_sizes>`  tracking header and body sizes
    /// of requests and responses will be published.
    #[prost(bool, tag = "2")]
    pub request_response_sizes: bool,
}
