// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_flow::_create_flow_output::CreateFlowOutputBuilder;

pub use crate::operation::create_flow::_create_flow_input::CreateFlowInputBuilder;

impl crate::operation::create_flow::builders::CreateFlowInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_flow::CreateFlowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_flow::CreateFlowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_flow();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateFlow`.
///
/// <p>Creates a new flow. The request must include one source. The request optionally can include outputs (up to 50) and entitlements (up to 50).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateFlowFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_flow::builders::CreateFlowInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_flow::CreateFlowOutput,
        crate::operation::create_flow::CreateFlowError,
    > for CreateFlowFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_flow::CreateFlowOutput,
            crate::operation::create_flow::CreateFlowError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateFlowFluentBuilder {
    /// Creates a new `CreateFlowFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateFlow as a reference.
    pub fn as_input(&self) -> &crate::operation::create_flow::builders::CreateFlowInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::create_flow::CreateFlowOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_flow::CreateFlowError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_flow::CreateFlow::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_flow::CreateFlow::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_flow::CreateFlowOutput,
        crate::operation::create_flow::CreateFlowError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The Availability Zone that you want to create the flow in. These options are limited to the Availability Zones within the current Amazon Web Services Region.</p>
    pub fn availability_zone(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.availability_zone(input.into());
        self
    }
    /// <p>The Availability Zone that you want to create the flow in. These options are limited to the Availability Zones within the current Amazon Web Services Region.</p>
    pub fn set_availability_zone(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_availability_zone(input);
        self
    }
    /// <p>The Availability Zone that you want to create the flow in. These options are limited to the Availability Zones within the current Amazon Web Services Region.</p>
    pub fn get_availability_zone(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_availability_zone()
    }
    ///
    /// Appends an item to `Entitlements`.
    ///
    /// To override the contents of this collection use [`set_entitlements`](Self::set_entitlements).
    ///
    /// <p>The entitlements that you want to grant on a flow.</p>
    pub fn entitlements(mut self, input: crate::types::GrantEntitlementRequest) -> Self {
        self.inner = self.inner.entitlements(input);
        self
    }
    /// <p>The entitlements that you want to grant on a flow.</p>
    pub fn set_entitlements(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::GrantEntitlementRequest>>) -> Self {
        self.inner = self.inner.set_entitlements(input);
        self
    }
    /// <p>The entitlements that you want to grant on a flow.</p>
    pub fn get_entitlements(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::GrantEntitlementRequest>> {
        self.inner.get_entitlements()
    }
    ///
    /// Appends an item to `MediaStreams`.
    ///
    /// To override the contents of this collection use [`set_media_streams`](Self::set_media_streams).
    ///
    /// <p>The media streams that you want to add to the flow. You can associate these media streams with sources and outputs on the flow.</p>
    pub fn media_streams(mut self, input: crate::types::AddMediaStreamRequest) -> Self {
        self.inner = self.inner.media_streams(input);
        self
    }
    /// <p>The media streams that you want to add to the flow. You can associate these media streams with sources and outputs on the flow.</p>
    pub fn set_media_streams(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AddMediaStreamRequest>>) -> Self {
        self.inner = self.inner.set_media_streams(input);
        self
    }
    /// <p>The media streams that you want to add to the flow. You can associate these media streams with sources and outputs on the flow.</p>
    pub fn get_media_streams(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AddMediaStreamRequest>> {
        self.inner.get_media_streams()
    }
    /// <p>The name of the flow.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the flow.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the flow.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    ///
    /// Appends an item to `Outputs`.
    ///
    /// To override the contents of this collection use [`set_outputs`](Self::set_outputs).
    ///
    /// <p>The outputs that you want to add to this flow.</p>
    pub fn outputs(mut self, input: crate::types::AddOutputRequest) -> Self {
        self.inner = self.inner.outputs(input);
        self
    }
    /// <p>The outputs that you want to add to this flow.</p>
    pub fn set_outputs(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AddOutputRequest>>) -> Self {
        self.inner = self.inner.set_outputs(input);
        self
    }
    /// <p>The outputs that you want to add to this flow.</p>
    pub fn get_outputs(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AddOutputRequest>> {
        self.inner.get_outputs()
    }
    /// <p>The settings for the source that you want to use for the new flow.</p>
    pub fn source(mut self, input: crate::types::SetSourceRequest) -> Self {
        self.inner = self.inner.source(input);
        self
    }
    /// <p>The settings for the source that you want to use for the new flow.</p>
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::SetSourceRequest>) -> Self {
        self.inner = self.inner.set_source(input);
        self
    }
    /// <p>The settings for the source that you want to use for the new flow.</p>
    pub fn get_source(&self) -> &::std::option::Option<crate::types::SetSourceRequest> {
        self.inner.get_source()
    }
    /// <p>The settings for source failover.</p>
    pub fn source_failover_config(mut self, input: crate::types::FailoverConfig) -> Self {
        self.inner = self.inner.source_failover_config(input);
        self
    }
    /// <p>The settings for source failover.</p>
    pub fn set_source_failover_config(mut self, input: ::std::option::Option<crate::types::FailoverConfig>) -> Self {
        self.inner = self.inner.set_source_failover_config(input);
        self
    }
    /// <p>The settings for source failover.</p>
    pub fn get_source_failover_config(&self) -> &::std::option::Option<crate::types::FailoverConfig> {
        self.inner.get_source_failover_config()
    }
    ///
    /// Appends an item to `Sources`.
    ///
    /// To override the contents of this collection use [`set_sources`](Self::set_sources).
    ///
    /// <p>The sources that are assigned to the flow.</p>
    pub fn sources(mut self, input: crate::types::SetSourceRequest) -> Self {
        self.inner = self.inner.sources(input);
        self
    }
    /// <p>The sources that are assigned to the flow.</p>
    pub fn set_sources(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::SetSourceRequest>>) -> Self {
        self.inner = self.inner.set_sources(input);
        self
    }
    /// <p>The sources that are assigned to the flow.</p>
    pub fn get_sources(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::SetSourceRequest>> {
        self.inner.get_sources()
    }
    ///
    /// Appends an item to `VpcInterfaces`.
    ///
    /// To override the contents of this collection use [`set_vpc_interfaces`](Self::set_vpc_interfaces).
    ///
    /// <p>The VPC interfaces you want on the flow.</p>
    pub fn vpc_interfaces(mut self, input: crate::types::VpcInterfaceRequest) -> Self {
        self.inner = self.inner.vpc_interfaces(input);
        self
    }
    /// <p>The VPC interfaces you want on the flow.</p>
    pub fn set_vpc_interfaces(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::VpcInterfaceRequest>>) -> Self {
        self.inner = self.inner.set_vpc_interfaces(input);
        self
    }
    /// <p>The VPC interfaces you want on the flow.</p>
    pub fn get_vpc_interfaces(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::VpcInterfaceRequest>> {
        self.inner.get_vpc_interfaces()
    }
    /// <p>The maintenance settings you want to use for the flow.</p>
    pub fn maintenance(mut self, input: crate::types::AddMaintenance) -> Self {
        self.inner = self.inner.maintenance(input);
        self
    }
    /// <p>The maintenance settings you want to use for the flow.</p>
    pub fn set_maintenance(mut self, input: ::std::option::Option<crate::types::AddMaintenance>) -> Self {
        self.inner = self.inner.set_maintenance(input);
        self
    }
    /// <p>The maintenance settings you want to use for the flow.</p>
    pub fn get_maintenance(&self) -> &::std::option::Option<crate::types::AddMaintenance> {
        self.inner.get_maintenance()
    }
    /// <p>The settings for source monitoring.</p>
    pub fn source_monitoring_config(mut self, input: crate::types::MonitoringConfig) -> Self {
        self.inner = self.inner.source_monitoring_config(input);
        self
    }
    /// <p>The settings for source monitoring.</p>
    pub fn set_source_monitoring_config(mut self, input: ::std::option::Option<crate::types::MonitoringConfig>) -> Self {
        self.inner = self.inner.set_source_monitoring_config(input);
        self
    }
    /// <p>The settings for source monitoring.</p>
    pub fn get_source_monitoring_config(&self) -> &::std::option::Option<crate::types::MonitoringConfig> {
        self.inner.get_source_monitoring_config()
    }
    /// <p>Determines the processing capacity and feature set of the flow. Set this optional parameter to <code>LARGE</code> if you want to enable NDI outputs on the flow.</p>
    pub fn flow_size(mut self, input: crate::types::FlowSize) -> Self {
        self.inner = self.inner.flow_size(input);
        self
    }
    /// <p>Determines the processing capacity and feature set of the flow. Set this optional parameter to <code>LARGE</code> if you want to enable NDI outputs on the flow.</p>
    pub fn set_flow_size(mut self, input: ::std::option::Option<crate::types::FlowSize>) -> Self {
        self.inner = self.inner.set_flow_size(input);
        self
    }
    /// <p>Determines the processing capacity and feature set of the flow. Set this optional parameter to <code>LARGE</code> if you want to enable NDI outputs on the flow.</p>
    pub fn get_flow_size(&self) -> &::std::option::Option<crate::types::FlowSize> {
        self.inner.get_flow_size()
    }
    /// <p>Specifies the configuration settings for NDI outputs. Required when the flow includes NDI outputs.</p>
    pub fn ndi_config(mut self, input: crate::types::NdiConfig) -> Self {
        self.inner = self.inner.ndi_config(input);
        self
    }
    /// <p>Specifies the configuration settings for NDI outputs. Required when the flow includes NDI outputs.</p>
    pub fn set_ndi_config(mut self, input: ::std::option::Option<crate::types::NdiConfig>) -> Self {
        self.inner = self.inner.set_ndi_config(input);
        self
    }
    /// <p>Specifies the configuration settings for NDI outputs. Required when the flow includes NDI outputs.</p>
    pub fn get_ndi_config(&self) -> &::std::option::Option<crate::types::NdiConfig> {
        self.inner.get_ndi_config()
    }
}
