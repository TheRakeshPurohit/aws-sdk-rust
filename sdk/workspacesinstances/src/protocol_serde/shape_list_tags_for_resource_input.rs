// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_tags_for_resource_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_tags_for_resource::ListTagsForResourceInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.workspace_instance_id {
        object.key("WorkspaceInstanceId").string(var_1.as_str());
    }
    Ok(())
}
