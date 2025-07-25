// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_whats_app_message_template_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_whats_app_message_template::CreateWhatsAppMessageTemplateInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.id {
        object.key("id").string(var_1.as_str());
    }
    if let Some(var_2) = &input.template_definition {
        object
            .key("templateDefinition")
            .string_unchecked(&::aws_smithy_types::base64::encode(var_2));
    }
    Ok(())
}
