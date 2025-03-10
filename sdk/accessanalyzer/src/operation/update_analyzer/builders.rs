// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_analyzer::_update_analyzer_output::UpdateAnalyzerOutputBuilder;

pub use crate::operation::update_analyzer::_update_analyzer_input::UpdateAnalyzerInputBuilder;

impl crate::operation::update_analyzer::builders::UpdateAnalyzerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_analyzer::UpdateAnalyzerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_analyzer::UpdateAnalyzerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_analyzer();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateAnalyzer`.
///
/// <p>Modifies the configuration of an existing analyzer.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateAnalyzerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_analyzer::builders::UpdateAnalyzerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_analyzer::UpdateAnalyzerOutput,
        crate::operation::update_analyzer::UpdateAnalyzerError,
    > for UpdateAnalyzerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_analyzer::UpdateAnalyzerOutput,
            crate::operation::update_analyzer::UpdateAnalyzerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateAnalyzerFluentBuilder {
    /// Creates a new `UpdateAnalyzerFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateAnalyzer as a reference.
    pub fn as_input(&self) -> &crate::operation::update_analyzer::builders::UpdateAnalyzerInputBuilder {
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
        crate::operation::update_analyzer::UpdateAnalyzerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_analyzer::UpdateAnalyzerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_analyzer::UpdateAnalyzer::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_analyzer::UpdateAnalyzer::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_analyzer::UpdateAnalyzerOutput,
        crate::operation::update_analyzer::UpdateAnalyzerError,
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
    /// <p>The name of the analyzer to modify.</p>
    pub fn analyzer_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.analyzer_name(input.into());
        self
    }
    /// <p>The name of the analyzer to modify.</p>
    pub fn set_analyzer_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_analyzer_name(input);
        self
    }
    /// <p>The name of the analyzer to modify.</p>
    pub fn get_analyzer_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_analyzer_name()
    }
    /// <p>Contains information about the configuration of an analyzer for an Amazon Web Services organization or account.</p>
    pub fn configuration(mut self, input: crate::types::AnalyzerConfiguration) -> Self {
        self.inner = self.inner.configuration(input);
        self
    }
    /// <p>Contains information about the configuration of an analyzer for an Amazon Web Services organization or account.</p>
    pub fn set_configuration(mut self, input: ::std::option::Option<crate::types::AnalyzerConfiguration>) -> Self {
        self.inner = self.inner.set_configuration(input);
        self
    }
    /// <p>Contains information about the configuration of an analyzer for an Amazon Web Services organization or account.</p>
    pub fn get_configuration(&self) -> &::std::option::Option<crate::types::AnalyzerConfiguration> {
        self.inner.get_configuration()
    }
}
