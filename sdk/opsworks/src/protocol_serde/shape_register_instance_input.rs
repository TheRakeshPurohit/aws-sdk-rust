// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_register_instance_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::register_instance::RegisterInstanceInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.stack_id {
        object.key("StackId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.hostname {
        object.key("Hostname").string(var_2.as_str());
    }
    if let Some(var_3) = &input.public_ip {
        object.key("PublicIp").string(var_3.as_str());
    }
    if let Some(var_4) = &input.private_ip {
        object.key("PrivateIp").string(var_4.as_str());
    }
    if let Some(var_5) = &input.rsa_public_key {
        object.key("RsaPublicKey").string(var_5.as_str());
    }
    if let Some(var_6) = &input.rsa_public_key_fingerprint {
        object.key("RsaPublicKeyFingerprint").string(var_6.as_str());
    }
    if let Some(var_7) = &input.instance_identity {
        #[allow(unused_mut)]
        let mut object_8 = object.key("InstanceIdentity").start_object();
        crate::protocol_serde::shape_instance_identity::ser_instance_identity(&mut object_8, var_7)?;
        object_8.finish();
    }
    Ok(())
}
