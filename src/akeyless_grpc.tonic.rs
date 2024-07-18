// @generated
/// Generated client implementations.
pub mod akeyless_v2_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    #[derive(Debug, Clone)]
    pub struct AkeylessV2ServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl AkeylessV2ServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> AkeylessV2ServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> AkeylessV2ServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            AkeylessV2ServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        pub async fn auth(
            &mut self,
            request: impl tonic::IntoRequest<super::AuthRequest>,
        ) -> std::result::Result<tonic::Response<super::AuthOutput>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/akeyless_grpc.AkeylessV2Service/Auth",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("akeyless_grpc.AkeylessV2Service", "Auth"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn create_secret(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSecretRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CreateSecretOutput>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/akeyless_grpc.AkeylessV2Service/CreateSecret",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("akeyless_grpc.AkeylessV2Service", "CreateSecret"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn delete_item(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::DeleteItemOutput>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/akeyless_grpc.AkeylessV2Service/DeleteItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("akeyless_grpc.AkeylessV2Service", "DeleteItem"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn describe_item(
            &mut self,
            request: impl tonic::IntoRequest<super::DescribeItemRequest>,
        ) -> std::result::Result<tonic::Response<super::Item>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/akeyless_grpc.AkeylessV2Service/DescribeItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("akeyless_grpc.AkeylessV2Service", "DescribeItem"),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn get_rotated_secret_value(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRotatedSecretValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetRotatedSecretValueResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/akeyless_grpc.AkeylessV2Service/GetRotatedSecretValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new(
                        "akeyless_grpc.AkeylessV2Service",
                        "GetRotatedSecretValue",
                    ),
                );
            self.inner.unary(req, path, codec).await
        }
        ///
        pub async fn get_secret_value(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSecretValueRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetSecretValueResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/akeyless_grpc.AkeylessV2Service/GetSecretValue",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("akeyless_grpc.AkeylessV2Service", "GetSecretValue"),
                );
            self.inner.unary(req, path, codec).await
        }
        pub async fn list_items(
            &mut self,
            request: impl tonic::IntoRequest<super::ListItemsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::ListItemsInPathOutput>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/akeyless_grpc.AkeylessV2Service/ListItems",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("akeyless_grpc.AkeylessV2Service", "ListItems"));
            self.inner.unary(req, path, codec).await
        }
        pub async fn update_secret_val(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateSecretValRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateSecretValOutput>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/akeyless_grpc.AkeylessV2Service/UpdateSecretVal",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("akeyless_grpc.AkeylessV2Service", "UpdateSecretVal"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
