// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetAgentRuntime`](crate::operation::get_agent_runtime::builders::GetAgentRuntimeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`agent_runtime_id(impl Into<String>)`](crate::operation::get_agent_runtime::builders::GetAgentRuntimeFluentBuilder::agent_runtime_id) / [`set_agent_runtime_id(Option<String>)`](crate::operation::get_agent_runtime::builders::GetAgentRuntimeFluentBuilder::set_agent_runtime_id):<br>required: **true**<br><p>The unique identifier of the agent runtime to retrieve.</p><br>
    ///   - [`agent_runtime_version(impl Into<String>)`](crate::operation::get_agent_runtime::builders::GetAgentRuntimeFluentBuilder::agent_runtime_version) / [`set_agent_runtime_version(Option<String>)`](crate::operation::get_agent_runtime::builders::GetAgentRuntimeFluentBuilder::set_agent_runtime_version):<br>required: **false**<br><p>The version of the agent runtime to retrieve.</p><br>
    /// - On success, responds with [`GetAgentRuntimeOutput`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput) with field(s):
    ///   - [`agent_runtime_arn(String)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::agent_runtime_arn): <p>The Amazon Resource Name (ARN) of the agent runtime.</p>
    ///   - [`workload_identity_details(Option<WorkloadIdentityDetails>)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::workload_identity_details): <p>The workload identity details for the agent runtime.</p>
    ///   - [`agent_runtime_name(String)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::agent_runtime_name): <p>The name of the agent runtime.</p>
    ///   - [`description(Option<String>)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::description): <p>The description of the agent runtime.</p>
    ///   - [`agent_runtime_id(String)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::agent_runtime_id): <p>The unique identifier of the agent runtime.</p>
    ///   - [`agent_runtime_version(String)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::agent_runtime_version): <p>The version of the agent runtime.</p>
    ///   - [`created_at(DateTime)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::created_at): <p>The timestamp when the agent runtime was created.</p>
    ///   - [`last_updated_at(DateTime)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::last_updated_at): <p>The timestamp when the agent runtime was last updated.</p>
    ///   - [`role_arn(String)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::role_arn): <p>The IAM role ARN that provides permissions for the agent runtime.</p>
    ///   - [`agent_runtime_artifact(Option<AgentArtifact>)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::agent_runtime_artifact): <p>The artifact of the agent runtime.</p>
    ///   - [`network_configuration(Option<NetworkConfiguration>)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::network_configuration): <p>The network configuration for the agent runtime.</p>
    ///   - [`protocol_configuration(Option<ProtocolConfiguration>)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::protocol_configuration): <p>The protocol configuration for an agent runtime. This structure defines how the agent runtime communicates with clients.</p>
    ///   - [`environment_variables(Option<HashMap::<String, String>>)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::environment_variables): <p>Environment variables set in the agent runtime environment.</p>
    ///   - [`authorizer_configuration(Option<AuthorizerConfiguration>)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::authorizer_configuration): <p>The authorizer configuration for the agent runtime.</p>
    ///   - [`status(AgentStatus)`](crate::operation::get_agent_runtime::GetAgentRuntimeOutput::status): <p>The current status of the agent runtime.</p>
    /// - On failure, responds with [`SdkError<GetAgentRuntimeError>`](crate::operation::get_agent_runtime::GetAgentRuntimeError)
    pub fn get_agent_runtime(&self) -> crate::operation::get_agent_runtime::builders::GetAgentRuntimeFluentBuilder {
        crate::operation::get_agent_runtime::builders::GetAgentRuntimeFluentBuilder::new(self.handle.clone())
    }
}
