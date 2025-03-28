// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_instance_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_instance::CreateInstanceInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("ClientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.directory_id {
        object.key("DirectoryId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.identity_management_type {
        object.key("IdentityManagementType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.inbound_calls_enabled {
        object.key("InboundCallsEnabled").boolean(*var_4);
    }
    if let Some(var_5) = &input.instance_alias {
        object.key("InstanceAlias").string(var_5.as_str());
    }
    if let Some(var_6) = &input.outbound_calls_enabled {
        object.key("OutboundCallsEnabled").boolean(*var_6);
    }
    if let Some(var_7) = &input.tags {
        #[allow(unused_mut)]
        let mut object_8 = object.key("Tags").start_object();
        for (key_9, value_10) in var_7 {
            {
                object_8.key(key_9.as_str()).string(value_10.as_str());
            }
        }
        object_8.finish();
    }
    Ok(())
}
