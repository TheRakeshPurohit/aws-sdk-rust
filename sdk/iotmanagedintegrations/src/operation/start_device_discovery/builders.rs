// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_device_discovery::_start_device_discovery_output::StartDeviceDiscoveryOutputBuilder;

pub use crate::operation::start_device_discovery::_start_device_discovery_input::StartDeviceDiscoveryInputBuilder;

impl crate::operation::start_device_discovery::builders::StartDeviceDiscoveryInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_device_discovery::StartDeviceDiscoveryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_device_discovery::StartDeviceDiscoveryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_device_discovery();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartDeviceDiscovery`.
///
/// <p>This API is used to start device discovery for hub-connected and third-party-connected devices. The authentication material (install code) is passed as a message to the controller telling it to start the discovery.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartDeviceDiscoveryFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_device_discovery::builders::StartDeviceDiscoveryInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_device_discovery::StartDeviceDiscoveryOutput,
        crate::operation::start_device_discovery::StartDeviceDiscoveryError,
    > for StartDeviceDiscoveryFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_device_discovery::StartDeviceDiscoveryOutput,
            crate::operation::start_device_discovery::StartDeviceDiscoveryError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartDeviceDiscoveryFluentBuilder {
    /// Creates a new `StartDeviceDiscoveryFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartDeviceDiscovery as a reference.
    pub fn as_input(&self) -> &crate::operation::start_device_discovery::builders::StartDeviceDiscoveryInputBuilder {
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
        crate::operation::start_device_discovery::StartDeviceDiscoveryOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_device_discovery::StartDeviceDiscoveryError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_device_discovery::StartDeviceDiscovery::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_device_discovery::StartDeviceDiscovery::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_device_discovery::StartDeviceDiscoveryOutput,
        crate::operation::start_device_discovery::StartDeviceDiscoveryError,
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
    /// <p>The discovery type supporting the type of device to be discovered in the device discovery task request.</p>
    pub fn discovery_type(mut self, input: crate::types::DiscoveryType) -> Self {
        self.inner = self.inner.discovery_type(input);
        self
    }
    /// <p>The discovery type supporting the type of device to be discovered in the device discovery task request.</p>
    pub fn set_discovery_type(mut self, input: ::std::option::Option<crate::types::DiscoveryType>) -> Self {
        self.inner = self.inner.set_discovery_type(input);
        self
    }
    /// <p>The discovery type supporting the type of device to be discovered in the device discovery task request.</p>
    pub fn get_discovery_type(&self) -> &::std::option::Option<crate::types::DiscoveryType> {
        self.inner.get_discovery_type()
    }
    ///
    /// Adds a key-value pair to `CustomProtocolDetail`.
    ///
    /// To override the contents of this collection use [`set_custom_protocol_detail`](Self::set_custom_protocol_detail).
    ///
    /// <p>Additional protocol-specific details required for device discovery, which vary based on the discovery type.</p><note>
    /// <p>For a <code>DiscoveryType</code> of <code>CUSTOM</code>, the string-to-string map must have a key value of <code>Name</code> set to a non-empty-string.</p>
    /// </note>
    pub fn custom_protocol_detail(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.custom_protocol_detail(k.into(), v.into());
        self
    }
    /// <p>Additional protocol-specific details required for device discovery, which vary based on the discovery type.</p><note>
    /// <p>For a <code>DiscoveryType</code> of <code>CUSTOM</code>, the string-to-string map must have a key value of <code>Name</code> set to a non-empty-string.</p>
    /// </note>
    pub fn set_custom_protocol_detail(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_custom_protocol_detail(input);
        self
    }
    /// <p>Additional protocol-specific details required for device discovery, which vary based on the discovery type.</p><note>
    /// <p>For a <code>DiscoveryType</code> of <code>CUSTOM</code>, the string-to-string map must have a key value of <code>Name</code> set to a non-empty-string.</p>
    /// </note>
    pub fn get_custom_protocol_detail(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_custom_protocol_detail()
    }
    /// <p>The id of the end-user's IoT hub.</p>
    pub fn controller_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.controller_identifier(input.into());
        self
    }
    /// <p>The id of the end-user's IoT hub.</p>
    pub fn set_controller_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_controller_identifier(input);
        self
    }
    /// <p>The id of the end-user's IoT hub.</p>
    pub fn get_controller_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_controller_identifier()
    }
    /// <p>The id of the connector association.</p>
    #[deprecated(note = "ConnectorAssociationIdentifier is deprecated", since = "06-25-2025")]
    pub fn connector_association_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.connector_association_identifier(input.into());
        self
    }
    /// <p>The id of the connector association.</p>
    #[deprecated(note = "ConnectorAssociationIdentifier is deprecated", since = "06-25-2025")]
    pub fn set_connector_association_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_connector_association_identifier(input);
        self
    }
    /// <p>The id of the connector association.</p>
    #[deprecated(note = "ConnectorAssociationIdentifier is deprecated", since = "06-25-2025")]
    pub fn get_connector_association_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_connector_association_identifier()
    }
    /// <p>The identifier of the cloud-to-cloud account association to use for discovery of third-party devices.</p>
    pub fn account_association_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.account_association_id(input.into());
        self
    }
    /// <p>The identifier of the cloud-to-cloud account association to use for discovery of third-party devices.</p>
    pub fn set_account_association_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_account_association_id(input);
        self
    }
    /// <p>The identifier of the cloud-to-cloud account association to use for discovery of third-party devices.</p>
    pub fn get_account_association_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_account_association_id()
    }
    /// <p>The authentication material required to start the local device discovery job request.</p>
    pub fn authentication_material(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.authentication_material(input.into());
        self
    }
    /// <p>The authentication material required to start the local device discovery job request.</p>
    pub fn set_authentication_material(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_authentication_material(input);
        self
    }
    /// <p>The authentication material required to start the local device discovery job request.</p>
    pub fn get_authentication_material(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_authentication_material()
    }
    /// <p>The type of authentication material used for device discovery jobs.</p>
    pub fn authentication_material_type(mut self, input: crate::types::DiscoveryAuthMaterialType) -> Self {
        self.inner = self.inner.authentication_material_type(input);
        self
    }
    /// <p>The type of authentication material used for device discovery jobs.</p>
    pub fn set_authentication_material_type(mut self, input: ::std::option::Option<crate::types::DiscoveryAuthMaterialType>) -> Self {
        self.inner = self.inner.set_authentication_material_type(input);
        self
    }
    /// <p>The type of authentication material used for device discovery jobs.</p>
    pub fn get_authentication_material_type(&self) -> &::std::option::Option<crate::types::DiscoveryAuthMaterialType> {
        self.inner.get_authentication_material_type()
    }
    /// <p>An idempotency token. If you retry a request that completed successfully initially using the same client token and parameters, then the retry attempt will succeed without performing any further actions.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>An idempotency token. If you retry a request that completed successfully initially using the same client token and parameters, then the retry attempt will succeed without performing any further actions.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>An idempotency token. If you retry a request that completed successfully initially using the same client token and parameters, then the retry attempt will succeed without performing any further actions.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    ///
    /// Adds a key-value pair to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A set of key/value pairs that are used to manage the device discovery request.</p>
    #[deprecated(note = "Tags have been deprecated from this api", since = "06-25-2025")]
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>A set of key/value pairs that are used to manage the device discovery request.</p>
    #[deprecated(note = "Tags have been deprecated from this api", since = "06-25-2025")]
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A set of key/value pairs that are used to manage the device discovery request.</p>
    #[deprecated(note = "Tags have been deprecated from this api", since = "06-25-2025")]
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
