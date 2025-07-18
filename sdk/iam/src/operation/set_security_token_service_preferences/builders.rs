// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::set_security_token_service_preferences::_set_security_token_service_preferences_output::SetSecurityTokenServicePreferencesOutputBuilder;

pub use crate::operation::set_security_token_service_preferences::_set_security_token_service_preferences_input::SetSecurityTokenServicePreferencesInputBuilder;

impl crate::operation::set_security_token_service_preferences::builders::SetSecurityTokenServicePreferencesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.set_security_token_service_preferences();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `SetSecurityTokenServicePreferences`.
///
/// <p>Sets the specified version of the global endpoint token as the token version used for the Amazon Web Services account.</p>
/// <p>By default, Security Token Service (STS) is available as a global service, and all STS requests go to a single endpoint at <code>https://sts.amazonaws.com</code>. Amazon Web Services recommends using Regional STS endpoints to reduce latency, build in redundancy, and increase session token availability. For information about Regional endpoints for STS, see <a href="https://docs.aws.amazon.com/general/latest/gr/sts.html">Security Token Service endpoints and quotas</a> in the <i>Amazon Web Services General Reference</i>.</p>
/// <p>If you make an STS call to the global endpoint, the resulting session tokens might be valid in some Regions but not others. It depends on the version that is set in this operation. Version 1 tokens are valid only in Amazon Web Services Regions that are available by default. These tokens do not work in manually enabled Regions, such as Asia Pacific (Hong Kong). Version 2 tokens are valid in all Regions. However, version 2 tokens are longer and might affect systems where you temporarily store tokens. For information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
/// <p>To view the current session token version, see the <code>GlobalEndpointTokenVersion</code> entry in the response of the <a href="https://docs.aws.amazon.com/IAM/latest/APIReference/API_GetAccountSummary.html">GetAccountSummary</a> operation.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SetSecurityTokenServicePreferencesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::set_security_token_service_preferences::builders::SetSecurityTokenServicePreferencesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesOutput,
        crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesError,
    > for SetSecurityTokenServicePreferencesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesOutput,
            crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl SetSecurityTokenServicePreferencesFluentBuilder {
    /// Creates a new `SetSecurityTokenServicePreferencesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the SetSecurityTokenServicePreferences as a reference.
    pub fn as_input(&self) -> &crate::operation::set_security_token_service_preferences::builders::SetSecurityTokenServicePreferencesInputBuilder {
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
        crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferences::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferences::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesOutput,
        crate::operation::set_security_token_service_preferences::SetSecurityTokenServicePreferencesError,
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
    /// <p>The version of the global endpoint token. Version 1 tokens are valid only in Amazon Web Services Regions that are available by default. These tokens do not work in manually enabled Regions, such as Asia Pacific (Hong Kong). Version 2 tokens are valid in all Regions. However, version 2 tokens are longer and might affect systems where you temporarily store tokens.</p>
    /// <p>For information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
    pub fn global_endpoint_token_version(mut self, input: crate::types::GlobalEndpointTokenVersion) -> Self {
        self.inner = self.inner.global_endpoint_token_version(input);
        self
    }
    /// <p>The version of the global endpoint token. Version 1 tokens are valid only in Amazon Web Services Regions that are available by default. These tokens do not work in manually enabled Regions, such as Asia Pacific (Hong Kong). Version 2 tokens are valid in all Regions. However, version 2 tokens are longer and might affect systems where you temporarily store tokens.</p>
    /// <p>For information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
    pub fn set_global_endpoint_token_version(mut self, input: ::std::option::Option<crate::types::GlobalEndpointTokenVersion>) -> Self {
        self.inner = self.inner.set_global_endpoint_token_version(input);
        self
    }
    /// <p>The version of the global endpoint token. Version 1 tokens are valid only in Amazon Web Services Regions that are available by default. These tokens do not work in manually enabled Regions, such as Asia Pacific (Hong Kong). Version 2 tokens are valid in all Regions. However, version 2 tokens are longer and might affect systems where you temporarily store tokens.</p>
    /// <p>For information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_credentials_temp_enable-regions.html">Activating and deactivating STS in an Amazon Web Services Region</a> in the <i>IAM User Guide</i>.</p>
    pub fn get_global_endpoint_token_version(&self) -> &::std::option::Option<crate::types::GlobalEndpointTokenVersion> {
        self.inner.get_global_endpoint_token_version()
    }
}
