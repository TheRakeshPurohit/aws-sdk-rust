// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_send_serial_console_ssh_public_key_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::send_serial_console_ssh_public_key::SendSerialConsoleSshPublicKeyInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.instance_id {
        object.key("InstanceId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.serial_port {
        object.key("SerialPort").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.ssh_public_key {
        object.key("SSHPublicKey").string(var_3.as_str());
    }
    Ok(())
}
