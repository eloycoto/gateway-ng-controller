// [#protodoc-title: Common discovery API components]

/// A DiscoveryRequest requests a set of versioned resources of the same type for
/// a given Envoy node on some API.
/// [#next-free-field: 7]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryRequest {
    /// The version_info provided in the request messages will be the version_info
    /// received with the most recent successfully processed response or empty on
    /// the first request. It is expected that no new request is sent after a
    /// response is received until the Envoy instance is ready to ACK/NACK the new
    /// configuration. ACK/NACK takes place by returning the new API config version
    /// as applied or the previous API config version respectively. Each type_url
    /// (see below) has an independent version associated with it.
    #[prost(string, tag = "1")]
    pub version_info: std::string::String,
    /// The node making the request.
    #[prost(message, optional, tag = "2")]
    pub node: ::std::option::Option<super::super::super::config::core::v3::Node>,
    /// List of resources to subscribe to, e.g. list of cluster names or a route
    /// configuration name. If this is empty, all resources for the API are
    /// returned. LDS/CDS may have empty resource_names, which will cause all
    /// resources for the Envoy instance to be returned. The LDS and CDS responses
    /// will then imply a number of resources that need to be fetched via EDS/RDS,
    /// which will be explicitly enumerated in resource_names.
    #[prost(string, repeated, tag = "3")]
    pub resource_names: ::std::vec::Vec<std::string::String>,
    /// Type of the resource that is being requested, e.g.
    /// "type.googleapis.com/envoy.api.v2.ClusterLoadAssignment". This is implicit
    /// in requests made via singleton xDS APIs such as CDS, LDS, etc. but is
    /// required for ADS.
    #[prost(string, tag = "4")]
    pub type_url: std::string::String,
    /// nonce corresponding to DiscoveryResponse being ACK/NACKed. See above
    /// discussion on version_info and the DiscoveryResponse nonce comment. This
    /// may be empty only if 1) this is a non-persistent-stream xDS such as HTTP,
    /// or 2) the client has not yet accepted an update in this xDS stream (unlike
    /// delta, where it is populated only for new explicit ACKs).
    #[prost(string, tag = "5")]
    pub response_nonce: std::string::String,
    /// This is populated when the previous :ref:`DiscoveryResponse <envoy_api_msg_service.discovery.v3.DiscoveryResponse>`
    /// failed to update configuration. The *message* field in *error_details* provides the Envoy
    /// internal exception related to the failure. It is only intended for consumption during manual
    /// debugging, the string provided is not guaranteed to be stable across Envoy versions.
    #[prost(message, optional, tag = "6")]
    pub error_detail: ::std::option::Option<super::super::super::super::google::rpc::Status>,
}
/// [#next-free-field: 7]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiscoveryResponse {
    /// The version of the response data.
    #[prost(string, tag = "1")]
    pub version_info: std::string::String,
    /// The response resources. These resources are typed and depend on the API being called.
    #[prost(message, repeated, tag = "2")]
    pub resources: ::std::vec::Vec<::prost_types::Any>,
    /// [#not-implemented-hide:]
    /// Canary is used to support two Envoy command line flags:
    ///
    /// * --terminate-on-canary-transition-failure. When set, Envoy is able to
    ///   terminate if it detects that configuration is stuck at canary. Consider
    ///   this example sequence of updates:
    ///   - Management server applies a canary config successfully.
    ///   - Management server rolls back to a production config.
    ///   - Envoy rejects the new production config.
    ///   Since there is no sensible way to continue receiving configuration
    ///   updates, Envoy will then terminate and apply production config from a
    ///   clean slate.
    /// * --dry-run-canary. When set, a canary response will never be applied, only
    ///   validated via a dry run.
    #[prost(bool, tag = "3")]
    pub canary: bool,
    /// Type URL for resources. Identifies the xDS API when muxing over ADS.
    /// Must be consistent with the type_url in the 'resources' repeated Any (if non-empty).
    #[prost(string, tag = "4")]
    pub type_url: std::string::String,
    /// For gRPC based subscriptions, the nonce provides a way to explicitly ack a
    /// specific DiscoveryResponse in a following DiscoveryRequest. Additional
    /// messages may have been sent by Envoy to the management server for the
    /// previous version on the stream prior to this DiscoveryResponse, that were
    /// unprocessed at response send time. The nonce allows the management server
    /// to ignore any further DiscoveryRequests for the previous version until a
    /// DiscoveryRequest bearing the nonce. The nonce is optional and is not
    /// required for non-stream based xDS implementations.
    #[prost(string, tag = "5")]
    pub nonce: std::string::String,
    /// [#not-implemented-hide:]
    /// The control plane instance that sent the response.
    #[prost(message, optional, tag = "6")]
    pub control_plane: ::std::option::Option<super::super::super::config::core::v3::ControlPlane>,
}
/// DeltaDiscoveryRequest and DeltaDiscoveryResponse are used in a new gRPC
/// endpoint for Delta xDS.
///
/// With Delta xDS, the DeltaDiscoveryResponses do not need to include a full
/// snapshot of the tracked resources. Instead, DeltaDiscoveryResponses are a
/// diff to the state of a xDS client.
/// In Delta XDS there are per-resource versions, which allow tracking state at
/// the resource granularity.
/// An xDS Delta session is always in the context of a gRPC bidirectional
/// stream. This allows the xDS server to keep track of the state of xDS clients
/// connected to it.
///
/// In Delta xDS the nonce field is required and used to pair
/// DeltaDiscoveryResponse to a DeltaDiscoveryRequest ACK or NACK.
/// Optionally, a response message level system_version_info is present for
/// debugging purposes only.
///
/// DeltaDiscoveryRequest plays two independent roles. Any DeltaDiscoveryRequest
/// can be either or both of: [1] informing the server of what resources the
/// client has gained/lost interest in (using resource_names_subscribe and
/// resource_names_unsubscribe), or [2] (N)ACKing an earlier resource update from
/// the server (using response_nonce, with presence of error_detail making it a NACK).
/// Additionally, the first message (for a given type_url) of a reconnected gRPC stream
/// has a third role: informing the server of the resources (and their versions)
/// that the client already possesses, using the initial_resource_versions field.
///
/// As with state-of-the-world, when multiple resource types are multiplexed (ADS),
/// all requests/acknowledgments/updates are logically walled off by type_url:
/// a Cluster ACK exists in a completely separate world from a prior Route NACK.
/// In particular, initial_resource_versions being sent at the "start" of every
/// gRPC stream actually entails a message for each type_url, each with its own
/// initial_resource_versions.
/// [#next-free-field: 10]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaDiscoveryRequest {
    /// The node making the request.
    #[prost(message, optional, tag = "1")]
    pub node: ::std::option::Option<super::super::super::config::core::v3::Node>,
    /// Type of the resource that is being requested, e.g.
    /// "type.googleapis.com/envoy.api.v2.ClusterLoadAssignment". This does not need to be set if
    /// resources are only referenced via *udpa_resource_subscribe* and
    /// *udpa_resources_unsubscribe*.
    #[prost(string, tag = "2")]
    pub type_url: std::string::String,
    /// DeltaDiscoveryRequests allow the client to add or remove individual
    /// resources to the set of tracked resources in the context of a stream.
    /// All resource names in the resource_names_subscribe list are added to the
    /// set of tracked resources and all resource names in the resource_names_unsubscribe
    /// list are removed from the set of tracked resources.
    ///
    /// *Unlike* state-of-the-world xDS, an empty resource_names_subscribe or
    /// resource_names_unsubscribe list simply means that no resources are to be
    /// added or removed to the resource list.
    /// *Like* state-of-the-world xDS, the server must send updates for all tracked
    /// resources, but can also send updates for resources the client has not subscribed to.
    ///
    /// NOTE: the server must respond with all resources listed in resource_names_subscribe,
    /// even if it believes the client has the most recent version of them. The reason:
    /// the client may have dropped them, but then regained interest before it had a chance
    /// to send the unsubscribe message. See DeltaSubscriptionStateTest.RemoveThenAdd.
    ///
    /// These two fields can be set in any DeltaDiscoveryRequest, including ACKs
    /// and initial_resource_versions.
    ///
    /// A list of Resource names to add to the list of tracked resources.
    #[prost(string, repeated, tag = "3")]
    pub resource_names_subscribe: ::std::vec::Vec<std::string::String>,
    /// As with *resource_names_subscribe* but used when subscribing to resources indicated
    /// by a *udpa.core.v1.ResourceLocator*. The directives in the resource locator
    /// are ignored and the context parameters are matched with
    /// *context_param_specifier* specific semantics.
    /// [#not-implemented-hide:]
    #[prost(message, repeated, tag = "8")]
    pub udpa_resources_subscribe:
        ::std::vec::Vec<super::super::super::super::udpa::core::v1::ResourceLocator>,
    /// A list of Resource names to remove from the list of tracked resources.
    #[prost(string, repeated, tag = "4")]
    pub resource_names_unsubscribe: ::std::vec::Vec<std::string::String>,
    /// As with *resource_names_unsubscribe* but used when unsubscribing to resources indicated by a
    /// *udpa.core.v1.ResourceLocator*. This must match a previously subscribed
    /// resource locator provided in *udpa_resources_subscribe*.
    /// [#not-implemented-hide:]
    #[prost(message, repeated, tag = "9")]
    pub udpa_resources_unsubscribe:
        ::std::vec::Vec<super::super::super::super::udpa::core::v1::ResourceLocator>,
    /// Informs the server of the versions of the resources the xDS client knows of, to enable the
    /// client to continue the same logical xDS session even in the face of gRPC stream reconnection.
    /// It will not be populated: [1] in the very first stream of a session, since the client will
    /// not yet have any resources,  [2] in any message after the first in a stream (for a given
    /// type_url), since the server will already be correctly tracking the client's state.
    /// (In ADS, the first message *of each type_url* of a reconnected stream populates this map.)
    /// The map's keys are names of xDS resources known to the xDS client.
    /// The map's values are opaque resource versions.
    #[prost(map = "string, string", tag = "5")]
    pub initial_resource_versions:
        ::std::collections::HashMap<std::string::String, std::string::String>,
    /// When the DeltaDiscoveryRequest is a ACK or NACK message in response
    /// to a previous DeltaDiscoveryResponse, the response_nonce must be the
    /// nonce in the DeltaDiscoveryResponse.
    /// Otherwise (unlike in DiscoveryRequest) response_nonce must be omitted.
    #[prost(string, tag = "6")]
    pub response_nonce: std::string::String,
    /// This is populated when the previous :ref:`DiscoveryResponse <envoy_api_msg_service.discovery.v3.DiscoveryResponse>`
    /// failed to update configuration. The *message* field in *error_details*
    /// provides the Envoy internal exception related to the failure.
    #[prost(message, optional, tag = "7")]
    pub error_detail: ::std::option::Option<super::super::super::super::google::rpc::Status>,
}
/// [#next-free-field: 8]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeltaDiscoveryResponse {
    /// The version of the response data (used for debugging).
    #[prost(string, tag = "1")]
    pub system_version_info: std::string::String,
    /// The response resources. These are typed resources, whose types must match
    /// the type_url field.
    #[prost(message, repeated, tag = "2")]
    pub resources: ::std::vec::Vec<Resource>,
    // field id 3 IS available!
    /// Type URL for resources. Identifies the xDS API when muxing over ADS.
    /// Must be consistent with the type_url in the Any within 'resources' if 'resources' is non-empty.
    /// This does not need to be set if *udpa_removed_resources* is used instead of
    /// *removed_resources*.
    #[prost(string, tag = "4")]
    pub type_url: std::string::String,
    /// Resources names of resources that have be deleted and to be removed from the xDS Client.
    /// Removed resources for missing resources can be ignored.
    #[prost(string, repeated, tag = "6")]
    pub removed_resources: ::std::vec::Vec<std::string::String>,
    /// As with *removed_resources* but used when a removed resource was named in
    /// its *Resource*s with a *udpa.core.v1.ResourceName*.
    /// [#not-implemented-hide:]
    #[prost(message, repeated, tag = "7")]
    pub udpa_removed_resources:
        ::std::vec::Vec<super::super::super::super::udpa::core::v1::ResourceName>,
    /// The nonce provides a way for DeltaDiscoveryRequests to uniquely
    /// reference a DeltaDiscoveryResponse when (N)ACKing. The nonce is required.
    #[prost(string, tag = "5")]
    pub nonce: std::string::String,
}
/// [#next-free-field: 6]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    /// The resource's name, to distinguish it from others of the same type of resource.
    #[prost(string, tag = "3")]
    pub name: std::string::String,
    /// Used instead of *name* when a resource with a *udpa.core.v1.ResourceName* is delivered.
    #[prost(message, optional, tag = "5")]
    pub udpa_resource_name:
        ::std::option::Option<super::super::super::super::udpa::core::v1::ResourceName>,
    /// The aliases are a list of other names that this resource can go by.
    #[prost(string, repeated, tag = "4")]
    pub aliases: ::std::vec::Vec<std::string::String>,
    /// The resource level version. It allows xDS to track the state of individual
    /// resources.
    #[prost(string, tag = "1")]
    pub version: std::string::String,
    /// The resource being tracked.
    #[prost(message, optional, tag = "2")]
    pub resource: ::std::option::Option<::prost_types::Any>,
}
