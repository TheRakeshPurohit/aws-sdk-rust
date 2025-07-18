// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_table::_update_table_output::UpdateTableOutputBuilder;

pub use crate::operation::update_table::_update_table_input::UpdateTableInputBuilder;

impl crate::operation::update_table::builders::UpdateTableInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_table::UpdateTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_table::UpdateTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_table();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateTable`.
///
/// <p>Updates a metadata table in the Data Catalog.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateTableFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_table::builders::UpdateTableInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_table::UpdateTableOutput,
        crate::operation::update_table::UpdateTableError,
    > for UpdateTableFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_table::UpdateTableOutput,
            crate::operation::update_table::UpdateTableError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateTableFluentBuilder {
    /// Creates a new `UpdateTableFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateTable as a reference.
    pub fn as_input(&self) -> &crate::operation::update_table::builders::UpdateTableInputBuilder {
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
        crate::operation::update_table::UpdateTableOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_table::UpdateTableError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_table::UpdateTable::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_table::UpdateTable::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_table::UpdateTableOutput,
        crate::operation::update_table::UpdateTableError,
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
    /// <p>The ID of the Data Catalog where the table resides. If none is provided, the Amazon Web Services account ID is used by default.</p>
    pub fn catalog_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.catalog_id(input.into());
        self
    }
    /// <p>The ID of the Data Catalog where the table resides. If none is provided, the Amazon Web Services account ID is used by default.</p>
    pub fn set_catalog_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_catalog_id(input);
        self
    }
    /// <p>The ID of the Data Catalog where the table resides. If none is provided, the Amazon Web Services account ID is used by default.</p>
    pub fn get_catalog_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_catalog_id()
    }
    /// <p>The name of the catalog database in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    pub fn database_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.database_name(input.into());
        self
    }
    /// <p>The name of the catalog database in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    pub fn set_database_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_database_name(input);
        self
    }
    /// <p>The name of the catalog database in which the table resides. For Hive compatibility, this name is entirely lowercase.</p>
    pub fn get_database_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_database_name()
    }
    /// <p>The unique identifier for the table within the specified database that will be created in the Glue Data Catalog.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The unique identifier for the table within the specified database that will be created in the Glue Data Catalog.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The unique identifier for the table within the specified database that will be created in the Glue Data Catalog.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>An updated <code>TableInput</code> object to define the metadata table in the catalog.</p>
    pub fn table_input(mut self, input: crate::types::TableInput) -> Self {
        self.inner = self.inner.table_input(input);
        self
    }
    /// <p>An updated <code>TableInput</code> object to define the metadata table in the catalog.</p>
    pub fn set_table_input(mut self, input: ::std::option::Option<crate::types::TableInput>) -> Self {
        self.inner = self.inner.set_table_input(input);
        self
    }
    /// <p>An updated <code>TableInput</code> object to define the metadata table in the catalog.</p>
    pub fn get_table_input(&self) -> &::std::option::Option<crate::types::TableInput> {
        self.inner.get_table_input()
    }
    /// <p>By default, <code>UpdateTable</code> always creates an archived version of the table before updating it. However, if <code>skipArchive</code> is set to true, <code>UpdateTable</code> does not create the archived version.</p>
    pub fn skip_archive(mut self, input: bool) -> Self {
        self.inner = self.inner.skip_archive(input);
        self
    }
    /// <p>By default, <code>UpdateTable</code> always creates an archived version of the table before updating it. However, if <code>skipArchive</code> is set to true, <code>UpdateTable</code> does not create the archived version.</p>
    pub fn set_skip_archive(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_skip_archive(input);
        self
    }
    /// <p>By default, <code>UpdateTable</code> always creates an archived version of the table before updating it. However, if <code>skipArchive</code> is set to true, <code>UpdateTable</code> does not create the archived version.</p>
    pub fn get_skip_archive(&self) -> &::std::option::Option<bool> {
        self.inner.get_skip_archive()
    }
    /// <p>The transaction ID at which to update the table contents.</p>
    pub fn transaction_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.transaction_id(input.into());
        self
    }
    /// <p>The transaction ID at which to update the table contents.</p>
    pub fn set_transaction_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_transaction_id(input);
        self
    }
    /// <p>The transaction ID at which to update the table contents.</p>
    pub fn get_transaction_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_transaction_id()
    }
    /// <p>The version ID at which to update the table contents.</p>
    pub fn version_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.version_id(input.into());
        self
    }
    /// <p>The version ID at which to update the table contents.</p>
    pub fn set_version_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_version_id(input);
        self
    }
    /// <p>The version ID at which to update the table contents.</p>
    pub fn get_version_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_version_id()
    }
    /// <p>The operation to be performed when updating the view.</p>
    pub fn view_update_action(mut self, input: crate::types::ViewUpdateAction) -> Self {
        self.inner = self.inner.view_update_action(input);
        self
    }
    /// <p>The operation to be performed when updating the view.</p>
    pub fn set_view_update_action(mut self, input: ::std::option::Option<crate::types::ViewUpdateAction>) -> Self {
        self.inner = self.inner.set_view_update_action(input);
        self
    }
    /// <p>The operation to be performed when updating the view.</p>
    pub fn get_view_update_action(&self) -> &::std::option::Option<crate::types::ViewUpdateAction> {
        self.inner.get_view_update_action()
    }
    /// <p>A flag that can be set to true to ignore matching storage descriptor and subobject matching requirements.</p>
    pub fn force(mut self, input: bool) -> Self {
        self.inner = self.inner.force(input);
        self
    }
    /// <p>A flag that can be set to true to ignore matching storage descriptor and subobject matching requirements.</p>
    pub fn set_force(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_force(input);
        self
    }
    /// <p>A flag that can be set to true to ignore matching storage descriptor and subobject matching requirements.</p>
    pub fn get_force(&self) -> &::std::option::Option<bool> {
        self.inner.get_force()
    }
    /// <p>Input parameters for updating open table format tables in GlueData Catalog, serving as a wrapper for format-specific update operations such as Apache Iceberg.</p>
    pub fn update_open_table_format_input(mut self, input: crate::types::UpdateOpenTableFormatInput) -> Self {
        self.inner = self.inner.update_open_table_format_input(input);
        self
    }
    /// <p>Input parameters for updating open table format tables in GlueData Catalog, serving as a wrapper for format-specific update operations such as Apache Iceberg.</p>
    pub fn set_update_open_table_format_input(mut self, input: ::std::option::Option<crate::types::UpdateOpenTableFormatInput>) -> Self {
        self.inner = self.inner.set_update_open_table_format_input(input);
        self
    }
    /// <p>Input parameters for updating open table format tables in GlueData Catalog, serving as a wrapper for format-specific update operations such as Apache Iceberg.</p>
    pub fn get_update_open_table_format_input(&self) -> &::std::option::Option<crate::types::UpdateOpenTableFormatInput> {
        self.inner.get_update_open_table_format_input()
    }
}
