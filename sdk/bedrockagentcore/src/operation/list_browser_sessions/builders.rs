// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::list_browser_sessions::_list_browser_sessions_output::ListBrowserSessionsOutputBuilder;

pub use crate::operation::list_browser_sessions::_list_browser_sessions_input::ListBrowserSessionsInputBuilder;

impl crate::operation::list_browser_sessions::builders::ListBrowserSessionsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::list_browser_sessions::ListBrowserSessionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_browser_sessions::ListBrowserSessionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.list_browser_sessions();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ListBrowserSessions`.
///
/// <p>Retrieves a list of browser sessions in Amazon Bedrock that match the specified criteria. This operation returns summary information about each session, including identifiers, status, and timestamps.</p>
/// <p>You can filter the results by browser identifier and session status. The operation supports pagination to handle large result sets efficiently.</p>
/// <p>We recommend using pagination to ensure that the operation returns quickly and successfully when retrieving large numbers of sessions.</p>
/// <p>The following operations are related to <code>ListBrowserSessions</code>:</p>
/// <ul>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/API_StartBrowserSession.html">StartBrowserSession</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/API_GetBrowserSession.html">GetBrowserSession</a></p></li>
/// </ul>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ListBrowserSessionsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::list_browser_sessions::builders::ListBrowserSessionsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::list_browser_sessions::ListBrowserSessionsOutput,
        crate::operation::list_browser_sessions::ListBrowserSessionsError,
    > for ListBrowserSessionsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::list_browser_sessions::ListBrowserSessionsOutput,
            crate::operation::list_browser_sessions::ListBrowserSessionsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ListBrowserSessionsFluentBuilder {
    /// Creates a new `ListBrowserSessionsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ListBrowserSessions as a reference.
    pub fn as_input(&self) -> &crate::operation::list_browser_sessions::builders::ListBrowserSessionsInputBuilder {
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
        crate::operation::list_browser_sessions::ListBrowserSessionsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_browser_sessions::ListBrowserSessionsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::list_browser_sessions::ListBrowserSessions::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::list_browser_sessions::ListBrowserSessions::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::list_browser_sessions::ListBrowserSessionsOutput,
        crate::operation::list_browser_sessions::ListBrowserSessionsError,
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
    /// <p>The unique identifier of the browser to list sessions for. If specified, only sessions for this browser are returned. If not specified, sessions for all browsers are returned.</p>
    pub fn browser_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.browser_identifier(input.into());
        self
    }
    /// <p>The unique identifier of the browser to list sessions for. If specified, only sessions for this browser are returned. If not specified, sessions for all browsers are returned.</p>
    pub fn set_browser_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_browser_identifier(input);
        self
    }
    /// <p>The unique identifier of the browser to list sessions for. If specified, only sessions for this browser are returned. If not specified, sessions for all browsers are returned.</p>
    pub fn get_browser_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_browser_identifier()
    }
    /// <p>The maximum number of results to return in a single call. The default value is 10. Valid values range from 1 to 100. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn max_results(mut self, input: i32) -> Self {
        self.inner = self.inner.max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. The default value is 10. Valid values range from 1 to 100. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn set_max_results(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_max_results(input);
        self
    }
    /// <p>The maximum number of results to return in a single call. The default value is 10. Valid values range from 1 to 100. To retrieve the remaining results, make another call with the returned <code>nextToken</code> value.</p>
    pub fn get_max_results(&self) -> &::std::option::Option<i32> {
        self.inner.get_max_results()
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results. If not specified, Amazon Bedrock returns the first page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.next_token(input.into());
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results. If not specified, Amazon Bedrock returns the first page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_next_token(input);
        self
    }
    /// <p>The token for the next set of results. Use the value returned in the previous response in the next request to retrieve the next set of results. If not specified, Amazon Bedrock returns the first page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_next_token()
    }
    /// <p>The status of the browser sessions to list. Valid values include ACTIVE, STOPPING, and STOPPED. If not specified, sessions with any status are returned.</p>
    pub fn status(mut self, input: crate::types::BrowserSessionStatus) -> Self {
        self.inner = self.inner.status(input);
        self
    }
    /// <p>The status of the browser sessions to list. Valid values include ACTIVE, STOPPING, and STOPPED. If not specified, sessions with any status are returned.</p>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::BrowserSessionStatus>) -> Self {
        self.inner = self.inner.set_status(input);
        self
    }
    /// <p>The status of the browser sessions to list. Valid values include ACTIVE, STOPPING, and STOPPED. If not specified, sessions with any status are returned.</p>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::BrowserSessionStatus> {
        self.inner.get_status()
    }
}
