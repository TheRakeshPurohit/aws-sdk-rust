// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_cluster_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_cluster::CreateClusterInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.backup_retention_policy {
        #[allow(unused_mut)]
        let mut object_2 = object.key("BackupRetentionPolicy").start_object();
        crate::protocol_serde::shape_backup_retention_policy::ser_backup_retention_policy(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.hsm_type {
        object.key("HsmType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.source_backup_id {
        object.key("SourceBackupId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.subnet_ids {
        let mut array_6 = object.key("SubnetIds").start_array();
        for item_7 in var_5 {
            {
                array_6.value().string(item_7.as_str());
            }
        }
        array_6.finish();
    }
    if let Some(var_8) = &input.network_type {
        object.key("NetworkType").string(var_8.as_str());
    }
    if let Some(var_9) = &input.tag_list {
        let mut array_10 = object.key("TagList").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_tag::ser_tag(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.mode {
        object.key("Mode").string(var_13.as_str());
    }
    Ok(())
}
