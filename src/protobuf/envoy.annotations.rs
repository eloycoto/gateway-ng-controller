#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ResourceAnnotation {
    /// Annotation for xDS services that indicates the fully-qualified Protobuf type for the resource
    /// type.
    #[prost(string, tag = "1")]
    pub r#type: std::string::String,
}
