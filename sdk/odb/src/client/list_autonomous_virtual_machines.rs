// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListAutonomousVirtualMachines`](crate::operation::list_autonomous_virtual_machines::builders::ListAutonomousVirtualMachinesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_autonomous_virtual_machines::builders::ListAutonomousVirtualMachinesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`max_results(i32)`](crate::operation::list_autonomous_virtual_machines::builders::ListAutonomousVirtualMachinesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_autonomous_virtual_machines::builders::ListAutonomousVirtualMachinesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of items to return per page.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_autonomous_virtual_machines::builders::ListAutonomousVirtualMachinesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_autonomous_virtual_machines::builders::ListAutonomousVirtualMachinesFluentBuilder::set_next_token):<br>required: **false**<br><p>The pagination token to continue listing from.</p><br>
    ///   - [`cloud_autonomous_vm_cluster_id(impl Into<String>)`](crate::operation::list_autonomous_virtual_machines::builders::ListAutonomousVirtualMachinesFluentBuilder::cloud_autonomous_vm_cluster_id) / [`set_cloud_autonomous_vm_cluster_id(Option<String>)`](crate::operation::list_autonomous_virtual_machines::builders::ListAutonomousVirtualMachinesFluentBuilder::set_cloud_autonomous_vm_cluster_id):<br>required: **true**<br><p>The unique identifier of the Autonomous VM cluster whose virtual machines you're listing.</p><br>
    /// - On success, responds with [`ListAutonomousVirtualMachinesOutput`](crate::operation::list_autonomous_virtual_machines::ListAutonomousVirtualMachinesOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_autonomous_virtual_machines::ListAutonomousVirtualMachinesOutput::next_token): <p>The pagination token from which to continue listing.</p>
    ///   - [`autonomous_virtual_machines(Vec::<AutonomousVirtualMachineSummary>)`](crate::operation::list_autonomous_virtual_machines::ListAutonomousVirtualMachinesOutput::autonomous_virtual_machines): <p>The list of Autonomous VMs in the specified Autonomous VM cluster.</p>
    /// - On failure, responds with [`SdkError<ListAutonomousVirtualMachinesError>`](crate::operation::list_autonomous_virtual_machines::ListAutonomousVirtualMachinesError)
    pub fn list_autonomous_virtual_machines(
        &self,
    ) -> crate::operation::list_autonomous_virtual_machines::builders::ListAutonomousVirtualMachinesFluentBuilder {
        crate::operation::list_autonomous_virtual_machines::builders::ListAutonomousVirtualMachinesFluentBuilder::new(self.handle.clone())
    }
}
