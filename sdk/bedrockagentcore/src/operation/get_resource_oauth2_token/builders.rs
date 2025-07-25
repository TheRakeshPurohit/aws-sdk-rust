// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_resource_oauth2_token::_get_resource_oauth2_token_output::GetResourceOauth2TokenOutputBuilder;

pub use crate::operation::get_resource_oauth2_token::_get_resource_oauth2_token_input::GetResourceOauth2TokenInputBuilder;

impl crate::operation::get_resource_oauth2_token::builders::GetResourceOauth2TokenInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_resource_oauth2_token::GetResourceOauth2TokenOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_resource_oauth2_token::GetResourceOauth2TokenError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_resource_oauth2_token();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetResourceOauth2Token`.
///
/// <p>Reaturns the Oauth2Token of the provided resource</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetResourceOauth2TokenFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_resource_oauth2_token::builders::GetResourceOauth2TokenInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_resource_oauth2_token::GetResourceOauth2TokenOutput,
        crate::operation::get_resource_oauth2_token::GetResourceOauth2TokenError,
    > for GetResourceOauth2TokenFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_resource_oauth2_token::GetResourceOauth2TokenOutput,
            crate::operation::get_resource_oauth2_token::GetResourceOauth2TokenError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetResourceOauth2TokenFluentBuilder {
    /// Creates a new `GetResourceOauth2TokenFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetResourceOauth2Token as a reference.
    pub fn as_input(&self) -> &crate::operation::get_resource_oauth2_token::builders::GetResourceOauth2TokenInputBuilder {
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
        crate::operation::get_resource_oauth2_token::GetResourceOauth2TokenOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_resource_oauth2_token::GetResourceOauth2TokenError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_resource_oauth2_token::GetResourceOauth2Token::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_resource_oauth2_token::GetResourceOauth2Token::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_resource_oauth2_token::GetResourceOauth2TokenOutput,
        crate::operation::get_resource_oauth2_token::GetResourceOauth2TokenError,
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
    /// <p>The identity token of the workload you want to retrive the Oauth2 Token of.</p>
    pub fn workload_identity_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.workload_identity_token(input.into());
        self
    }
    /// <p>The identity token of the workload you want to retrive the Oauth2 Token of.</p>
    pub fn set_workload_identity_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_workload_identity_token(input);
        self
    }
    /// <p>The identity token of the workload you want to retrive the Oauth2 Token of.</p>
    pub fn get_workload_identity_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_workload_identity_token()
    }
    /// <p>The user ID of the user you're retrieving the token on behalf of.</p>
    pub fn user_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.user_id(input.into());
        self
    }
    /// <p>The user ID of the user you're retrieving the token on behalf of.</p>
    pub fn set_user_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_user_id(input);
        self
    }
    /// <p>The user ID of the user you're retrieving the token on behalf of.</p>
    pub fn get_user_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_user_id()
    }
    /// <p>Reference to the credential provider</p>
    pub fn resource_credential_provider_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_credential_provider_name(input.into());
        self
    }
    /// <p>Reference to the credential provider</p>
    pub fn set_resource_credential_provider_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_credential_provider_name(input);
        self
    }
    /// <p>Reference to the credential provider</p>
    pub fn get_resource_credential_provider_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_credential_provider_name()
    }
    ///
    /// Appends an item to `scopes`.
    ///
    /// To override the contents of this collection use [`set_scopes`](Self::set_scopes).
    ///
    /// <p>The OAuth scopes requested</p>
    pub fn scopes(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.scopes(input.into());
        self
    }
    /// <p>The OAuth scopes requested</p>
    pub fn set_scopes(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_scopes(input);
        self
    }
    /// <p>The OAuth scopes requested</p>
    pub fn get_scopes(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_scopes()
    }
    /// <p>The type of flow to be performed</p>
    pub fn oauth2_flow(mut self, input: crate::types::Oauth2FlowType) -> Self {
        self.inner = self.inner.oauth2_flow(input);
        self
    }
    /// <p>The type of flow to be performed</p>
    pub fn set_oauth2_flow(mut self, input: ::std::option::Option<crate::types::Oauth2FlowType>) -> Self {
        self.inner = self.inner.set_oauth2_flow(input);
        self
    }
    /// <p>The type of flow to be performed</p>
    pub fn get_oauth2_flow(&self) -> &::std::option::Option<crate::types::Oauth2FlowType> {
        self.inner.get_oauth2_flow()
    }
    /// <p>Callback url to redirect after token retrieval completes. Should be one of the provideded urls during WorkloadIdentity creation</p>
    pub fn resource_oauth2_return_url(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_oauth2_return_url(input.into());
        self
    }
    /// <p>Callback url to redirect after token retrieval completes. Should be one of the provideded urls during WorkloadIdentity creation</p>
    pub fn set_resource_oauth2_return_url(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_oauth2_return_url(input);
        self
    }
    /// <p>Callback url to redirect after token retrieval completes. Should be one of the provideded urls during WorkloadIdentity creation</p>
    pub fn get_resource_oauth2_return_url(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_oauth2_return_url()
    }
    /// <p>If true, always initiate a new 3LO flow</p>
    pub fn force_authentication(mut self, input: bool) -> Self {
        self.inner = self.inner.force_authentication(input);
        self
    }
    /// <p>If true, always initiate a new 3LO flow</p>
    pub fn set_force_authentication(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force_authentication(input);
        self
    }
    /// <p>If true, always initiate a new 3LO flow</p>
    pub fn get_force_authentication(&self) -> &::std::option::Option<bool> {
        self.inner.get_force_authentication()
    }
    ///
    /// Adds a key-value pair to `customParameters`.
    ///
    /// To override the contents of this collection use [`set_custom_parameters`](Self::set_custom_parameters).
    ///
    /// <p>Gives the ability to send extra/custom parameters to the resource credentials provider during the authorization process. Standard OAuth2 flow parameters will not be overriden.</p>
    pub fn custom_parameters(
        mut self,
        k: impl ::std::convert::Into<::std::string::String>,
        v: impl ::std::convert::Into<::std::string::String>,
    ) -> Self {
        self.inner = self.inner.custom_parameters(k.into(), v.into());
        self
    }
    /// <p>Gives the ability to send extra/custom parameters to the resource credentials provider during the authorization process. Standard OAuth2 flow parameters will not be overriden.</p>
    pub fn set_custom_parameters(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>,
    ) -> Self {
        self.inner = self.inner.set_custom_parameters(input);
        self
    }
    /// <p>Gives the ability to send extra/custom parameters to the resource credentials provider during the authorization process. Standard OAuth2 flow parameters will not be overriden.</p>
    pub fn get_custom_parameters(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_custom_parameters()
    }
}
