// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_automation_rule_v2::_get_automation_rule_v2_output::GetAutomationRuleV2OutputBuilder;

pub use crate::operation::get_automation_rule_v2::_get_automation_rule_v2_input::GetAutomationRuleV2InputBuilder;

impl crate::operation::get_automation_rule_v2::builders::GetAutomationRuleV2InputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_automation_rule_v2::GetAutomationRuleV2Output,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_automation_rule_v2::GetAutomationRuleV2Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_automation_rule_v2();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetAutomationRuleV2`.
///
/// <p>Returns an automation rule for the V2 service. This API is in private preview and subject to change.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetAutomationRuleV2FluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_automation_rule_v2::builders::GetAutomationRuleV2InputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_automation_rule_v2::GetAutomationRuleV2Output,
        crate::operation::get_automation_rule_v2::GetAutomationRuleV2Error,
    > for GetAutomationRuleV2FluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_automation_rule_v2::GetAutomationRuleV2Output,
            crate::operation::get_automation_rule_v2::GetAutomationRuleV2Error,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetAutomationRuleV2FluentBuilder {
    /// Creates a new `GetAutomationRuleV2FluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetAutomationRuleV2 as a reference.
    pub fn as_input(&self) -> &crate::operation::get_automation_rule_v2::builders::GetAutomationRuleV2InputBuilder {
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
        crate::operation::get_automation_rule_v2::GetAutomationRuleV2Output,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_automation_rule_v2::GetAutomationRuleV2Error,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_automation_rule_v2::GetAutomationRuleV2::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_automation_rule_v2::GetAutomationRuleV2::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_automation_rule_v2::GetAutomationRuleV2Output,
        crate::operation::get_automation_rule_v2::GetAutomationRuleV2Error,
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
    /// <p>The ARN of the V2 automation rule.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identifier(input.into());
        self
    }
    /// <p>The ARN of the V2 automation rule.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identifier(input);
        self
    }
    /// <p>The ARN of the V2 automation rule.</p>
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identifier()
    }
}
