// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_media_storage_configuration_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_media_storage_configuration::UpdateMediaStorageConfigurationInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.channel_arn {
        object.key("ChannelARN").string(var_1.as_str());
    }
    if let Some(var_2) = &input.media_storage_configuration {
        #[allow(unused_mut)]
        let mut object_3 = object.key("MediaStorageConfiguration").start_object();
        crate::protocol_serde::shape_media_storage_configuration::ser_media_storage_configuration(&mut object_3, var_2)?;
        object_3.finish();
    }
    Ok(())
}
