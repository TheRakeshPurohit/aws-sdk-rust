// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_ai_prompt::_update_ai_prompt_output::UpdateAiPromptOutputBuilder;

pub use crate::operation::update_ai_prompt::_update_ai_prompt_input::UpdateAiPromptInputBuilder;

impl crate::operation::update_ai_prompt::builders::UpdateAiPromptInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_ai_prompt::UpdateAiPromptOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_ai_prompt::UpdateAIPromptError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_ai_prompt();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateAIPrompt`.
///
/// <p>Updates an AI Prompt.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateAIPromptFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_ai_prompt::builders::UpdateAiPromptInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_ai_prompt::UpdateAiPromptOutput,
        crate::operation::update_ai_prompt::UpdateAIPromptError,
    > for UpdateAIPromptFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_ai_prompt::UpdateAiPromptOutput,
            crate::operation::update_ai_prompt::UpdateAIPromptError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateAIPromptFluentBuilder {
    /// Creates a new `UpdateAIPromptFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateAIPrompt as a reference.
    pub fn as_input(&self) -> &crate::operation::update_ai_prompt::builders::UpdateAiPromptInputBuilder {
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
        crate::operation::update_ai_prompt::UpdateAiPromptOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_ai_prompt::UpdateAIPromptError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_ai_prompt::UpdateAIPrompt::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_ai_prompt::UpdateAIPrompt::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_ai_prompt::UpdateAiPromptOutput,
        crate::operation::update_ai_prompt::UpdateAIPromptError,
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
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="http://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>..</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="http://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>..</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="http://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>..</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>The identifier of the Amazon Q in Connect assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn assistant_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.assistant_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Q in Connect assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn set_assistant_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_assistant_id(input);
        self
    }
    /// <p>The identifier of the Amazon Q in Connect assistant. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn get_assistant_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_assistant_id()
    }
    /// <p>The identifier of the Amazon Q in Connect AI Prompt.</p>
    pub fn ai_prompt_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.ai_prompt_id(input.into());
        self
    }
    /// <p>The identifier of the Amazon Q in Connect AI Prompt.</p>
    pub fn set_ai_prompt_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_ai_prompt_id(input);
        self
    }
    /// <p>The identifier of the Amazon Q in Connect AI Prompt.</p>
    pub fn get_ai_prompt_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_ai_prompt_id()
    }
    /// <p>The visibility status of the Amazon Q in Connect AI prompt.</p>
    pub fn visibility_status(mut self, input: crate::types::VisibilityStatus) -> Self {
        self.inner = self.inner.visibility_status(input);
        self
    }
    /// <p>The visibility status of the Amazon Q in Connect AI prompt.</p>
    pub fn set_visibility_status(mut self, input: ::std::option::Option<crate::types::VisibilityStatus>) -> Self {
        self.inner = self.inner.set_visibility_status(input);
        self
    }
    /// <p>The visibility status of the Amazon Q in Connect AI prompt.</p>
    pub fn get_visibility_status(&self) -> &::std::option::Option<crate::types::VisibilityStatus> {
        self.inner.get_visibility_status()
    }
    /// <p>The configuration of the prompt template for this AI Prompt.</p>
    pub fn template_configuration(mut self, input: crate::types::AiPromptTemplateConfiguration) -> Self {
        self.inner = self.inner.template_configuration(input);
        self
    }
    /// <p>The configuration of the prompt template for this AI Prompt.</p>
    pub fn set_template_configuration(mut self, input: ::std::option::Option<crate::types::AiPromptTemplateConfiguration>) -> Self {
        self.inner = self.inner.set_template_configuration(input);
        self
    }
    /// <p>The configuration of the prompt template for this AI Prompt.</p>
    pub fn get_template_configuration(&self) -> &::std::option::Option<crate::types::AiPromptTemplateConfiguration> {
        self.inner.get_template_configuration()
    }
    /// <p>The description of the Amazon Q in Connect AI Prompt.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the Amazon Q in Connect AI Prompt.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description of the Amazon Q in Connect AI Prompt.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
}
