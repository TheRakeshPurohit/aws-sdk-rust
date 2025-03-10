// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_protection_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::describe_protection::DescribeProtectionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.protection_id {
        object.key("ProtectionId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.resource_arn {
        object.key("ResourceArn").string(var_2.as_str());
    }
    Ok(())
}
