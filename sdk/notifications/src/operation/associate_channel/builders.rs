// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::associate_channel::_associate_channel_output::AssociateChannelOutputBuilder;

pub use crate::operation::associate_channel::_associate_channel_input::AssociateChannelInputBuilder;

impl crate::operation::associate_channel::builders::AssociateChannelInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::associate_channel::AssociateChannelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_channel::AssociateChannelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.associate_channel();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `AssociateChannel`.
///
/// <p>Associates a delivery <a href="https://docs.aws.amazon.com/notifications/latest/userguide/managing-delivery-channels.html">Channel</a> with a particular <code>NotificationConfiguration</code>. Supported Channels include Chatbot, the Console Mobile Application, and emails (notifications-contacts).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct AssociateChannelFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::associate_channel::builders::AssociateChannelInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::associate_channel::AssociateChannelOutput,
        crate::operation::associate_channel::AssociateChannelError,
    > for AssociateChannelFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::associate_channel::AssociateChannelOutput,
            crate::operation::associate_channel::AssociateChannelError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl AssociateChannelFluentBuilder {
    /// Creates a new `AssociateChannelFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the AssociateChannel as a reference.
    pub fn as_input(&self) -> &crate::operation::associate_channel::builders::AssociateChannelInputBuilder {
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
        crate::operation::associate_channel::AssociateChannelOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::associate_channel::AssociateChannelError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::associate_channel::AssociateChannel::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::associate_channel::AssociateChannel::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::associate_channel::AssociateChannelOutput,
        crate::operation::associate_channel::AssociateChannelError,
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
    /// <p>The Amazon Resource Name (ARN) of the Channel to associate with the <code>NotificationConfiguration</code>.</p>
    /// <p>Supported ARNs include Chatbot, the Console Mobile Application, and notifications-contacts.</p>
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Channel to associate with the <code>NotificationConfiguration</code>.</p>
    /// <p>Supported ARNs include Chatbot, the Console Mobile Application, and notifications-contacts.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Channel to associate with the <code>NotificationConfiguration</code>.</p>
    /// <p>Supported ARNs include Chatbot, the Console Mobile Application, and notifications-contacts.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_arn()
    }
    /// <p>The ARN of the <code>NotificationConfiguration</code> to associate with the Channel.</p>
    pub fn notification_configuration_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.notification_configuration_arn(input.into());
        self
    }
    /// <p>The ARN of the <code>NotificationConfiguration</code> to associate with the Channel.</p>
    pub fn set_notification_configuration_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_notification_configuration_arn(input);
        self
    }
    /// <p>The ARN of the <code>NotificationConfiguration</code> to associate with the Channel.</p>
    pub fn get_notification_configuration_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_notification_configuration_arn()
    }
}
