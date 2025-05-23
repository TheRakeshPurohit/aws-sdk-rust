// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_crl_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_crl::UpdateCrlInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.crl_data {
        object.key("crlData").string_unchecked(&::aws_smithy_types::base64::encode(var_1));
    }
    if let Some(var_2) = &input.name {
        object.key("name").string(var_2.as_str());
    }
    Ok(())
}
