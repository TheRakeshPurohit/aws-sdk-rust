// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_mount_target_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_mount_target::CreateMountTargetInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.file_system_id {
        object.key("FileSystemId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.ip_address {
        object.key("IpAddress").string(var_2.as_str());
    }
    if let Some(var_3) = &input.ip_address_type {
        object.key("IpAddressType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.ipv6_address {
        object.key("Ipv6Address").string(var_4.as_str());
    }
    if let Some(var_5) = &input.security_groups {
        let mut array_6 = object.key("SecurityGroups").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.subnet_id {
        object.key("SubnetId").string(var_8.as_str());
    }
    Ok(())
}
