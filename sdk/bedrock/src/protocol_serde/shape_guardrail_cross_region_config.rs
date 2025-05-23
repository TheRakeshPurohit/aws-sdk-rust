// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_guardrail_cross_region_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::GuardrailCrossRegionConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object
            .key("guardrailProfileIdentifier")
            .string(input.guardrail_profile_identifier.as_str());
    }
    Ok(())
}
