// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_project::_update_project_output::UpdateProjectOutputBuilder;

pub use crate::operation::update_project::_update_project_input::UpdateProjectInputBuilder;

impl crate::operation::update_project::builders::UpdateProjectInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_project::UpdateProjectOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_project::UpdateProjectError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_project();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateProject`.
///
/// <p>Updates the specified project in Amazon DataZone.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateProjectFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_project::builders::UpdateProjectInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_project::UpdateProjectOutput,
        crate::operation::update_project::UpdateProjectError,
    > for UpdateProjectFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_project::UpdateProjectOutput,
            crate::operation::update_project::UpdateProjectError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateProjectFluentBuilder {
    /// Creates a new `UpdateProjectFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateProject as a reference.
    pub fn as_input(&self) -> &crate::operation::update_project::builders::UpdateProjectInputBuilder {
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
        crate::operation::update_project::UpdateProjectOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_project::UpdateProjectError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_project::UpdateProject::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_project::UpdateProject::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_project::UpdateProjectOutput,
        crate::operation::update_project::UpdateProjectError,
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
    /// <p>The ID of the Amazon DataZone domain where a project is being updated.</p>
    pub fn domain_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_identifier(input.into());
        self
    }
    /// <p>The ID of the Amazon DataZone domain where a project is being updated.</p>
    pub fn set_domain_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_identifier(input);
        self
    }
    /// <p>The ID of the Amazon DataZone domain where a project is being updated.</p>
    pub fn get_domain_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_identifier()
    }
    /// <p>The identifier of the project that is to be updated.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identifier(input.into());
        self
    }
    /// <p>The identifier of the project that is to be updated.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identifier(input);
        self
    }
    /// <p>The identifier of the project that is to be updated.</p>
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identifier()
    }
    /// <p>The name to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>The description to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>The description to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>The description to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    ///
    /// Appends an item to `glossaryTerms`.
    ///
    /// To override the contents of this collection use [`set_glossary_terms`](Self::set_glossary_terms).
    ///
    /// <p>The glossary terms to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn glossary_terms(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.glossary_terms(input.into());
        self
    }
    /// <p>The glossary terms to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn set_glossary_terms(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_glossary_terms(input);
        self
    }
    /// <p>The glossary terms to be updated as part of the <code>UpdateProject</code> action.</p>
    pub fn get_glossary_terms(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_glossary_terms()
    }
    /// <p>The ID of the domain unit.</p>
    pub fn domain_unit_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.domain_unit_id(input.into());
        self
    }
    /// <p>The ID of the domain unit.</p>
    pub fn set_domain_unit_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_domain_unit_id(input);
        self
    }
    /// <p>The ID of the domain unit.</p>
    pub fn get_domain_unit_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_domain_unit_id()
    }
    /// <p>The environment deployment details of the project.</p>
    pub fn environment_deployment_details(mut self, input: crate::types::EnvironmentDeploymentDetails) -> Self {
        self.inner = self.inner.environment_deployment_details(input);
        self
    }
    /// <p>The environment deployment details of the project.</p>
    pub fn set_environment_deployment_details(mut self, input: ::std::option::Option<crate::types::EnvironmentDeploymentDetails>) -> Self {
        self.inner = self.inner.set_environment_deployment_details(input);
        self
    }
    /// <p>The environment deployment details of the project.</p>
    pub fn get_environment_deployment_details(&self) -> &::std::option::Option<crate::types::EnvironmentDeploymentDetails> {
        self.inner.get_environment_deployment_details()
    }
    ///
    /// Appends an item to `userParameters`.
    ///
    /// To override the contents of this collection use [`set_user_parameters`](Self::set_user_parameters).
    ///
    /// <p>The user parameters of the project.</p>
    pub fn user_parameters(mut self, input: crate::types::EnvironmentConfigurationUserParameter) -> Self {
        self.inner = self.inner.user_parameters(input);
        self
    }
    /// <p>The user parameters of the project.</p>
    pub fn set_user_parameters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::EnvironmentConfigurationUserParameter>>) -> Self {
        self.inner = self.inner.set_user_parameters(input);
        self
    }
    /// <p>The user parameters of the project.</p>
    pub fn get_user_parameters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::EnvironmentConfigurationUserParameter>> {
        self.inner.get_user_parameters()
    }
    /// <p>The project profile version to which the project should be updated. You can only specify the following string for this parameter: <code>latest</code>.</p>
    pub fn project_profile_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.project_profile_version(input.into());
        self
    }
    /// <p>The project profile version to which the project should be updated. You can only specify the following string for this parameter: <code>latest</code>.</p>
    pub fn set_project_profile_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_project_profile_version(input);
        self
    }
    /// <p>The project profile version to which the project should be updated. You can only specify the following string for this parameter: <code>latest</code>.</p>
    pub fn get_project_profile_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_project_profile_version()
    }
}
