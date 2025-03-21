// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_alarm::_delete_alarm_output::DeleteAlarmOutputBuilder;

pub use crate::operation::delete_alarm::_delete_alarm_input::DeleteAlarmInputBuilder;

impl crate::operation::delete_alarm::builders::DeleteAlarmInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_alarm::DeleteAlarmOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_alarm::DeleteAlarmError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_alarm();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteAlarm`.
///
/// <p>Deletes an alarm.</p>
/// <p>An alarm is used to monitor a single metric for one of your resources. When a metric condition is met, the alarm can notify you by email, SMS text message, and a banner displayed on the Amazon Lightsail console. For more information, see <a href="https://docs.aws.amazon.com/lightsail/latest/userguide/amazon-lightsail-alarms">Alarms in Amazon Lightsail</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteAlarmFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_alarm::builders::DeleteAlarmInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_alarm::DeleteAlarmOutput,
        crate::operation::delete_alarm::DeleteAlarmError,
    > for DeleteAlarmFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_alarm::DeleteAlarmOutput,
            crate::operation::delete_alarm::DeleteAlarmError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteAlarmFluentBuilder {
    /// Creates a new `DeleteAlarmFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteAlarm as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_alarm::builders::DeleteAlarmInputBuilder {
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
        crate::operation::delete_alarm::DeleteAlarmOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_alarm::DeleteAlarmError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_alarm::DeleteAlarm::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_alarm::DeleteAlarm::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_alarm::DeleteAlarmOutput,
        crate::operation::delete_alarm::DeleteAlarmError,
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
    /// <p>The name of the alarm to delete.</p>
    pub fn alarm_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.alarm_name(input.into());
        self
    }
    /// <p>The name of the alarm to delete.</p>
    pub fn set_alarm_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_alarm_name(input);
        self
    }
    /// <p>The name of the alarm to delete.</p>
    pub fn get_alarm_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_alarm_name()
    }
}
