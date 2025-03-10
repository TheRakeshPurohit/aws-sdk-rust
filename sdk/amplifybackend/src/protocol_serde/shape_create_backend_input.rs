// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_backend_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_backend::CreateBackendInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.app_id {
        object.key("appId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.app_name {
        object.key("appName").string(var_2.as_str());
    }
    if let Some(var_3) = &input.backend_environment_name {
        object.key("backendEnvironmentName").string(var_3.as_str());
    }
    if let Some(var_4) = &input.resource_config {
        #[allow(unused_mut)]
        let mut object_5 = object.key("resourceConfig").start_object();
        crate::protocol_serde::shape_resource_config::ser_resource_config(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.resource_name {
        object.key("resourceName").string(var_6.as_str());
    }
    Ok(())
}
