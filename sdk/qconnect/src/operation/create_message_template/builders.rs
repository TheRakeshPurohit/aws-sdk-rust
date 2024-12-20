// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_message_template::_create_message_template_output::CreateMessageTemplateOutputBuilder;

pub use crate::operation::create_message_template::_create_message_template_input::CreateMessageTemplateInputBuilder;

impl crate::operation::create_message_template::builders::CreateMessageTemplateInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_message_template::CreateMessageTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_message_template::CreateMessageTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_message_template();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateMessageTemplate`.
///
/// <p>Creates an Amazon Q in Connect message template. The name of the message template has to be unique for each knowledge base. The channel subtype of the message template is immutable and cannot be modified after creation. After the message template is created, you can use the <code>$LATEST</code> qualifier to reference the created message template.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateMessageTemplateFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_message_template::builders::CreateMessageTemplateInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_message_template::CreateMessageTemplateOutput,
        crate::operation::create_message_template::CreateMessageTemplateError,
    > for CreateMessageTemplateFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_message_template::CreateMessageTemplateOutput,
            crate::operation::create_message_template::CreateMessageTemplateError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateMessageTemplateFluentBuilder {
    /// Creates a new `CreateMessageTemplateFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateMessageTemplate as a reference.
    pub fn as_input(&self) -> &crate::operation::create_message_template::builders::CreateMessageTemplateInputBuilder {
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
        crate::operation::create_message_template::CreateMessageTemplateOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_message_template::CreateMessageTemplateError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_message_template::CreateMessageTemplate::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_message_template::CreateMessageTemplate::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_message_template::CreateMessageTemplateOutput,
        crate::operation::create_message_template::CreateMessageTemplateError,
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
    /// <p>The identifier of the knowledge base. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn knowledge_base_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.knowledge_base_id(input.into());
        self
    }
    /// <p>The identifier of the knowledge base. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn set_knowledge_base_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_knowledge_base_id(input);
        self
    }
    /// <p>The identifier of the knowledge base. Can be either the ID or the ARN. URLs cannot contain the ARN.</p>
    pub fn get_knowledge_base_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_knowledge_base_id()
    }
    /// <p>The name of the message template.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the message template.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the message template.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The content of the message template.</p>
    pub fn content(mut self, input: crate::types::MessageTemplateContentProvider) -> Self {
        self.inner = self.inner.content(input);
        self
    }
    /// <p>The content of the message template.</p>
    pub fn set_content(mut self, input: ::std::option::Option<crate::types::MessageTemplateContentProvider>) -> Self {
        self.inner = self.inner.set_content(input);
        self
    }
    /// <p>The content of the message template.</p>
    pub fn get_content(&self) -> &::std::option::Option<crate::types::MessageTemplateContentProvider> {
        self.inner.get_content()
    }
    /// <p>The description of the message template.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description of the message template.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description of the message template.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The channel subtype this message template applies to.</p>
    pub fn channel_subtype(mut self, input: crate::types::ChannelSubtype) -> Self {
        self.inner = self.inner.channel_subtype(input);
        self
    }
    /// <p>The channel subtype this message template applies to.</p>
    pub fn set_channel_subtype(mut self, input: ::std::option::Option<crate::types::ChannelSubtype>) -> Self {
        self.inner = self.inner.set_channel_subtype(input);
        self
    }
    /// <p>The channel subtype this message template applies to.</p>
    pub fn get_channel_subtype(&self) -> &::std::option::Option<crate::types::ChannelSubtype> {
        self.inner.get_channel_subtype()
    }
    /// <p>The language code value for the language in which the quick response is written. The supported language codes include <code>de_DE</code>, <code>en_US</code>, <code>es_ES</code>, <code>fr_FR</code>, <code>id_ID</code>, <code>it_IT</code>, <code>ja_JP</code>, <code>ko_KR</code>, <code>pt_BR</code>, <code>zh_CN</code>, <code>zh_TW</code></p>
    pub fn language(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.language(input.into());
        self
    }
    /// <p>The language code value for the language in which the quick response is written. The supported language codes include <code>de_DE</code>, <code>en_US</code>, <code>es_ES</code>, <code>fr_FR</code>, <code>id_ID</code>, <code>it_IT</code>, <code>ja_JP</code>, <code>ko_KR</code>, <code>pt_BR</code>, <code>zh_CN</code>, <code>zh_TW</code></p>
    pub fn set_language(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_language(input);
        self
    }
    /// <p>The language code value for the language in which the quick response is written. The supported language codes include <code>de_DE</code>, <code>en_US</code>, <code>es_ES</code>, <code>fr_FR</code>, <code>id_ID</code>, <code>it_IT</code>, <code>ja_JP</code>, <code>ko_KR</code>, <code>pt_BR</code>, <code>zh_CN</code>, <code>zh_TW</code></p>
    pub fn get_language(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_language()
    }
    /// <p>An object that specifies the default values to use for variables in the message template. This object contains different categories of key-value pairs. Each key defines a variable or placeholder in the message template. The corresponding value defines the default value for that variable.</p>
    pub fn default_attributes(mut self, input: crate::types::MessageTemplateAttributes) -> Self {
        self.inner = self.inner.default_attributes(input);
        self
    }
    /// <p>An object that specifies the default values to use for variables in the message template. This object contains different categories of key-value pairs. Each key defines a variable or placeholder in the message template. The corresponding value defines the default value for that variable.</p>
    pub fn set_default_attributes(mut self, input: ::std::option::Option<crate::types::MessageTemplateAttributes>) -> Self {
        self.inner = self.inner.set_default_attributes(input);
        self
    }
    /// <p>An object that specifies the default values to use for variables in the message template. This object contains different categories of key-value pairs. Each key defines a variable or placeholder in the message template. The corresponding value defines the default value for that variable.</p>
    pub fn get_default_attributes(&self) -> &::std::option::Option<crate::types::MessageTemplateAttributes> {
        self.inner.get_default_attributes()
    }
    /// <p>The configuration information of the grouping of Amazon Q in Connect users.</p>
    pub fn grouping_configuration(mut self, input: crate::types::GroupingConfiguration) -> Self {
        self.inner = self.inner.grouping_configuration(input);
        self
    }
    /// <p>The configuration information of the grouping of Amazon Q in Connect users.</p>
    pub fn set_grouping_configuration(mut self, input: ::std::option::Option<crate::types::GroupingConfiguration>) -> Self {
        self.inner = self.inner.set_grouping_configuration(input);
        self
    }
    /// <p>The configuration information of the grouping of Amazon Q in Connect users.</p>
    pub fn get_grouping_configuration(&self) -> &::std::option::Option<crate::types::GroupingConfiguration> {
        self.inner.get_grouping_configuration()
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="http://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="http://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>A unique, case-sensitive identifier that you provide to ensure the idempotency of the request. If not provided, the Amazon Web Services SDK populates this field. For more information about idempotency, see <a href="http://aws.amazon.com/builders-library/making-retries-safe-with-idempotent-APIs/">Making retries safe with idempotent APIs</a>.</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    ///
    /// Adds a key-value pair to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn tags(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.tags(k.into(), v.into());
        self
    }
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>The tags used to organize, track, or control access for this resource.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_tags()
    }
}
