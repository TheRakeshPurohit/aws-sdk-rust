// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_cloud_autonomous_vm_cluster_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_cloud_autonomous_vm_cluster::CreateCloudAutonomousVmClusterInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.cloud_exadata_infrastructure_id {
        object.key("cloudExadataInfrastructureId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.odb_network_id {
        object.key("odbNetworkId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.display_name {
        object.key("displayName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.client_token {
        object.key("clientToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.autonomous_data_storage_size_in_tbs {
        object.key("autonomousDataStorageSizeInTBs").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_5).into()),
        );
    }
    if let Some(var_6) = &input.cpu_core_count_per_node {
        object.key("cpuCoreCountPerNode").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    if let Some(var_7) = &input.db_servers {
        let mut array_8 = object.key("dbServers").start_array();
        for item_9 in var_7 {
            {
                array_8.value().string(item_9.as_str());
            }
        }
        array_8.finish();
    }
    if let Some(var_10) = &input.description {
        object.key("description").string(var_10.as_str());
    }
    if let Some(var_11) = &input.is_mtls_enabled_vm_cluster {
        object.key("isMtlsEnabledVmCluster").boolean(*var_11);
    }
    if let Some(var_12) = &input.license_model {
        object.key("licenseModel").string(var_12.as_str());
    }
    if let Some(var_13) = &input.maintenance_window {
        #[allow(unused_mut)]
        let mut object_14 = object.key("maintenanceWindow").start_object();
        crate::protocol_serde::shape_maintenance_window::ser_maintenance_window(&mut object_14, var_13)?;
        object_14.finish();
    }
    if let Some(var_15) = &input.memory_per_oracle_compute_unit_in_gbs {
        object.key("memoryPerOracleComputeUnitInGBs").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_15).into()),
        );
    }
    if let Some(var_16) = &input.scan_listener_port_non_tls {
        object.key("scanListenerPortNonTls").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_16).into()),
        );
    }
    if let Some(var_17) = &input.scan_listener_port_tls {
        object.key("scanListenerPortTls").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_17).into()),
        );
    }
    if let Some(var_18) = &input.tags {
        #[allow(unused_mut)]
        let mut object_19 = object.key("tags").start_object();
        for (key_20, value_21) in var_18 {
            {
                object_19.key(key_20.as_str()).string(value_21.as_str());
            }
        }
        object_19.finish();
    }
    if let Some(var_22) = &input.time_zone {
        object.key("timeZone").string(var_22.as_str());
    }
    if let Some(var_23) = &input.total_container_databases {
        object.key("totalContainerDatabases").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_23).into()),
        );
    }
    Ok(())
}
