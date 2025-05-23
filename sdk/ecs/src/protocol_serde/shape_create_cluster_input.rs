// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_cluster_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_cluster::CreateClusterInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.cluster_name {
        object.key("clusterName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.tags {
        let mut array_3 = object.key("tags").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.settings {
        let mut array_7 = object.key("settings").start_array();
        for item_8 in var_6 {
            {
                #[allow(unused_mut)]
                let mut object_9 = array_7.value().start_object();
                crate::protocol_serde::shape_cluster_setting::ser_cluster_setting(&mut object_9, item_8)?;
                object_9.finish();
            }
        }
        array_7.finish();
    }
    if let Some(var_10) = &input.configuration {
        #[allow(unused_mut)]
        let mut object_11 = object.key("configuration").start_object();
        crate::protocol_serde::shape_cluster_configuration::ser_cluster_configuration(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.capacity_providers {
        let mut array_13 = object.key("capacityProviders").start_array();
        for item_14 in var_12 {
            {
                array_13.value().string(item_14.as_str());
            }
        }
        array_13.finish();
    }
    if let Some(var_15) = &input.default_capacity_provider_strategy {
        let mut array_16 = object.key("defaultCapacityProviderStrategy").start_array();
        for item_17 in var_15 {
            {
                #[allow(unused_mut)]
                let mut object_18 = array_16.value().start_object();
                crate::protocol_serde::shape_capacity_provider_strategy_item::ser_capacity_provider_strategy_item(&mut object_18, item_17)?;
                object_18.finish();
            }
        }
        array_16.finish();
    }
    if let Some(var_19) = &input.service_connect_defaults {
        #[allow(unused_mut)]
        let mut object_20 = object.key("serviceConnectDefaults").start_object();
        crate::protocol_serde::shape_cluster_service_connect_defaults_request::ser_cluster_service_connect_defaults_request(&mut object_20, var_19)?;
        object_20.finish();
    }
    Ok(())
}
