/// [#not-implemented-hide:] Not configuration. Workaround c++ protobuf issue with importing
/// services: https://github.com/google/protobuf/issues/4221 and protoxform to upgrade the file.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CdsDummy {}
#[doc = r" Generated client implementations."]
pub mod cluster_discovery_service_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = " Return list of all clusters this proxy will load balance to."]
    pub struct ClusterDiscoveryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ClusterDiscoveryServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ClusterDiscoveryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn stream_clusters(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::discovery::v3::DiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::discovery::v3::DiscoveryResponse>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.cluster.v3.ClusterDiscoveryService/StreamClusters",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        pub async fn delta_clusters(
            &mut self,
            request: impl tonic::IntoStreamingRequest<
                Message = super::super::super::discovery::v3::DeltaDiscoveryRequest,
            >,
        ) -> Result<
            tonic::Response<
                tonic::codec::Streaming<super::super::super::discovery::v3::DeltaDiscoveryResponse>,
            >,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.cluster.v3.ClusterDiscoveryService/DeltaClusters",
            );
            self.inner
                .streaming(request.into_streaming_request(), path, codec)
                .await
        }
        pub async fn fetch_clusters(
            &mut self,
            request: impl tonic::IntoRequest<super::super::super::discovery::v3::DiscoveryRequest>,
        ) -> Result<
            tonic::Response<super::super::super::discovery::v3::DiscoveryResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/envoy.service.cluster.v3.ClusterDiscoveryService/FetchClusters",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ClusterDiscoveryServiceClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for ClusterDiscoveryServiceClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "ClusterDiscoveryServiceClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod cluster_discovery_service_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ClusterDiscoveryServiceServer."]
    #[async_trait]
    pub trait ClusterDiscoveryService: Send + Sync + 'static {
        #[doc = "Server streaming response type for the StreamClusters method."]
        type StreamClustersStream: Stream<
                Item = Result<super::super::super::discovery::v3::DiscoveryResponse, tonic::Status>,
            > + Send
            + Sync
            + 'static;
        async fn stream_clusters(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::discovery::v3::DiscoveryRequest>,
            >,
        ) -> Result<tonic::Response<Self::StreamClustersStream>, tonic::Status>;
        #[doc = "Server streaming response type for the DeltaClusters method."]
        type DeltaClustersStream: Stream<
                Item = Result<
                    super::super::super::discovery::v3::DeltaDiscoveryResponse,
                    tonic::Status,
                >,
            > + Send
            + Sync
            + 'static;
        async fn delta_clusters(
            &self,
            request: tonic::Request<
                tonic::Streaming<super::super::super::discovery::v3::DeltaDiscoveryRequest>,
            >,
        ) -> Result<tonic::Response<Self::DeltaClustersStream>, tonic::Status>;
        async fn fetch_clusters(
            &self,
            request: tonic::Request<super::super::super::discovery::v3::DiscoveryRequest>,
        ) -> Result<
            tonic::Response<super::super::super::discovery::v3::DiscoveryResponse>,
            tonic::Status,
        >;
    }
    #[doc = " Return list of all clusters this proxy will load balance to."]
    #[derive(Debug)]
    pub struct ClusterDiscoveryServiceServer<T: ClusterDiscoveryService> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ClusterDiscoveryService> ClusterDiscoveryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for ClusterDiscoveryServiceServer<T>
    where
        T: ClusterDiscoveryService,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/envoy.service.cluster.v3.ClusterDiscoveryService/StreamClusters" => {
                    #[allow(non_camel_case_types)]
                    struct StreamClustersSvc<T: ClusterDiscoveryService>(pub Arc<T>);
                    impl<T: ClusterDiscoveryService>
                        tonic::server::StreamingService<
                            super::super::super::discovery::v3::DiscoveryRequest,
                        > for StreamClustersSvc<T>
                    {
                        type Response = super::super::super::discovery::v3::DiscoveryResponse;
                        type ResponseStream = T::StreamClustersStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::discovery::v3::DiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).stream_clusters(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = StreamClustersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/envoy.service.cluster.v3.ClusterDiscoveryService/DeltaClusters" => {
                    #[allow(non_camel_case_types)]
                    struct DeltaClustersSvc<T: ClusterDiscoveryService>(pub Arc<T>);
                    impl<T: ClusterDiscoveryService>
                        tonic::server::StreamingService<
                            super::super::super::discovery::v3::DeltaDiscoveryRequest,
                        > for DeltaClustersSvc<T>
                    {
                        type Response = super::super::super::discovery::v3::DeltaDiscoveryResponse;
                        type ResponseStream = T::DeltaClustersStream;
                        type Future =
                            BoxFuture<tonic::Response<Self::ResponseStream>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                tonic::Streaming<
                                    super::super::super::discovery::v3::DeltaDiscoveryRequest,
                                >,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delta_clusters(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1;
                        let inner = inner.0;
                        let method = DeltaClustersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.streaming(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/envoy.service.cluster.v3.ClusterDiscoveryService/FetchClusters" => {
                    #[allow(non_camel_case_types)]
                    struct FetchClustersSvc<T: ClusterDiscoveryService>(pub Arc<T>);
                    impl<T: ClusterDiscoveryService>
                        tonic::server::UnaryService<
                            super::super::super::discovery::v3::DiscoveryRequest,
                        > for FetchClustersSvc<T>
                    {
                        type Response = super::super::super::discovery::v3::DiscoveryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<
                                super::super::super::discovery::v3::DiscoveryRequest,
                            >,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).fetch_clusters(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = FetchClustersSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ClusterDiscoveryService> Clone for ClusterDiscoveryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ClusterDiscoveryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ClusterDiscoveryService> tonic::transport::NamedService
        for ClusterDiscoveryServiceServer<T>
    {
        const NAME: &'static str = "envoy.service.cluster.v3.ClusterDiscoveryService";
    }
}
