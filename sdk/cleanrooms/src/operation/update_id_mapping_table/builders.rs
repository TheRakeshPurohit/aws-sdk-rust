// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_id_mapping_table::_update_id_mapping_table_output::UpdateIdMappingTableOutputBuilder;

pub use crate::operation::update_id_mapping_table::_update_id_mapping_table_input::UpdateIdMappingTableInputBuilder;

impl crate::operation::update_id_mapping_table::builders::UpdateIdMappingTableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_id_mapping_table::UpdateIdMappingTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_id_mapping_table::UpdateIdMappingTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_id_mapping_table();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateIdMappingTable`.
///
/// <p>Provides the details that are necessary to update an ID mapping table.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateIdMappingTableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_id_mapping_table::builders::UpdateIdMappingTableInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_id_mapping_table::UpdateIdMappingTableOutput,
        crate::operation::update_id_mapping_table::UpdateIdMappingTableError,
    > for UpdateIdMappingTableFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_id_mapping_table::UpdateIdMappingTableOutput,
            crate::operation::update_id_mapping_table::UpdateIdMappingTableError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateIdMappingTableFluentBuilder {
    /// Creates a new `UpdateIdMappingTableFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateIdMappingTable as a reference.
    pub fn as_input(&self) -> &crate::operation::update_id_mapping_table::builders::UpdateIdMappingTableInputBuilder {
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
        crate::operation::update_id_mapping_table::UpdateIdMappingTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_id_mapping_table::UpdateIdMappingTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_id_mapping_table::UpdateIdMappingTable::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_id_mapping_table::UpdateIdMappingTable::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_id_mapping_table::UpdateIdMappingTableOutput,
        crate::operation::update_id_mapping_table::UpdateIdMappingTableError,
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
    /// <p>The unique identifier of the ID mapping table that you want to update.</p>
    pub fn id_mapping_table_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.id_mapping_table_identifier(input.into());
        self
    }
    /// <p>The unique identifier of the ID mapping table that you want to update.</p>
    pub fn set_id_mapping_table_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_id_mapping_table_identifier(input);
        self
    }
    /// <p>The unique identifier of the ID mapping table that you want to update.</p>
    pub fn get_id_mapping_table_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_id_mapping_table_identifier()
    }
    /// <p>The unique identifier of the membership that contains the ID mapping table that you want to update.</p>
    pub fn membership_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.membership_identifier(input.into());
        self
    }
    /// <p>The unique identifier of the membership that contains the ID mapping table that you want to update.</p>
    pub fn set_membership_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_membership_identifier(input);
        self
    }
    /// <p>The unique identifier of the membership that contains the ID mapping table that you want to update.</p>
    pub fn get_membership_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_membership_identifier()
    }
    /// <p>A new description for the ID mapping table.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A new description for the ID mapping table.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A new description for the ID mapping table.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services KMS key.</p>
    pub fn kms_key_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.kms_key_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services KMS key.</p>
    pub fn set_kms_key_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_kms_key_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the Amazon Web Services KMS key.</p>
    pub fn get_kms_key_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_kms_key_arn()
    }
}
