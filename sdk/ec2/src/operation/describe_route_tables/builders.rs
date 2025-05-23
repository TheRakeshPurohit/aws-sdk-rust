// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::describe_route_tables::_describe_route_tables_output::DescribeRouteTablesOutputBuilder;

pub use crate::operation::describe_route_tables::_describe_route_tables_input::DescribeRouteTablesInputBuilder;

impl crate::operation::describe_route_tables::builders::DescribeRouteTablesInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::describe_route_tables::DescribeRouteTablesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_route_tables::DescribeRouteTablesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.describe_route_tables();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DescribeRouteTables`.
///
/// <p>Describes your route tables. The default is to describe all your route tables. Alternatively, you can specify specific route table IDs or filter the results to include only the route tables that match specific criteria.</p>
/// <p>Each subnet in your VPC must be associated with a route table. If a subnet is not explicitly associated with any route table, it is implicitly associated with the main route table. This command does not return the subnet ID for implicit associations.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/vpc/latest/userguide/VPC_Route_Tables.html">Route tables</a> in the <i>Amazon VPC User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DescribeRouteTablesFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::describe_route_tables::builders::DescribeRouteTablesInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::describe_route_tables::DescribeRouteTablesOutput,
        crate::operation::describe_route_tables::DescribeRouteTablesError,
    > for DescribeRouteTablesFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::describe_route_tables::DescribeRouteTablesOutput,
            crate::operation::describe_route_tables::DescribeRouteTablesError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DescribeRouteTablesFluentBuilder {
    /// Creates a new `DescribeRouteTablesFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DescribeRouteTables as a reference.
    pub fn as_input(&self) -> &crate::operation::describe_route_tables::builders::DescribeRouteTablesInputBuilder {
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
        crate::operation::describe_route_tables::DescribeRouteTablesOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::describe_route_tables::DescribeRouteTablesError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::describe_route_tables::DescribeRouteTables::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::describe_route_tables::DescribeRouteTables::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::describe_route_tables::DescribeRouteTablesOutput,
        crate::operation::describe_route_tables::DescribeRouteTablesError,
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
    /// Paginators are used by calling [`send().await`](crate::operation::describe_route_tables::paginator::DescribeRouteTablesPaginator::send) which returns a [`PaginationStream`](aws_smithy_async::future::pagination_stream::PaginationStream).
    pub fn into_paginator(self) -> crate::operation::describe_route_tables::paginator::DescribeRouteTablesPaginator {
        crate::operation::describe_route_tables::paginator::DescribeRouteTablesPaginator::new(self.handle, self.inner)
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token returned from a previous paginated request. Pagination continues from the end of the items returned by the previous request.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of items to return for this request. To get the next page of items, make another request with the token returned in the output. For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/APIReference/Query-Requests.html#api-pagination">Pagination</a>.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    ///
    /// Appends an item to `RouteTableIds`.
    ///
    /// To override the contents of this collection use [`set_route_table_ids`](Self::set_route_table_ids).
    ///
    /// <p>The IDs of the route tables.</p>
    pub fn route_table_ids(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.route_table_ids(input.into());
        self
    }
    /// <p>The IDs of the route tables.</p>
    pub fn set_route_table_ids(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.inner = self.inner.set_route_table_ids(input);
        self
    }
    /// <p>The IDs of the route tables.</p>
    pub fn get_route_table_ids(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        self.inner.get_route_table_ids()
    }
    ///
    /// Appends an item to `Filters`.
    ///
    /// To override the contents of this collection use [`set_filters`](Self::set_filters).
    ///
    /// <p>The filters.</p>
    /// <ul>
    /// <li>
    /// <p><code>association.gateway-id</code> - The ID of the gateway involved in the association.</p></li>
    /// <li>
    /// <p><code>association.route-table-association-id</code> - The ID of an association ID for the route table.</p></li>
    /// <li>
    /// <p><code>association.route-table-id</code> - The ID of the route table involved in the association.</p></li>
    /// <li>
    /// <p><code>association.subnet-id</code> - The ID of the subnet involved in the association.</p></li>
    /// <li>
    /// <p><code>association.main</code> - Indicates whether the route table is the main route table for the VPC (<code>true</code> | <code>false</code>). Route tables that do not have an association ID are not returned in the response.</p></li>
    /// <li>
    /// <p><code>owner-id</code> - The ID of the Amazon Web Services account that owns the route table.</p></li>
    /// <li>
    /// <p><code>route-table-id</code> - The ID of the route table.</p></li>
    /// <li>
    /// <p><code>route.destination-cidr-block</code> - The IPv4 CIDR range specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>route.destination-ipv6-cidr-block</code> - The IPv6 CIDR range specified in a route in the route table.</p></li>
    /// <li>
    /// <p><code>route.destination-prefix-list-id</code> - The ID (prefix) of the Amazon Web Services service specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>route.egress-only-internet-gateway-id</code> - The ID of an egress-only Internet gateway specified in a route in the route table.</p></li>
    /// <li>
    /// <p><code>route.gateway-id</code> - The ID of a gateway specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>route.instance-id</code> - The ID of an instance specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>route.nat-gateway-id</code> - The ID of a NAT gateway.</p></li>
    /// <li>
    /// <p><code>route.transit-gateway-id</code> - The ID of a transit gateway.</p></li>
    /// <li>
    /// <p><code>route.origin</code> - Describes how the route was created. <code>CreateRouteTable</code> indicates that the route was automatically created when the route table was created; <code>CreateRoute</code> indicates that the route was manually added to the route table; <code>EnableVgwRoutePropagation</code> indicates that the route was propagated by route propagation.</p></li>
    /// <li>
    /// <p><code>route.state</code> - The state of a route in the route table (<code>active</code> | <code>blackhole</code>). The blackhole state indicates that the route's target isn't available (for example, the specified gateway isn't attached to the VPC, the specified NAT instance has been terminated, and so on).</p></li>
    /// <li>
    /// <p><code>route.vpc-peering-connection-id</code> - The ID of a VPC peering connection specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>tag</code> - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p></li>
    /// <li>
    /// <p><code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p></li>
    /// <li>
    /// <p><code>vpc-id</code> - The ID of the VPC for the route table.</p></li>
    /// </ul>
    pub fn filters(mut self, input: crate::types::Filter) -> Self {
        self.inner = self.inner.filters(input);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li>
    /// <p><code>association.gateway-id</code> - The ID of the gateway involved in the association.</p></li>
    /// <li>
    /// <p><code>association.route-table-association-id</code> - The ID of an association ID for the route table.</p></li>
    /// <li>
    /// <p><code>association.route-table-id</code> - The ID of the route table involved in the association.</p></li>
    /// <li>
    /// <p><code>association.subnet-id</code> - The ID of the subnet involved in the association.</p></li>
    /// <li>
    /// <p><code>association.main</code> - Indicates whether the route table is the main route table for the VPC (<code>true</code> | <code>false</code>). Route tables that do not have an association ID are not returned in the response.</p></li>
    /// <li>
    /// <p><code>owner-id</code> - The ID of the Amazon Web Services account that owns the route table.</p></li>
    /// <li>
    /// <p><code>route-table-id</code> - The ID of the route table.</p></li>
    /// <li>
    /// <p><code>route.destination-cidr-block</code> - The IPv4 CIDR range specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>route.destination-ipv6-cidr-block</code> - The IPv6 CIDR range specified in a route in the route table.</p></li>
    /// <li>
    /// <p><code>route.destination-prefix-list-id</code> - The ID (prefix) of the Amazon Web Services service specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>route.egress-only-internet-gateway-id</code> - The ID of an egress-only Internet gateway specified in a route in the route table.</p></li>
    /// <li>
    /// <p><code>route.gateway-id</code> - The ID of a gateway specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>route.instance-id</code> - The ID of an instance specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>route.nat-gateway-id</code> - The ID of a NAT gateway.</p></li>
    /// <li>
    /// <p><code>route.transit-gateway-id</code> - The ID of a transit gateway.</p></li>
    /// <li>
    /// <p><code>route.origin</code> - Describes how the route was created. <code>CreateRouteTable</code> indicates that the route was automatically created when the route table was created; <code>CreateRoute</code> indicates that the route was manually added to the route table; <code>EnableVgwRoutePropagation</code> indicates that the route was propagated by route propagation.</p></li>
    /// <li>
    /// <p><code>route.state</code> - The state of a route in the route table (<code>active</code> | <code>blackhole</code>). The blackhole state indicates that the route's target isn't available (for example, the specified gateway isn't attached to the VPC, the specified NAT instance has been terminated, and so on).</p></li>
    /// <li>
    /// <p><code>route.vpc-peering-connection-id</code> - The ID of a VPC peering connection specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>tag</code> - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p></li>
    /// <li>
    /// <p><code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p></li>
    /// <li>
    /// <p><code>vpc-id</code> - The ID of the VPC for the route table.</p></li>
    /// </ul>
    pub fn set_filters(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Filter>>) -> Self {
        self.inner = self.inner.set_filters(input);
        self
    }
    /// <p>The filters.</p>
    /// <ul>
    /// <li>
    /// <p><code>association.gateway-id</code> - The ID of the gateway involved in the association.</p></li>
    /// <li>
    /// <p><code>association.route-table-association-id</code> - The ID of an association ID for the route table.</p></li>
    /// <li>
    /// <p><code>association.route-table-id</code> - The ID of the route table involved in the association.</p></li>
    /// <li>
    /// <p><code>association.subnet-id</code> - The ID of the subnet involved in the association.</p></li>
    /// <li>
    /// <p><code>association.main</code> - Indicates whether the route table is the main route table for the VPC (<code>true</code> | <code>false</code>). Route tables that do not have an association ID are not returned in the response.</p></li>
    /// <li>
    /// <p><code>owner-id</code> - The ID of the Amazon Web Services account that owns the route table.</p></li>
    /// <li>
    /// <p><code>route-table-id</code> - The ID of the route table.</p></li>
    /// <li>
    /// <p><code>route.destination-cidr-block</code> - The IPv4 CIDR range specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>route.destination-ipv6-cidr-block</code> - The IPv6 CIDR range specified in a route in the route table.</p></li>
    /// <li>
    /// <p><code>route.destination-prefix-list-id</code> - The ID (prefix) of the Amazon Web Services service specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>route.egress-only-internet-gateway-id</code> - The ID of an egress-only Internet gateway specified in a route in the route table.</p></li>
    /// <li>
    /// <p><code>route.gateway-id</code> - The ID of a gateway specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>route.instance-id</code> - The ID of an instance specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>route.nat-gateway-id</code> - The ID of a NAT gateway.</p></li>
    /// <li>
    /// <p><code>route.transit-gateway-id</code> - The ID of a transit gateway.</p></li>
    /// <li>
    /// <p><code>route.origin</code> - Describes how the route was created. <code>CreateRouteTable</code> indicates that the route was automatically created when the route table was created; <code>CreateRoute</code> indicates that the route was manually added to the route table; <code>EnableVgwRoutePropagation</code> indicates that the route was propagated by route propagation.</p></li>
    /// <li>
    /// <p><code>route.state</code> - The state of a route in the route table (<code>active</code> | <code>blackhole</code>). The blackhole state indicates that the route's target isn't available (for example, the specified gateway isn't attached to the VPC, the specified NAT instance has been terminated, and so on).</p></li>
    /// <li>
    /// <p><code>route.vpc-peering-connection-id</code> - The ID of a VPC peering connection specified in a route in the table.</p></li>
    /// <li>
    /// <p><code>tag</code> - The key/value combination of a tag assigned to the resource. Use the tag key in the filter name and the tag value as the filter value. For example, to find all resources that have a tag with the key <code>Owner</code> and the value <code>TeamA</code>, specify <code>tag:Owner</code> for the filter name and <code>TeamA</code> for the filter value.</p></li>
    /// <li>
    /// <p><code>tag-key</code> - The key of a tag assigned to the resource. Use this filter to find all resources assigned a tag with a specific key, regardless of the tag value.</p></li>
    /// <li>
    /// <p><code>vpc-id</code> - The ID of the VPC for the route table.</p></li>
    /// </ul>
    pub fn get_filters(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Filter>> {
        self.inner.get_filters()
    }
}
