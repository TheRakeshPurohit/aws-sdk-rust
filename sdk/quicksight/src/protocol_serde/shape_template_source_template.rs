// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_template_source_template(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TemplateSourceTemplate,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("Arn").string(input.arn.as_str());
    }
    Ok(())
}
