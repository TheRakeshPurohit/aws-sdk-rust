// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_query_insights(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::QueryInsights,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Mode").string(input.mode.as_str());
    }
    Ok(())
}
