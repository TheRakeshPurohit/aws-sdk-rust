// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_selfservice_permissions(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SelfservicePermissions,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.restart_workspace {
        object.key("RestartWorkspace").string(var_1.as_str());
    }
    if let Some(var_2) = &input.increase_volume_size {
        object.key("IncreaseVolumeSize").string(var_2.as_str());
    }
    if let Some(var_3) = &input.change_compute_type {
        object.key("ChangeComputeType").string(var_3.as_str());
    }
    if let Some(var_4) = &input.switch_running_mode {
        object.key("SwitchRunningMode").string(var_4.as_str());
    }
    if let Some(var_5) = &input.rebuild_workspace {
        object.key("RebuildWorkspace").string(var_5.as_str());
    }
    Ok(())
}

pub(crate) fn de_selfservice_permissions<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::SelfservicePermissions>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::SelfservicePermissionsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "RestartWorkspace" => {
                            builder = builder.set_restart_workspace(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ReconnectEnum::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "IncreaseVolumeSize" => {
                            builder = builder.set_increase_volume_size(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ReconnectEnum::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "ChangeComputeType" => {
                            builder = builder.set_change_compute_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ReconnectEnum::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "SwitchRunningMode" => {
                            builder = builder.set_switch_running_mode(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ReconnectEnum::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "RebuildWorkspace" => {
                            builder = builder.set_rebuild_workspace(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ReconnectEnum::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
                    },
                    other => {
                        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                            "expected object key or end object, found: {:?}",
                            other
                        )))
                    }
                }
            }
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
