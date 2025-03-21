// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_multi_region_cluster_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_multi_region_cluster::UpdateMultiRegionClusterInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.multi_region_cluster_name {
        object.key("MultiRegionClusterName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.node_type {
        object.key("NodeType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.description {
        object.key("Description").string(var_3.as_str());
    }
    if let Some(var_4) = &input.engine_version {
        object.key("EngineVersion").string(var_4.as_str());
    }
    if let Some(var_5) = &input.shard_configuration {
        #[allow(unused_mut)]
        let mut object_6 = object.key("ShardConfiguration").start_object();
        crate::protocol_serde::shape_shard_configuration_request::ser_shard_configuration_request(&mut object_6, var_5)?;
        object_6.finish();
    }
    if let Some(var_7) = &input.multi_region_parameter_group_name {
        object.key("MultiRegionParameterGroupName").string(var_7.as_str());
    }
    if let Some(var_8) = &input.update_strategy {
        object.key("UpdateStrategy").string(var_8.as_str());
    }
    Ok(())
}
