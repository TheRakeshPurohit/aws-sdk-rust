// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

///
/// Fluent builder for the `command_executed` waiter.
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
pub struct CommandExecutedFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::get_command_invocation::builders::GetCommandInvocationInputBuilder,
}
impl CommandExecutedFluentBuilder {
    /// Creates a new `CommandExecutedFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
        }
    }
    /// Access the GetCommandInvocation as a reference.
    pub fn as_input(&self) -> &crate::operation::get_command_invocation::builders::GetCommandInvocationInputBuilder {
        &self.inner
    }
    /// Wait for `command_executed`
    pub async fn wait(
        self,
        max_wait: ::std::time::Duration,
    ) -> ::std::result::Result<
        crate::waiters::command_executed::CommandExecutedFinalPoll,
        crate::waiters::command_executed::WaitUntilCommandExecutedError,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::waiters::error::WaiterError::construction_failure)?;
        let runtime_plugins = crate::operation::get_command_invocation::GetCommandInvocation::operation_runtime_plugins(
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

        let acceptor = move |result: ::std::result::Result<
            &crate::operation::get_command_invocation::GetCommandInvocationOutput,
            &crate::operation::get_command_invocation::GetCommandInvocationError,
        >| {
            // Matches: {"output":{"path":"Status","expected":"Pending","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_command_invocation_6f9158f9231e5d6f7(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Retry;
            }
            // Matches: {"output":{"path":"Status","expected":"InProgress","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_command_invocation_00bb09b3077ef8fe3(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Retry;
            }
            // Matches: {"output":{"path":"Status","expected":"Delayed","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_command_invocation_af4f0e62bb4e2df8e(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Retry;
            }
            // Matches: {"output":{"path":"Status","expected":"Success","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_command_invocation_1cbdfd792e1f604ca(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Success;
            }
            // Matches: {"output":{"path":"Status","expected":"Cancelled","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_command_invocation_c1220f1751678bdb8(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"output":{"path":"Status","expected":"TimedOut","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_command_invocation_6566d5bc527202e21(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"output":{"path":"Status","expected":"Failed","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_command_invocation_74e589a66394e154b(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"output":{"path":"Status","expected":"Cancelling","comparator":"stringEquals"}}
            if crate::waiters::matchers::match_get_command_invocation_f9ea517237b0ffd40(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Failure;
            }
            // Matches: {"errorType":"InvocationDoesNotExist"}
            if crate::waiters::matchers::match_get_command_invocation_bbe7351a13d015aba(result) {
                return ::aws_smithy_runtime::client::waiters::AcceptorState::Retry;
            }
            ::aws_smithy_runtime::client::waiters::AcceptorState::NoAcceptorsMatched
        };
        let operation = move || {
            let input = input.clone();
            let runtime_plugins = runtime_plugins.clone();
            async move { crate::operation::get_command_invocation::GetCommandInvocation::orchestrate(&runtime_plugins, input).await }
        };
        let orchestrator = ::aws_smithy_runtime::client::waiters::WaiterOrchestrator::builder()
            .min_delay(::std::time::Duration::from_secs(5))
            .max_delay(::std::time::Duration::from_secs(120))
            .max_wait(max_wait)
            .time_source(time_source)
            .sleep_impl(sleep_impl)
            .acceptor(acceptor)
            .operation(operation)
            .build();
        ::aws_smithy_runtime::client::waiters::attach_waiter_tracing_span(orchestrator.orchestrate()).await
    }
    /// <p>(Required) The parent command ID of the invocation plugin.</p>
    pub fn command_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.command_id(input.into());
        self
    }
    /// <p>(Required) The parent command ID of the invocation plugin.</p>
    pub fn set_command_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_command_id(input);
        self
    }
    /// <p>(Required) The parent command ID of the invocation plugin.</p>
    pub fn get_command_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_command_id()
    }
    /// <p>(Required) The ID of the managed node targeted by the command. A <i>managed node</i> can be an Amazon Elastic Compute Cloud (Amazon EC2) instance, edge device, and on-premises server or VM in your hybrid environment that is configured for Amazon Web Services Systems Manager.</p>
    pub fn instance_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.instance_id(input.into());
        self
    }
    /// <p>(Required) The ID of the managed node targeted by the command. A <i>managed node</i> can be an Amazon Elastic Compute Cloud (Amazon EC2) instance, edge device, and on-premises server or VM in your hybrid environment that is configured for Amazon Web Services Systems Manager.</p>
    pub fn set_instance_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_instance_id(input);
        self
    }
    /// <p>(Required) The ID of the managed node targeted by the command. A <i>managed node</i> can be an Amazon Elastic Compute Cloud (Amazon EC2) instance, edge device, and on-premises server or VM in your hybrid environment that is configured for Amazon Web Services Systems Manager.</p>
    pub fn get_instance_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_instance_id()
    }
    /// <p>The name of the step for which you want detailed results. If the document contains only one step, you can omit the name and details for that step. If the document contains more than one step, you must specify the name of the step for which you want to view details. Be sure to specify the name of the step, not the name of a plugin like <code>aws:RunShellScript</code>.</p>
    /// <p>To find the <code>PluginName</code>, check the document content and find the name of the step you want details for. Alternatively, use <code>ListCommandInvocations</code> with the <code>CommandId</code> and <code>Details</code> parameters. The <code>PluginName</code> is the <code>Name</code> attribute of the <code>CommandPlugin</code> object in the <code>CommandPlugins</code> list.</p>
    pub fn plugin_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.plugin_name(input.into());
        self
    }
    /// <p>The name of the step for which you want detailed results. If the document contains only one step, you can omit the name and details for that step. If the document contains more than one step, you must specify the name of the step for which you want to view details. Be sure to specify the name of the step, not the name of a plugin like <code>aws:RunShellScript</code>.</p>
    /// <p>To find the <code>PluginName</code>, check the document content and find the name of the step you want details for. Alternatively, use <code>ListCommandInvocations</code> with the <code>CommandId</code> and <code>Details</code> parameters. The <code>PluginName</code> is the <code>Name</code> attribute of the <code>CommandPlugin</code> object in the <code>CommandPlugins</code> list.</p>
    pub fn set_plugin_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_plugin_name(input);
        self
    }
    /// <p>The name of the step for which you want detailed results. If the document contains only one step, you can omit the name and details for that step. If the document contains more than one step, you must specify the name of the step for which you want to view details. Be sure to specify the name of the step, not the name of a plugin like <code>aws:RunShellScript</code>.</p>
    /// <p>To find the <code>PluginName</code>, check the document content and find the name of the step you want details for. Alternatively, use <code>ListCommandInvocations</code> with the <code>CommandId</code> and <code>Details</code> parameters. The <code>PluginName</code> is the <code>Name</code> attribute of the <code>CommandPlugin</code> object in the <code>CommandPlugins</code> list.</p>
    pub fn get_plugin_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_plugin_name()
    }
}

/// Successful return type for the `command_executed` waiter.
pub type CommandExecutedFinalPoll = ::aws_smithy_runtime_api::client::waiters::FinalPoll<
    crate::operation::get_command_invocation::GetCommandInvocationOutput,
    ::aws_smithy_runtime_api::client::result::SdkError<
        crate::operation::get_command_invocation::GetCommandInvocationError,
        ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
    >,
>;

/// Error type for the `command_executed` waiter.
pub type WaitUntilCommandExecutedError = ::aws_smithy_runtime_api::client::waiters::error::WaiterError<
    crate::operation::get_command_invocation::GetCommandInvocationOutput,
    crate::operation::get_command_invocation::GetCommandInvocationError,
>;
