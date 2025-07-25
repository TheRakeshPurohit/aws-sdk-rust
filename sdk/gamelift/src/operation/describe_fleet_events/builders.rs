// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_fleet_events::_describe_fleet_events_output::DescribeFleetEventsOutputBuilder;

pub use crate::operation::describe_fleet_events::_describe_fleet_events_input::DescribeFleetEventsInputBuilder;

impl crate::operation::describe_fleet_events::builders::DescribeFleetEventsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_fleet_events::DescribeFleetEventsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_fleet_events::DescribeFleetEventsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_fleet_events();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeFleetEvents`.
///
/// <p>Retrieves entries from a fleet's event log. Fleet events are initiated by changes in status, such as during fleet creation and termination, changes in capacity, etc. If a fleet has multiple locations, events are also initiated by changes to status and capacity in remote locations.</p>
/// <p>You can specify a time range to limit the result set. Use the pagination parameters to retrieve results as a set of sequential pages.</p>
/// <p>If successful, a collection of event log entries matching the request are returned.</p>
/// <p><b>Learn more</b></p>
/// <p><a href="https://docs.aws.amazon.com/gamelift/latest/developerguide/fleets-intro.html">Setting up Amazon GameLift Servers fleets</a></p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeFleetEventsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_fleet_events::builders::DescribeFleetEventsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_fleet_events::DescribeFleetEventsOutput,
        crate::operation::describe_fleet_events::DescribeFleetEventsError,
    > for DescribeFleetEventsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_fleet_events::DescribeFleetEventsOutput,
            crate::operation::describe_fleet_events::DescribeFleetEventsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeFleetEventsFluentBuilder {
    /// Creates a new `DescribeFleetEventsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeFleetEvents as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_fleet_events::builders::DescribeFleetEventsInputBuilder {
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
        crate::operation::describe_fleet_events::DescribeFleetEventsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_fleet_events::DescribeFleetEventsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_fleet_events::DescribeFleetEvents::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_fleet_events::DescribeFleetEvents::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_fleet_events::DescribeFleetEventsOutput,
        crate::operation::describe_fleet_events::DescribeFleetEventsError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_fleet_events::paginator::DescribeFleetEventsPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_fleet_events::paginator::DescribeFleetEventsPaginator {
        crate::operation::describe_fleet_events::paginator::DescribeFleetEventsPaginator::new(self.handle, self.inner)
    }
    /// <p>A unique identifier for the fleet to get event logs for. You can use either the fleet ID or ARN value.</p>
    pub fn fleet_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.fleet_id(input.into());
        self
    }
    /// <p>A unique identifier for the fleet to get event logs for. You can use either the fleet ID or ARN value.</p>
    pub fn set_fleet_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_fleet_id(input);
        self
    }
    /// <p>A unique identifier for the fleet to get event logs for. You can use either the fleet ID or ARN value.</p>
    pub fn get_fleet_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_fleet_id()
    }
    /// <p>The earliest date to retrieve event logs for. If no start time is specified, this call returns entries starting from when the fleet was created to the specified end time. Format is a number expressed in Unix time as milliseconds (ex: "1469498468.057").</p>
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.start_time(input);
        self
    }
    /// <p>The earliest date to retrieve event logs for. If no start time is specified, this call returns entries starting from when the fleet was created to the specified end time. Format is a number expressed in Unix time as milliseconds (ex: "1469498468.057").</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_start_time(input);
        self
    }
    /// <p>The earliest date to retrieve event logs for. If no start time is specified, this call returns entries starting from when the fleet was created to the specified end time. Format is a number expressed in Unix time as milliseconds (ex: "1469498468.057").</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_start_time()
    }
    /// <p>The most recent date to retrieve event logs for. If no end time is specified, this call returns entries from the specified start time up to the present. Format is a number expressed in Unix time as milliseconds (ex: "1469498468.057").</p>
    pub fn end_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.inner = self.inner.end_time(input);
        self
    }
    /// <p>The most recent date to retrieve event logs for. If no end time is specified, this call returns entries from the specified start time up to the present. Format is a number expressed in Unix time as milliseconds (ex: "1469498468.057").</p>
    pub fn set_end_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.inner = self.inner.set_end_time(input);
        self
    }
    /// <p>The most recent date to retrieve event logs for. If no end time is specified, this call returns entries from the specified start time up to the present. Format is a number expressed in Unix time as milliseconds (ex: "1469498468.057").</p>
    pub fn get_end_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        self.inner.get_end_time()
    }
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    pub fn limit(mut self, input: i32) -> Self {
        self.inner = self.inner.limit(input);
        self
    }
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    pub fn set_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_limit(input);
        self
    }
    /// <p>The maximum number of results to return. Use this parameter with <code>NextToken</code> to get results as a set of sequential pages.</p>
    pub fn get_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_limit()
    }
    /// <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>A token that indicates the start of the next sequential page of results. Use the token that is returned with a previous call to this operation. To start at the beginning of the result set, do not specify a value.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
}
