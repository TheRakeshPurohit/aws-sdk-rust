// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_credential_locker::_get_credential_locker_output::GetCredentialLockerOutputBuilder;

pub use crate::operation::get_credential_locker::_get_credential_locker_input::GetCredentialLockerInputBuilder;

impl crate::operation::get_credential_locker::builders::GetCredentialLockerInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_credential_locker::GetCredentialLockerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_credential_locker::GetCredentialLockerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_credential_locker();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetCredentialLocker`.
///
/// <p>Get information on an existing credential locker</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetCredentialLockerFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_credential_locker::builders::GetCredentialLockerInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_credential_locker::GetCredentialLockerOutput,
        crate::operation::get_credential_locker::GetCredentialLockerError,
    > for GetCredentialLockerFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_credential_locker::GetCredentialLockerOutput,
            crate::operation::get_credential_locker::GetCredentialLockerError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetCredentialLockerFluentBuilder {
    /// Creates a new `GetCredentialLockerFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetCredentialLocker as a reference.
    pub fn as_input(&self) -> &crate::operation::get_credential_locker::builders::GetCredentialLockerInputBuilder {
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
        crate::operation::get_credential_locker::GetCredentialLockerOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_credential_locker::GetCredentialLockerError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_credential_locker::GetCredentialLocker::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_credential_locker::GetCredentialLocker::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_credential_locker::GetCredentialLockerOutput,
        crate::operation::get_credential_locker::GetCredentialLockerError,
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
    /// <p>The identifier of the credential locker.</p>
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.identifier(input.into());
        self
    }
    /// <p>The identifier of the credential locker.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_identifier(input);
        self
    }
    /// <p>The identifier of the credential locker.</p>
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_identifier()
    }
}
