// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_link::_update_link_output::UpdateLinkOutputBuilder;

pub use crate::operation::update_link::_update_link_input::UpdateLinkInputBuilder;

impl crate::operation::update_link::builders::UpdateLinkInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_link::UpdateLinkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_link::UpdateLinkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_link();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateLink`.
///
/// <p>Use this operation to change what types of data are shared from a source account to its linked monitoring account sink. You can't change the sink or change the monitoring account with this operation.</p>
/// <p>When you update a link, you can optionally specify filters that specify which metric namespaces and which log groups are shared from the source account to the monitoring account.</p>
/// <p>To update the list of tags associated with the sink, use <a href="https://docs.aws.amazon.com/OAM/latest/APIReference/API_TagResource.html">TagResource</a>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateLinkFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_link::builders::UpdateLinkInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_link::UpdateLinkOutput,
        crate::operation::update_link::UpdateLinkError,
    > for UpdateLinkFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_link::UpdateLinkOutput,
            crate::operation::update_link::UpdateLinkError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateLinkFluentBuilder {
    /// Creates a new `UpdateLinkFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateLink as a reference.
    pub fn as_input(&self) -> &crate::operation::update_link::builders::UpdateLinkInputBuilder {
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
        crate::operation::update_link::UpdateLinkOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_link::UpdateLinkError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_link::UpdateLink::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_link::UpdateLink::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_link::UpdateLinkOutput,
        crate::operation::update_link::UpdateLinkError,
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
    /// <p>The ARN of the link that you want to update.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identifier(input.into());
        self
    }
    /// <p>The ARN of the link that you want to update.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identifier(input);
        self
    }
    /// <p>The ARN of the link that you want to update.</p>
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identifier()
    }
    ///
    /// Appends an item to `ResourceTypes`.
    ///
    /// To override the contents of this collection use [`set_resource_types`](Self::set_resource_types).
    ///
    /// <p>An array of strings that define which types of data that the source account will send to the monitoring account.</p>
    /// <p>Your input here replaces the current set of data types that are shared.</p>
    pub fn resource_types(mut self, input: crate::types::ResourceType) -> Self {
        self.inner = self.inner.resource_types(input);
        self
    }
    /// <p>An array of strings that define which types of data that the source account will send to the monitoring account.</p>
    /// <p>Your input here replaces the current set of data types that are shared.</p>
    pub fn set_resource_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ResourceType>>) -> Self {
        self.inner = self.inner.set_resource_types(input);
        self
    }
    /// <p>An array of strings that define which types of data that the source account will send to the monitoring account.</p>
    /// <p>Your input here replaces the current set of data types that are shared.</p>
    pub fn get_resource_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ResourceType>> {
        self.inner.get_resource_types()
    }
    /// <p>Use this structure to filter which metric namespaces and which log groups are to be shared from the source account to the monitoring account.</p>
    pub fn link_configuration(mut self, input: crate::types::LinkConfiguration) -> Self {
        self.inner = self.inner.link_configuration(input);
        self
    }
    /// <p>Use this structure to filter which metric namespaces and which log groups are to be shared from the source account to the monitoring account.</p>
    pub fn set_link_configuration(mut self, input: ::std::option::Option<crate::types::LinkConfiguration>) -> Self {
        self.inner = self.inner.set_link_configuration(input);
        self
    }
    /// <p>Use this structure to filter which metric namespaces and which log groups are to be shared from the source account to the monitoring account.</p>
    pub fn get_link_configuration(&self) -> &::std::option::Option<crate::types::LinkConfiguration> {
        self.inner.get_link_configuration()
    }
    /// <p>Specifies whether to include the tags associated with the link in the response after the update operation. When <code>IncludeTags</code> is set to <code>true</code> and the caller has the required permission, <code>oam:ListTagsForResource</code>, the API will return the tags for the specified resource. If the caller doesn't have the required permission, <code>oam:ListTagsForResource</code>, the API will raise an exception.</p>
    /// <p>The default value is <code>false</code>.</p>
    pub fn include_tags(mut self, input: bool) -> Self {
        self.inner = self.inner.include_tags(input);
        self
    }
    /// <p>Specifies whether to include the tags associated with the link in the response after the update operation. When <code>IncludeTags</code> is set to <code>true</code> and the caller has the required permission, <code>oam:ListTagsForResource</code>, the API will return the tags for the specified resource. If the caller doesn't have the required permission, <code>oam:ListTagsForResource</code>, the API will raise an exception.</p>
    /// <p>The default value is <code>false</code>.</p>
    pub fn set_include_tags(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_include_tags(input);
        self
    }
    /// <p>Specifies whether to include the tags associated with the link in the response after the update operation. When <code>IncludeTags</code> is set to <code>true</code> and the caller has the required permission, <code>oam:ListTagsForResource</code>, the API will return the tags for the specified resource. If the caller doesn't have the required permission, <code>oam:ListTagsForResource</code>, the API will raise an exception.</p>
    /// <p>The default value is <code>false</code>.</p>
    pub fn get_include_tags(&self) -> &::std::option::Option<bool> {
        self.inner.get_include_tags()
    }
}
