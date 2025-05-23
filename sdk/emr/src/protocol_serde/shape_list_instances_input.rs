// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_instances_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_instances::ListInstancesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.cluster_id {
        object.key("ClusterId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.instance_group_id {
        object.key("InstanceGroupId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.instance_group_types {
        let mut array_4 = object.key("InstanceGroupTypes").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.instance_fleet_id {
        object.key("InstanceFleetId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.instance_fleet_type {
        object.key("InstanceFleetType").string(var_7.as_str());
    }
    if let Some(var_8) = &input.instance_states {
        let mut array_9 = object.key("InstanceStates").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.marker {
        object.key("Marker").string(var_11.as_str());
    }
    Ok(())
}
