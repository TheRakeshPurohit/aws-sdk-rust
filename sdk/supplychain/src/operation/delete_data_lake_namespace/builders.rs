// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_data_lake_namespace::_delete_data_lake_namespace_output::DeleteDataLakeNamespaceOutputBuilder;

pub use crate::operation::delete_data_lake_namespace::_delete_data_lake_namespace_input::DeleteDataLakeNamespaceInputBuilder;

impl crate::operation::delete_data_lake_namespace::builders::DeleteDataLakeNamespaceInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_data_lake_namespace::DeleteDataLakeNamespaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_data_lake_namespace::DeleteDataLakeNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_data_lake_namespace();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteDataLakeNamespace`.
///
/// <p>Enables you to programmatically delete an Amazon Web Services Supply Chain data lake namespace and its underling datasets. Developers can delete the existing namespaces for a given instance ID and namespace name.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteDataLakeNamespaceFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_data_lake_namespace::builders::DeleteDataLakeNamespaceInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_data_lake_namespace::DeleteDataLakeNamespaceOutput,
        crate::operation::delete_data_lake_namespace::DeleteDataLakeNamespaceError,
    > for DeleteDataLakeNamespaceFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_data_lake_namespace::DeleteDataLakeNamespaceOutput,
            crate::operation::delete_data_lake_namespace::DeleteDataLakeNamespaceError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteDataLakeNamespaceFluentBuilder {
    /// Creates a new `DeleteDataLakeNamespaceFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteDataLakeNamespace as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_data_lake_namespace::builders::DeleteDataLakeNamespaceInputBuilder {
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
        crate::operation::delete_data_lake_namespace::DeleteDataLakeNamespaceOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_data_lake_namespace::DeleteDataLakeNamespaceError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_data_lake_namespace::DeleteDataLakeNamespace::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_data_lake_namespace::DeleteDataLakeNamespace::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_data_lake_namespace::DeleteDataLakeNamespaceOutput,
        crate::operation::delete_data_lake_namespace::DeleteDataLakeNamespaceError,
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
    /// <p>The AWS Supply Chain instance identifier.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>The AWS Supply Chain instance identifier.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>The AWS Supply Chain instance identifier.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>The name of the namespace. Noted you cannot delete pre-defined namespace like <b>asc</b>, <b>default</b> which are only deleted through instance deletion.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the namespace. Noted you cannot delete pre-defined namespace like <b>asc</b>, <b>default</b> which are only deleted through instance deletion.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the namespace. Noted you cannot delete pre-defined namespace like <b>asc</b>, <b>default</b> which are only deleted through instance deletion.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
}
