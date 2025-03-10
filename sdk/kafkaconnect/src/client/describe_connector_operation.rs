// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeConnectorOperation`](crate::operation::describe_connector_operation::builders::DescribeConnectorOperationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`connector_operation_arn(impl Into<String>)`](crate::operation::describe_connector_operation::builders::DescribeConnectorOperationFluentBuilder::connector_operation_arn) / [`set_connector_operation_arn(Option<String>)`](crate::operation::describe_connector_operation::builders::DescribeConnectorOperationFluentBuilder::set_connector_operation_arn):<br>required: **true**<br><p>ARN of the connector operation to be described.</p><br>
    /// - On success, responds with [`DescribeConnectorOperationOutput`](crate::operation::describe_connector_operation::DescribeConnectorOperationOutput) with field(s):
    ///   - [`connector_arn(Option<String>)`](crate::operation::describe_connector_operation::DescribeConnectorOperationOutput::connector_arn): <p>The Amazon Resource Name (ARN) of the connector.</p>
    ///   - [`connector_operation_arn(Option<String>)`](crate::operation::describe_connector_operation::DescribeConnectorOperationOutput::connector_operation_arn): <p>The Amazon Resource Name (ARN) of the connector operation.</p>
    ///   - [`connector_operation_state(Option<ConnectorOperationState>)`](crate::operation::describe_connector_operation::DescribeConnectorOperationOutput::connector_operation_state): <p>The state of the connector operation.</p>
    ///   - [`connector_operation_type(Option<ConnectorOperationType>)`](crate::operation::describe_connector_operation::DescribeConnectorOperationOutput::connector_operation_type): <p>The type of connector operation performed.</p>
    ///   - [`operation_steps(Option<Vec::<ConnectorOperationStep>>)`](crate::operation::describe_connector_operation::DescribeConnectorOperationOutput::operation_steps): <p>The array of operation steps taken.</p>
    ///   - [`origin_worker_setting(Option<WorkerSetting>)`](crate::operation::describe_connector_operation::DescribeConnectorOperationOutput::origin_worker_setting): <p>The origin worker setting.</p>
    ///   - [`origin_connector_configuration(Option<HashMap::<String, String>>)`](crate::operation::describe_connector_operation::DescribeConnectorOperationOutput::origin_connector_configuration): <p>The origin connector configuration.</p>
    ///   - [`target_worker_setting(Option<WorkerSetting>)`](crate::operation::describe_connector_operation::DescribeConnectorOperationOutput::target_worker_setting): <p>The target worker setting.</p>
    ///   - [`target_connector_configuration(Option<HashMap::<String, String>>)`](crate::operation::describe_connector_operation::DescribeConnectorOperationOutput::target_connector_configuration): <p>The target connector configuration.</p>
    ///   - [`error_info(Option<StateDescription>)`](crate::operation::describe_connector_operation::DescribeConnectorOperationOutput::error_info): <p>Details about the state of a resource.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::describe_connector_operation::DescribeConnectorOperationOutput::creation_time): <p>The time when the operation was created.</p>
    ///   - [`end_time(Option<DateTime>)`](crate::operation::describe_connector_operation::DescribeConnectorOperationOutput::end_time): <p>The time when the operation ended.</p>
    /// - On failure, responds with [`SdkError<DescribeConnectorOperationError>`](crate::operation::describe_connector_operation::DescribeConnectorOperationError)
    pub fn describe_connector_operation(&self) -> crate::operation::describe_connector_operation::builders::DescribeConnectorOperationFluentBuilder {
        crate::operation::describe_connector_operation::builders::DescribeConnectorOperationFluentBuilder::new(self.handle.clone())
    }
}
