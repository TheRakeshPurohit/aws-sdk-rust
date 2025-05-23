// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_connection_recording_preferences::_update_connection_recording_preferences_output::UpdateConnectionRecordingPreferencesOutputBuilder;

pub use crate::operation::update_connection_recording_preferences::_update_connection_recording_preferences_input::UpdateConnectionRecordingPreferencesInputBuilder;

impl crate::operation::update_connection_recording_preferences::builders::UpdateConnectionRecordingPreferencesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_connection_recording_preferences::UpdateConnectionRecordingPreferencesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_connection_recording_preferences::UpdateConnectionRecordingPreferencesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_connection_recording_preferences();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateConnectionRecordingPreferences`.
///
/// <p>Updates the preferences for recording RDP connections.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateConnectionRecordingPreferencesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_connection_recording_preferences::builders::UpdateConnectionRecordingPreferencesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_connection_recording_preferences::UpdateConnectionRecordingPreferencesOutput,
        crate::operation::update_connection_recording_preferences::UpdateConnectionRecordingPreferencesError,
    > for UpdateConnectionRecordingPreferencesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_connection_recording_preferences::UpdateConnectionRecordingPreferencesOutput,
            crate::operation::update_connection_recording_preferences::UpdateConnectionRecordingPreferencesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateConnectionRecordingPreferencesFluentBuilder {
    /// Creates a new `UpdateConnectionRecordingPreferencesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateConnectionRecordingPreferences as a reference.
    pub fn as_input(&self) -> &crate::operation::update_connection_recording_preferences::builders::UpdateConnectionRecordingPreferencesInputBuilder {
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
        crate::operation::update_connection_recording_preferences::UpdateConnectionRecordingPreferencesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_connection_recording_preferences::UpdateConnectionRecordingPreferencesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::update_connection_recording_preferences::UpdateConnectionRecordingPreferences::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::update_connection_recording_preferences::UpdateConnectionRecordingPreferences::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_connection_recording_preferences::UpdateConnectionRecordingPreferencesOutput,
        crate::operation::update_connection_recording_preferences::UpdateConnectionRecordingPreferencesError,
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
    /// <p>The set of preferences used for recording RDP connections in the requesting Amazon Web Services account and Amazon Web Services Region. This includes details such as which S3 bucket recordings are stored in.</p>
    pub fn connection_recording_preferences(mut self, input: crate::types::ConnectionRecordingPreferences) -> Self {
        self.inner = self.inner.connection_recording_preferences(input);
        self
    }
    /// <p>The set of preferences used for recording RDP connections in the requesting Amazon Web Services account and Amazon Web Services Region. This includes details such as which S3 bucket recordings are stored in.</p>
    pub fn set_connection_recording_preferences(mut self, input: ::std::option::Option<crate::types::ConnectionRecordingPreferences>) -> Self {
        self.inner = self.inner.set_connection_recording_preferences(input);
        self
    }
    /// <p>The set of preferences used for recording RDP connections in the requesting Amazon Web Services account and Amazon Web Services Region. This includes details such as which S3 bucket recordings are stored in.</p>
    pub fn get_connection_recording_preferences(&self) -> &::std::option::Option<crate::types::ConnectionRecordingPreferences> {
        self.inner.get_connection_recording_preferences()
    }
    /// <p>User-provided idempotency token.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>User-provided idempotency token.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>User-provided idempotency token.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
}
