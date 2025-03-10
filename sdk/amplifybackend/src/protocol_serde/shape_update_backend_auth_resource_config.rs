// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_backend_auth_resource_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::UpdateBackendAuthResourceConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.auth_resources {
        object.key("authResources").string(var_1.as_str());
    }
    if let Some(var_2) = &input.identity_pool_configs {
        #[allow(unused_mut)]
        let mut object_3 = object.key("identityPoolConfigs").start_object();
        crate::protocol_serde::shape_update_backend_auth_identity_pool_config::ser_update_backend_auth_identity_pool_config(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.service {
        object.key("service").string(var_4.as_str());
    }
    if let Some(var_5) = &input.user_pool_configs {
        #[allow(unused_mut)]
        let mut object_6 = object.key("userPoolConfigs").start_object();
        crate::protocol_serde::shape_update_backend_auth_user_pool_config::ser_update_backend_auth_user_pool_config(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
