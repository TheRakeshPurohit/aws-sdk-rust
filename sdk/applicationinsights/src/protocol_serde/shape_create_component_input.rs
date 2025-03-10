// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_component_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_component::CreateComponentInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.resource_group_name {
        object.key("ResourceGroupName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.component_name {
        object.key("ComponentName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.resource_list {
        let mut array_4 = object.key("ResourceList").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    Ok(())
}
