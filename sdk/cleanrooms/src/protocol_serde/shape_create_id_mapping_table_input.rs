// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_id_mapping_table_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_id_mapping_table::CreateIdMappingTableInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.input_reference_config {
        #[allow(unused_mut)]
        let mut object_3 = object.key("inputReferenceConfig").start_object();
        crate::protocol_serde::shape_id_mapping_table_input_reference_config::ser_id_mapping_table_input_reference_config(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.kms_key_arn {
        object.key("kmsKeyArn").string(var_4.as_str());
    }
    if let Some(var_5) = &input.name {
        object.key("name").string(var_5.as_str());
    }
    if let Some(var_6) = &input.tags {
        #[allow(unused_mut)]
        let mut object_7 = object.key("tags").start_object();
        for (key_8, value_9) in var_6 {
            {
                object_7.key(key_8.as_str()).string(value_9.as_str());
            }
        }
        object_7.finish();
    }
    Ok(())
}
