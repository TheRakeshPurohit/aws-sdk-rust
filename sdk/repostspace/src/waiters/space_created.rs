// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

///
/// Fluent builder for the `space_created` waiter.
///
/// This builder is intended to be used similar to the other fluent builders for
/// normal operations on the client. However, instead of a `send` method, it has
/// a `wait` method that takes a maximum amount of time to wait.
///
/// Construct this fluent builder using the client by importing the
/// [`Waiters`](crate::client::Waiters) trait and calling the methods
/// prefixed with `wait_until`.
///
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct SpaceCreatedFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_space::builders::GetSpaceInputBuilder,
}
impl SpaceCreatedFluentBuilder {
    /// Creates a new `SpaceCreatedFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetSpace as a reference.
    pub fn as_input(&self) -> &crate::operation::get_space::builders::GetSpaceInputBuilder {
        &self.inner
    }
    /// Wait for `space_created`
    pub async fn wait(
        self,
        max_wait: ::std::time::Duration,
    ) -> ::std::result::Result<crate::waiters::space_created::SpaceCreatedFinalPoll, crate::waiters::space_created::WaitUntilSpaceCreatedError> {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let runtime_plugins = crate::operation::get_space::GetSpace::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            ::std::option::Option::None,
        )
        .with_operation_plugin(crate::sdk_feature_tracker::waiter::WaiterFeatureTrackerRuntimePlugin::new());
        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
        let runtime_components_builder = runtime_plugins
            .apply_client_configuration(&mut cfg)
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let time_components = runtime_components_builder.into_time_components();
        let sleep_impl = time_components.sleep_impl().expect("a sleep impl is required by waiters");
        let time_source = time_components.time_source().expect("a time source is required by waiters");

        let acceptor =
            move |result: ::std::result::Result<&crate::operation::get_space::GetSpaceOutput, &crate::operation::get_space::GetSpaceError>| {
                // Matches: {"output":{"path":"status","expected":"CREATED","comparator":"stringEquals"}}
                if crate::waiters::matchers::match_get_space_2b785f05e541db69e(result) {
                    return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
                }
                // Matches: {"output":{"path":"status","expected":"CREATE_FAILED","comparator":"stringEquals"}}
                if crate::waiters::matchers::match_get_space_00d7b810548bbcb91(result) {
                    return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
                }
                // Matches: {"output":{"path":"status","expected":"CREATING","comparator":"stringEquals"}}
                if crate::waiters::matchers::match_get_space_ab538ef2e7cb9d2b4(result) {
                    return ::aws_smithy_runtime::client::waiters::AcceptorState::Retry;
                }
                ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
            };
        let operation = move || {
            let input = input.clone();
            let runtime_plugins = runtime_plugins.clone();
            async move { crate::operation::get_space::GetSpace::orchestrate(&runtime_plugins, input).await }
        };
        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
            .min_delay(::std::time::Duration::from_secs(300))
            .max_delay(::std::time::Duration::from_secs(7200))
            .max_wait(max_wait)
            .time_source(time_source)
            .sleep_impl(sleep_impl)
            .acceptor(acceptor)
            .operation(operation)
            .build();
        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await
    }
    /// <p>The ID of the private re:Post.</p>
    pub fn space_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.space_id(input.into());
        self
    }
    /// <p>The ID of the private re:Post.</p>
    pub fn set_space_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_space_id(input);
        self
    }
    /// <p>The ID of the private re:Post.</p>
    pub fn get_space_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_space_id()
    }
}

/// Successful return type for the `space_created` waiter.
pub type SpaceCreatedFinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<
    crate::operation::get_space::GetSpaceOutput,
    ::aws_smithy_runtime_api::client::result::SdkError<
        crate::operation::get_space::GetSpaceError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
>;

/// Error type for the `space_created` waiter.
pub type WaitUntilSpaceCreatedError = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<
    crate::operation::get_space::GetSpaceOutput,
    crate::operation::get_space::GetSpaceError,
>;
