// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cancel_maintenance_window_execution_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::cancel_maintenance_window_execution::CancelMaintenanceWindowExecutionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.window_execution_id {
        object.key("WindowExecutionId").string(var_1.as_str());
    }
    Ok(())
}
