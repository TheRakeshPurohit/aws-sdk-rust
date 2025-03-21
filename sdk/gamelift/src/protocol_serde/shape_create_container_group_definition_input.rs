// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_container_group_definition_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_container_group_definition::CreateContainerGroupDefinitionInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.container_group_type {
        object.key("ContainerGroupType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.total_memory_limit_mebibytes {
        object.key("TotalMemoryLimitMebibytes").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.total_vcpu_limit {
        object.key("TotalVcpuLimit").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.game_server_container_definition {
        #[allow(unused_mut)]
        let mut object_6 = object.key("GameServerContainerDefinition").start_object();
        crate::protocol_serde::shape_game_server_container_definition_input::ser_game_server_container_definition_input(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.support_container_definitions {
        let mut array_8 = object.key("SupportContainerDefinitions").start_array();
        for item_9 in var_7 {
            {
                #[allow(unused_mut)]
                let mut object_10 = array_8.value().start_object();
                crate::protocol_serde::shape_support_container_definition_input::ser_support_container_definition_input(&mut object_10, item_9)?;
                object_10.finish();
            }
        }
        array_8.finish();
    }
    if let Some(var_11) = &input.operating_system {
        object.key("OperatingSystem").string(var_11.as_str());
    }
    if let Some(var_12) = &input.version_description {
        object.key("VersionDescription").string(var_12.as_str());
    }
    if let Some(var_13) = &input.tags {
        let mut array_14 = object.key("Tags").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_16, item_15)?;
                object_16.finish();
            }
        }
        array_14.finish();
    }
    Ok(())
}
