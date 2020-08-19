/// UDPA authority information.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authority {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
/// Additional parameters that can be used to select resource variants.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextParams {
    #[prost(map = "string, string", tag = "1")]
    pub params: ::std::collections::HashMap<std::string::String, std::string::String>,
}
/// UDPA resource locators identify a UDPA resource name and instruct the
/// data-plane load balancer on how the resource may be located.
///
/// Resource locators have a canonical udpa:// URI representation:
///
///   udpa://{authority}/{type_url}/{id/*}?{context_params}{#directive,*}
///
/// where context_params take the form of URI query parameters.
///
/// Resource locators have a similar canonical http:// URI representation:
///
///   http://{authority}/{type_url}/{id/*}?{context_params}{#directive,*}
///
/// Resource locators also have a simplified file:// URI representation:
///
///   file:///{id/*}{#directive,*}
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceLocator {
    /// URI scheme.
    #[prost(enumeration = "resource_locator::Scheme", tag = "1")]
    pub scheme: i32,
    /// Opaque identifiers for the resource. These are effectively concatenated
    /// with ‘/’ to form the non-query param path as resource ID. This may end
    /// with ‘*’ for glob collection references.
    #[prost(string, repeated, tag = "2")]
    pub id: ::std::vec::Vec<std::string::String>,
    /// Logical authority for resource (not necessarily transport network address).
    /// Authorities are opaque in the UDPA API, data-plane load balancers will map
    /// them to concrete network transports such as an xDS management server, e.g.
    /// via envoy.config.core.v3.ConfigSource.
    #[prost(string, tag = "3")]
    pub authority: std::string::String,
    /// Fully qualified resource type (as in type URL without types.googleapis.com/
    /// prefix).
    #[prost(string, tag = "4")]
    pub resource_type: std::string::String,
    /// A list of directives that appear in the UDPA resource locator #fragment.
    ///
    /// When encoding to URI form, directives are percent encoded with comma
    /// separation.
    #[prost(message, repeated, tag = "6")]
    pub directives: ::std::vec::Vec<resource_locator::Directive>,
    #[prost(oneof = "resource_locator::ContextParamSpecifier", tags = "5")]
    pub context_param_specifier: ::std::option::Option<resource_locator::ContextParamSpecifier>,
}
pub mod resource_locator {
    /// Directives provide information to data-plane load balancers on how UDPA
    /// resource names are to be interpreted and potentially further resolved. For
    /// example, they may provide alternative resource locators for when primary
    /// resolution fails. Directives are not part of resource names and do not
    /// appear in a xDS transport discovery request.
    ///
    /// When encoding to URIs, directives take the form:
    ///
    /// <directive name>=<string representation of directive value>
    ///
    /// For example, we can have alt=udpa://foo/bar or entry=some%20thing. Each
    /// directive value type may have its own string encoding, in the case of
    /// ResourceLocator there is a recursive URI encoding.
    ///
    /// Percent encoding applies to the URI encoding of the directive value.
    /// Multiple directives are comma-separated, so the reserved characters that
    /// require percent encoding in a directive value are [',', '#', '[', ']',
    /// '%']. These are the RFC3986 fragment reserved characters with the addition
    /// of the UDPA scheme specific ','. See
    /// https://tools.ietf.org/html/rfc3986#page-49 for further details on URI ABNF
    /// and reserved characters.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Directive {
        #[prost(oneof = "directive::Directive", tags = "1, 2")]
        pub directive: ::std::option::Option<directive::Directive>,
    }
    pub mod directive {
        #[derive(Clone, PartialEq, ::prost::Oneof)]
        pub enum Directive {
            /// An alternative resource locator for fallback if the resource is
            /// unavailable. For example, take the resource locator:
            ///
            ///   udpa://foo/some-type/some-route-table#alt=udpa://bar/some-type/another-route-table
            ///
            /// If the data-plane load balancer is unable to reach `foo` to fetch the
            /// resource, it will fallback to `bar`. Alternative resources do not need
            /// to have equivalent content, but they should be functional substitutes.
            #[prost(message, tag = "1")]
            Alt(super::super::ResourceLocator),
            /// List collections support inlining of resources via the entry field in
            /// Resource. These inlined Resource objects may have an optional name
            /// field specified. When specified, the entry directive allows
            /// UdpaResourceLocator to directly reference these inlined resources, e.g.
            /// udpa://.../foo#entry=bar.
            #[prost(string, tag = "2")]
            Entry(std::string::String),
        }
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Scheme {
        Udpa = 0,
        Http = 1,
        File = 2,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ContextParamSpecifier {
        /// Additional parameters that can be used to select resource variants.
        /// Matches must be exact, i.e. all context parameters must match exactly and
        /// there must be no additional context parameters set on the matched
        /// resource.
        #[prost(message, tag = "5")]
        ExactContext(super::ContextParams),
    }
}
/// UDPA collection resource wrapper. This encapsulates a UDPA resource when
/// appearing inside a list collection resource. List collection resources are
/// regular Resource messages of type:
///
/// message <T>Collection {
///   repeated CollectionEntry resources = 1;
/// }
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionEntry {
    #[prost(oneof = "collection_entry::ResourceSpecifier", tags = "1, 2")]
    pub resource_specifier: ::std::option::Option<collection_entry::ResourceSpecifier>,
}
pub mod collection_entry {
    /// Inlined resource entry.
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct InlineEntry {
        /// Optional name to describe the inlined resource. Resource names must
        /// [a-zA-Z0-9_-\./]+ (TODO(htuch): turn this into a PGV constraint once
        /// finalized, probably should be a RFC3986 pchar). This name allows
        /// reference via the #entry directive in ResourceLocator.
        #[prost(string, tag = "1")]
        pub name: std::string::String,
        /// The resource's logical version. It is illegal to have the same named UDPA
        /// resource name at a given version with different resource payloads.
        #[prost(string, tag = "2")]
        pub version: std::string::String,
        /// The resource payload, including type URL.
        #[prost(message, optional, tag = "3")]
        pub resource: ::std::option::Option<::prost_types::Any>,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ResourceSpecifier {
        /// A resource locator describing how the member resource is to be located.
        #[prost(message, tag = "1")]
        Locator(super::ResourceLocator),
        /// The resource is inlined in the list collection.
        #[prost(message, tag = "2")]
        InlineEntry(InlineEntry),
    }
}
/// UDPA resource name. This has a canonical udpa:// URI representation:
///
///   udpa://{authority}/{type_url}/{id/*}?{context_params}
///
/// where context_params take the form of URI query parameters.
///
/// A UDPA resource name fully identifies a network resource for transport
/// purposes. UDPA resource names in this form appear only in discovery
/// request/response messages used with the xDS transport.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceName {
    /// Opaque identifiers for the resource. These are effectively concatenated
    /// with ‘/’ to form the non-query param path as resource ID.
    #[prost(string, repeated, tag = "1")]
    pub id: ::std::vec::Vec<std::string::String>,
    /// Logical authority for resource (not necessarily transport network address).
    /// Authorities are opaque in the UDPA API, data-plane load balancers will map
    /// them to concrete network transports such as an xDS management server.
    #[prost(string, tag = "2")]
    pub authority: std::string::String,
    /// Fully qualified resource type (as in type URL without types.googleapis.com/
    /// prefix).
    #[prost(string, tag = "3")]
    pub resource_type: std::string::String,
    /// Additional parameters that can be used to select resource variants.
    #[prost(message, optional, tag = "4")]
    pub context: ::std::option::Option<ContextParams>,
}
