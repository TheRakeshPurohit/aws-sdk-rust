// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_cluster_restricted_instance_group_specification(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ClusterRestrictedInstanceGroupSpecification,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.instance_count {
        object.key("InstanceCount").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.instance_group_name {
        object.key("InstanceGroupName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.instance_type {
        object.key("InstanceType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.execution_role {
        object.key("ExecutionRole").string(var_4.as_str());
    }
    if let Some(var_5) = &input.threads_per_core {
        object.key("ThreadsPerCore").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.instance_storage_configs {
        let mut array_7 = object.key("InstanceStorageConfigs").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_cluster_instance_storage_config::ser_cluster_instance_storage_config(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.on_start_deep_health_checks {
        let mut array_11 = object.key("OnStartDeepHealthChecks").start_array();
        for item_12 in var_10 {
            {
                array_11.value().string(item_12.as_str());
            }
        }
        array_11.finish();
    }
    if let Some(var_13) = &input.training_plan_arn {
        object.key("TrainingPlanArn").string(var_13.as_str());
    }
    if let Some(var_14) = &input.override_vpc_config {
        #[allow(unused_mut)]
        let mut object_15 = object.key("OverrideVpcConfig").start_object();
        crate::protocol_serde::shape_vpc_config::ser_vpc_config(&mut object_15, var_14)?;
        object_15.finish();
    }
    if let Some(var_16) = &input.scheduled_update_config {
        #[allow(unused_mut)]
        let mut object_17 = object.key("ScheduledUpdateConfig").start_object();
        crate::protocol_serde::shape_scheduled_update_config::ser_scheduled_update_config(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.environment_config {
        #[allow(unused_mut)]
        let mut object_19 = object.key("EnvironmentConfig").start_object();
        crate::protocol_serde::shape_environment_config::ser_environment_config(&mut object_19, var_18)?;
        object_19.finish();
    }
    Ok(())
}
