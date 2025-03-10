// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::get_recovery_point_index_details::_get_recovery_point_index_details_output::GetRecoveryPointIndexDetailsOutputBuilder;

pub use crate::operation::get_recovery_point_index_details::_get_recovery_point_index_details_input::GetRecoveryPointIndexDetailsInputBuilder;

impl crate::operation::get_recovery_point_index_details::builders::GetRecoveryPointIndexDetailsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::get_recovery_point_index_details::GetRecoveryPointIndexDetailsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_recovery_point_index_details::GetRecoveryPointIndexDetailsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.get_recovery_point_index_details();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `GetRecoveryPointIndexDetails`.
///
/// <p>This operation returns the metadata and details specific to the backup index associated with the specified recovery point.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct GetRecoveryPointIndexDetailsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_recovery_point_index_details::builders::GetRecoveryPointIndexDetailsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::get_recovery_point_index_details::GetRecoveryPointIndexDetailsOutput,
        crate::operation::get_recovery_point_index_details::GetRecoveryPointIndexDetailsError,
    > for GetRecoveryPointIndexDetailsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::get_recovery_point_index_details::GetRecoveryPointIndexDetailsOutput,
            crate::operation::get_recovery_point_index_details::GetRecoveryPointIndexDetailsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl GetRecoveryPointIndexDetailsFluentBuilder {
    /// Creates a new `GetRecoveryPointIndexDetailsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the GetRecoveryPointIndexDetails as a reference.
    pub fn as_input(&self) -> &crate::operation::get_recovery_point_index_details::builders::GetRecoveryPointIndexDetailsInputBuilder {
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
        crate::operation::get_recovery_point_index_details::GetRecoveryPointIndexDetailsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::get_recovery_point_index_details::GetRecoveryPointIndexDetailsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::get_recovery_point_index_details::GetRecoveryPointIndexDetails::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::get_recovery_point_index_details::GetRecoveryPointIndexDetails::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::get_recovery_point_index_details::GetRecoveryPointIndexDetailsOutput,
        crate::operation::get_recovery_point_index_details::GetRecoveryPointIndexDetailsError,
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
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Region where they are created.</p>
    /// <p>Accepted characters include lowercase letters, numbers, and hyphens.</p>
    pub fn backup_vault_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.backup_vault_name(input.into());
        self
    }
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Region where they are created.</p>
    /// <p>Accepted characters include lowercase letters, numbers, and hyphens.</p>
    pub fn set_backup_vault_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_backup_vault_name(input);
        self
    }
    /// <p>The name of a logical container where backups are stored. Backup vaults are identified by names that are unique to the account used to create them and the Region where they are created.</p>
    /// <p>Accepted characters include lowercase letters, numbers, and hyphens.</p>
    pub fn get_backup_vault_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_backup_vault_name()
    }
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    pub fn recovery_point_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.recovery_point_arn(input.into());
        self
    }
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    pub fn set_recovery_point_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_recovery_point_arn(input);
        self
    }
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    pub fn get_recovery_point_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_recovery_point_arn()
    }
}
