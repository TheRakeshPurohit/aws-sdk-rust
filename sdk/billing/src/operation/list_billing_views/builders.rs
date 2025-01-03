// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_billing_views::_list_billing_views_output::ListBillingViewsOutputBuilder;

pub use crate::operation::list_billing_views::_list_billing_views_input::ListBillingViewsInputBuilder;

impl crate::operation::list_billing_views::builders::ListBillingViewsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_billing_views::ListBillingViewsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_billing_views::ListBillingViewsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_billing_views();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListBillingViews`.
///
/// <p>Lists the billing views available for a given time period.</p>
/// <p>Every Amazon Web Services account has a unique <code>PRIMARY</code> billing view that represents the billing data available by default. Accounts that use Billing Conductor also have <code>BILLING_GROUP</code> billing views representing pro forma costs associated with each created billing group.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListBillingViewsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_billing_views::builders::ListBillingViewsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_billing_views::ListBillingViewsOutput,
        crate::operation::list_billing_views::ListBillingViewsError,
    > for ListBillingViewsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_billing_views::ListBillingViewsOutput,
            crate::operation::list_billing_views::ListBillingViewsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListBillingViewsFluentBuilder {
    /// Creates a new `ListBillingViewsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListBillingViews as a reference.
    pub fn as_input(&self) -> &crate::operation::list_billing_views::builders::ListBillingViewsInputBuilder {
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
        crate::operation::list_billing_views::ListBillingViewsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_billing_views::ListBillingViewsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_billing_views::ListBillingViews::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_billing_views::ListBillingViews::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_billing_views::ListBillingViewsOutput,
        crate::operation::list_billing_views::ListBillingViewsError,
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
    /// Create a paginator for this request
    ///
    /// Paginators are used by calling [`send().await`](crate::operation::list_billing_views::paginator::ListBillingViewsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_billing_views::paginator::ListBillingViewsPaginator {
        crate::operation::list_billing_views::paginator::ListBillingViewsPaginator::new(self.handle, self.inner)
    }
    /// <p>The time range for the billing views listed. <code>PRIMARY</code> billing view is always listed. <code>BILLING_GROUP</code> billing views are listed for time ranges when the associated billing group resource in Billing Conductor is active. The time range must be within one calendar month.</p>
    pub fn active_time_range(mut self, input: crate::types::ActiveTimeRange) -> Self {
        self.inner = self.inner.active_time_range(input);
        self
    }
    /// <p>The time range for the billing views listed. <code>PRIMARY</code> billing view is always listed. <code>BILLING_GROUP</code> billing views are listed for time ranges when the associated billing group resource in Billing Conductor is active. The time range must be within one calendar month.</p>
    pub fn set_active_time_range(mut self, input: ::std::option::Option<crate::types::ActiveTimeRange>) -> Self {
        self.inner = self.inner.set_active_time_range(input);
        self
    }
    /// <p>The time range for the billing views listed. <code>PRIMARY</code> billing view is always listed. <code>BILLING_GROUP</code> billing views are listed for time ranges when the associated billing group resource in Billing Conductor is active. The time range must be within one calendar month.</p>
    pub fn get_active_time_range(&self) -> &::std::option::Option<crate::types::ActiveTimeRange> {
        self.inner.get_active_time_range()
    }
    ///
    /// Appends an item to `arns`.
    ///
    /// To override the contents of this collection use [`set_arns`](Self::set_arns).
    ///
    /// <p>The Amazon Resource Name (ARN) that can be used to uniquely identify the billing view.</p>
    pub fn arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.arns(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that can be used to uniquely identify the billing view.</p>
    pub fn set_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_arns(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) that can be used to uniquely identify the billing view.</p>
    pub fn get_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_arns()
    }
    ///
    /// Appends an item to `billingViewTypes`.
    ///
    /// To override the contents of this collection use [`set_billing_view_types`](Self::set_billing_view_types).
    ///
    /// <p>The type of billing view.</p>
    pub fn billing_view_types(mut self, input: crate::types::BillingViewType) -> Self {
        self.inner = self.inner.billing_view_types(input);
        self
    }
    /// <p>The type of billing view.</p>
    pub fn set_billing_view_types(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::BillingViewType>>) -> Self {
        self.inner = self.inner.set_billing_view_types(input);
        self
    }
    /// <p>The type of billing view.</p>
    pub fn get_billing_view_types(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::BillingViewType>> {
        self.inner.get_billing_view_types()
    }
    /// <p>The list of owners of the billing view.</p>
    pub fn owner_account_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.owner_account_id(input.into());
        self
    }
    /// <p>The list of owners of the billing view.</p>
    pub fn set_owner_account_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_owner_account_id(input);
        self
    }
    /// <p>The list of owners of the billing view.</p>
    pub fn get_owner_account_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_owner_account_id()
    }
    /// <p>The maximum number of billing views to retrieve. Default is 100.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of billing views to retrieve. Default is 100.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of billing views to retrieve. Default is 100.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The pagination token that is used on subsequent calls to list billing views.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The pagination token that is used on subsequent calls to list billing views.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The pagination token that is used on subsequent calls to list billing views.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
