// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::invoke_data_automation_async::_invoke_data_automation_async_output::InvokeDataAutomationAsyncOutputBuilder;

pub use crate::operation::invoke_data_automation_async::_invoke_data_automation_async_input::InvokeDataAutomationAsyncInputBuilder;

impl crate::operation::invoke_data_automation_async::builders::InvokeDataAutomationAsyncInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::invoke_data_automation_async::InvokeDataAutomationAsyncOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::invoke_data_automation_async::InvokeDataAutomationAsyncError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.invoke_data_automation_async();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `InvokeDataAutomationAsync`.
///
/// Async API: Invoke data automation.
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct InvokeDataAutomationAsyncFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::invoke_data_automation_async::builders::InvokeDataAutomationAsyncInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::invoke_data_automation_async::InvokeDataAutomationAsyncOutput,
        crate::operation::invoke_data_automation_async::InvokeDataAutomationAsyncError,
    > for InvokeDataAutomationAsyncFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::invoke_data_automation_async::InvokeDataAutomationAsyncOutput,
            crate::operation::invoke_data_automation_async::InvokeDataAutomationAsyncError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl InvokeDataAutomationAsyncFluentBuilder {
    /// Creates a new `InvokeDataAutomationAsyncFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the InvokeDataAutomationAsync as a reference.
    pub fn as_input(&self) -> &crate::operation::invoke_data_automation_async::builders::InvokeDataAutomationAsyncInputBuilder {
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
        crate::operation::invoke_data_automation_async::InvokeDataAutomationAsyncOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::invoke_data_automation_async::InvokeDataAutomationAsyncError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::invoke_data_automation_async::InvokeDataAutomationAsync::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::invoke_data_automation_async::InvokeDataAutomationAsync::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::invoke_data_automation_async::InvokeDataAutomationAsyncOutput,
        crate::operation::invoke_data_automation_async::InvokeDataAutomationAsyncError,
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
    /// Idempotency token.
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// Idempotency token.
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// Idempotency token.
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// Input configuration.
    pub fn input_configuration(mut self, input: crate::types::InputConfiguration) -> Self {
        self.inner = self.inner.input_configuration(input);
        self
    }
    /// Input configuration.
    pub fn set_input_configuration(mut self, input: ::std::option::Option<crate::types::InputConfiguration>) -> Self {
        self.inner = self.inner.set_input_configuration(input);
        self
    }
    /// Input configuration.
    pub fn get_input_configuration(&self) -> &::std::option::Option<crate::types::InputConfiguration> {
        self.inner.get_input_configuration()
    }
    /// Output configuration.
    pub fn output_configuration(mut self, input: crate::types::OutputConfiguration) -> Self {
        self.inner = self.inner.output_configuration(input);
        self
    }
    /// Output configuration.
    pub fn set_output_configuration(mut self, input: ::std::option::Option<crate::types::OutputConfiguration>) -> Self {
        self.inner = self.inner.set_output_configuration(input);
        self
    }
    /// Output configuration.
    pub fn get_output_configuration(&self) -> &::std::option::Option<crate::types::OutputConfiguration> {
        self.inner.get_output_configuration()
    }
    /// Data automation configuration.
    pub fn data_automation_configuration(mut self, input: crate::types::DataAutomationConfiguration) -> Self {
        self.inner = self.inner.data_automation_configuration(input);
        self
    }
    /// Data automation configuration.
    pub fn set_data_automation_configuration(mut self, input: ::std::option::Option<crate::types::DataAutomationConfiguration>) -> Self {
        self.inner = self.inner.set_data_automation_configuration(input);
        self
    }
    /// Data automation configuration.
    pub fn get_data_automation_configuration(&self) -> &::std::option::Option<crate::types::DataAutomationConfiguration> {
        self.inner.get_data_automation_configuration()
    }
    /// Encryption configuration.
    pub fn encryption_configuration(mut self, input: crate::types::EncryptionConfiguration) -> Self {
        self.inner = self.inner.encryption_configuration(input);
        self
    }
    /// Encryption configuration.
    pub fn set_encryption_configuration(mut self, input: ::std::option::Option<crate::types::EncryptionConfiguration>) -> Self {
        self.inner = self.inner.set_encryption_configuration(input);
        self
    }
    /// Encryption configuration.
    pub fn get_encryption_configuration(&self) -> &::std::option::Option<crate::types::EncryptionConfiguration> {
        self.inner.get_encryption_configuration()
    }
    /// Notification configuration.
    pub fn notification_configuration(mut self, input: crate::types::NotificationConfiguration) -> Self {
        self.inner = self.inner.notification_configuration(input);
        self
    }
    /// Notification configuration.
    pub fn set_notification_configuration(mut self, input: ::std::option::Option<crate::types::NotificationConfiguration>) -> Self {
        self.inner = self.inner.set_notification_configuration(input);
        self
    }
    /// Notification configuration.
    pub fn get_notification_configuration(&self) -> &::std::option::Option<crate::types::NotificationConfiguration> {
        self.inner.get_notification_configuration()
    }
    ///
    /// Appends an item to `blueprints`.
    ///
    /// To override the contents of this collection use [`set_blueprints`](Self::set_blueprints).
    ///
    /// Blueprint list.
    pub fn blueprints(mut self, input: crate::types::Blueprint) -> Self {
        self.inner = self.inner.blueprints(input);
        self
    }
    /// Blueprint list.
    pub fn set_blueprints(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Blueprint>>) -> Self {
        self.inner = self.inner.set_blueprints(input);
        self
    }
    /// Blueprint list.
    pub fn get_blueprints(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Blueprint>> {
        self.inner.get_blueprints()
    }
    /// Data automation profile ARN
    pub fn data_automation_profile_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.data_automation_profile_arn(input.into());
        self
    }
    /// Data automation profile ARN
    pub fn set_data_automation_profile_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_data_automation_profile_arn(input);
        self
    }
    /// Data automation profile ARN
    pub fn get_data_automation_profile_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_data_automation_profile_arn()
    }
    ///
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// List of tags.
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// List of tags.
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// List of tags.
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
}
