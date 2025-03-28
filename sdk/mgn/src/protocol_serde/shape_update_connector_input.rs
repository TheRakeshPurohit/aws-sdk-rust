// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_connector_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_connector::UpdateConnectorInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.connector_id {
        object.key("connectorID").string(var_1.as_str());
    }
    if let Some(var_2) = &input.name {
        object.key("name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.ssm_command_config {
        #[allow(unused_mut)]
        let mut object_4 = object.key("ssmCommandConfig").start_object();
        crate::protocol_serde::shape_connector_ssm_command_config::ser_connector_ssm_command_config(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
