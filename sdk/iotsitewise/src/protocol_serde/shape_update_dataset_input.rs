// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_dataset_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_dataset::UpdateDatasetInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.client_token {
        object.key("clientToken").string(var_1.as_str());
    }
    if let Some(var_2) = &input.dataset_description {
        object.key("datasetDescription").string(var_2.as_str());
    }
    if let Some(var_3) = &input.dataset_name {
        object.key("datasetName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.dataset_source {
        #[allow(unused_mut)]
        let mut object_5 = object.key("datasetSource").start_object();
        crate::protocol_serde::shape_dataset_source::ser_dataset_source(&mut object_5, var_4)?;
        object_5.finish();
    }
    Ok(())
}
