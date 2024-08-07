// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_quick_setup_types::_list_quick_setup_types_output::ListQuickSetupTypesOutputBuilder;

pub use crate::operation::list_quick_setup_types::_list_quick_setup_types_input::ListQuickSetupTypesInputBuilder;

impl crate::operation::list_quick_setup_types::builders::ListQuickSetupTypesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_quick_setup_types::ListQuickSetupTypesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_quick_setup_types::ListQuickSetupTypesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_quick_setup_types();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListQuickSetupTypes`.
///
/// <p>Returns the available Quick Setup types.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListQuickSetupTypesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_quick_setup_types::builders::ListQuickSetupTypesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_quick_setup_types::ListQuickSetupTypesOutput,
        crate::operation::list_quick_setup_types::ListQuickSetupTypesError,
    > for ListQuickSetupTypesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_quick_setup_types::ListQuickSetupTypesOutput,
            crate::operation::list_quick_setup_types::ListQuickSetupTypesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListQuickSetupTypesFluentBuilder {
    /// Creates a new `ListQuickSetupTypesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListQuickSetupTypes as a reference.
    pub fn as_input(&self) -> &crate::operation::list_quick_setup_types::builders::ListQuickSetupTypesInputBuilder {
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
        crate::operation::list_quick_setup_types::ListQuickSetupTypesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_quick_setup_types::ListQuickSetupTypesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_quick_setup_types::ListQuickSetupTypes::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_quick_setup_types::ListQuickSetupTypes::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_quick_setup_types::ListQuickSetupTypesOutput,
        crate::operation::list_quick_setup_types::ListQuickSetupTypesError,
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
}
