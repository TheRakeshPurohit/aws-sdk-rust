// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_stop_calculation_execution_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::stop_calculation_execution::StopCalculationExecutionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.calculation_execution_id {
        object.key("CalculationExecutionId").string(var_1.as_str());
    }
    Ok(())
}
