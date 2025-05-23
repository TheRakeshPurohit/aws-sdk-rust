// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_data_lake_exception_subscription::_update_data_lake_exception_subscription_output::UpdateDataLakeExceptionSubscriptionOutputBuilder;

pub use crate::operation::update_data_lake_exception_subscription::_update_data_lake_exception_subscription_input::UpdateDataLakeExceptionSubscriptionInputBuilder;

impl crate::operation::update_data_lake_exception_subscription::builders::UpdateDataLakeExceptionSubscriptionInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_data_lake_exception_subscription();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateDataLakeExceptionSubscription`.
///
/// <p>Updates the specified notification subscription in Amazon Security Lake for the organization you specify.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateDataLakeExceptionSubscriptionFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_data_lake_exception_subscription::builders::UpdateDataLakeExceptionSubscriptionInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscriptionOutput,
        crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscriptionError,
    > for UpdateDataLakeExceptionSubscriptionFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscriptionOutput,
            crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscriptionError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateDataLakeExceptionSubscriptionFluentBuilder {
    /// Creates a new `UpdateDataLakeExceptionSubscriptionFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateDataLakeExceptionSubscription as a reference.
    pub fn as_input(&self) -> &crate::operation::update_data_lake_exception_subscription::builders::UpdateDataLakeExceptionSubscriptionInputBuilder {
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
        crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscriptionOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscriptionError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins =
            crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscription::operation_runtime_plugins(
                self.handle.runtime_plugins.clone(),
                &self.handle.conf,
                self.config_override,
            );
        crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscription::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscriptionOutput,
        crate::operation::update_data_lake_exception_subscription::UpdateDataLakeExceptionSubscriptionError,
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
    /// <p>The subscription protocol to which exception messages are posted.</p>
    pub fn subscription_protocol(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.subscription_protocol(input.into());
        self
    }
    /// <p>The subscription protocol to which exception messages are posted.</p>
    pub fn set_subscription_protocol(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_subscription_protocol(input);
        self
    }
    /// <p>The subscription protocol to which exception messages are posted.</p>
    pub fn get_subscription_protocol(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_subscription_protocol()
    }
    /// <p>The account that is subscribed to receive exception notifications.</p>
    pub fn notification_endpoint(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.notification_endpoint(input.into());
        self
    }
    /// <p>The account that is subscribed to receive exception notifications.</p>
    pub fn set_notification_endpoint(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_notification_endpoint(input);
        self
    }
    /// <p>The account that is subscribed to receive exception notifications.</p>
    pub fn get_notification_endpoint(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_notification_endpoint()
    }
    /// <p>The time-to-live (TTL) for the exception message to remain. It is the duration of time until which the exception message remains.</p>
    pub fn exception_time_to_live(mut self, input: i64) -> Self {
        self.inner = self.inner.exception_time_to_live(input);
        self
    }
    /// <p>The time-to-live (TTL) for the exception message to remain. It is the duration of time until which the exception message remains.</p>
    pub fn set_exception_time_to_live(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_exception_time_to_live(input);
        self
    }
    /// <p>The time-to-live (TTL) for the exception message to remain. It is the duration of time until which the exception message remains.</p>
    pub fn get_exception_time_to_live(&self) -> &::std::option::Option<i64> {
        self.inner.get_exception_time_to_live()
    }
}
