// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_wireless_device_import_task_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_wireless_device_import_task::UpdateWirelessDeviceImportTaskInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.sidewalk {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Sidewalk").start_object();
        crate::protocol_serde::shape_sidewalk_update_import_info::ser_sidewalk_update_import_info(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
