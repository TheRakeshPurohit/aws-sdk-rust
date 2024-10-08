// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_grouping_statuses::_list_grouping_statuses_output::ListGroupingStatusesOutputBuilder;

pub use crate::operation::list_grouping_statuses::_list_grouping_statuses_input::ListGroupingStatusesInputBuilder;

impl crate::operation::list_grouping_statuses::builders::ListGroupingStatusesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_grouping_statuses::ListGroupingStatusesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_grouping_statuses::ListGroupingStatusesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_grouping_statuses();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListGroupingStatuses`.
///
/// <p>Returns the status of the last grouping or ungrouping action for each resource in the specified application group.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListGroupingStatusesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_grouping_statuses::builders::ListGroupingStatusesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_grouping_statuses::ListGroupingStatusesOutput,
        crate::operation::list_grouping_statuses::ListGroupingStatusesError,
    > for ListGroupingStatusesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_grouping_statuses::ListGroupingStatusesOutput,
            crate::operation::list_grouping_statuses::ListGroupingStatusesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListGroupingStatusesFluentBuilder {
    /// Creates a new `ListGroupingStatusesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListGroupingStatuses as a reference.
    pub fn as_input(&self) -> &crate::operation::list_grouping_statuses::builders::ListGroupingStatusesInputBuilder {
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
        crate::operation::list_grouping_statuses::ListGroupingStatusesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_grouping_statuses::ListGroupingStatusesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_grouping_statuses::ListGroupingStatuses::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_grouping_statuses::ListGroupingStatuses::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_grouping_statuses::ListGroupingStatusesOutput,
        crate::operation::list_grouping_statuses::ListGroupingStatusesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::list_grouping_statuses::paginator::ListGroupingStatusesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::list_grouping_statuses::paginator::ListGroupingStatusesPaginator {
        crate::operation::list_grouping_statuses::paginator::ListGroupingStatusesPaginator::new(self.handle, self.inner)
    }
    /// <p>The application group identifier, expressed as an Amazon resource name (ARN) or the application group name.</p>
    pub fn group(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.group(input.into());
        self
    }
    /// <p>The application group identifier, expressed as an Amazon resource name (ARN) or the application group name.</p>
    pub fn set_group(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_group(input);
        self
    }
    /// <p>The application group identifier, expressed as an Amazon resource name (ARN) or the application group name.</p>
    pub fn get_group(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_group()
    }
    /// <p>The maximum number of resources and their statuses returned in the response.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of resources and their statuses returned in the response.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of resources and their statuses returned in the response.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    ///
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filter name and value pair that is used to return more specific results from a list of resources.</p>
    pub fn filters(mut self, input: crate::types::ListGroupingStatusesFilter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>The filter name and value pair that is used to return more specific results from a list of resources.</p>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ListGroupingStatusesFilter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The filter name and value pair that is used to return more specific results from a list of resources.</p>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ListGroupingStatusesFilter>> {
        self.inner.get_filters()
    }
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value provided by a previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value provided by a previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The parameter for receiving additional results if you receive a <code>NextToken</code> response in a previous request. A <code>NextToken</code> response indicates that more output is available. Set this parameter to the value provided by a previous call's <code>NextToken</code> response to indicate where the output should continue from.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
