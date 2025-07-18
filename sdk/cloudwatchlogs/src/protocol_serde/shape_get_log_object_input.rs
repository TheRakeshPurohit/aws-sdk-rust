// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_log_object_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_log_object::GetLogObjectInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.unmask {
        object.key("unmask").boolean(*var_1);
    }
    if let Some(var_2) = &input.log_object_pointer {
        object.key("logObjectPointer").string(var_2.as_str());
    }
    Ok(())
}
