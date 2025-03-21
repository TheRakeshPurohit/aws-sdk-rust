// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_lake_formation_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::LakeFormationConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.authorized_session_tag_value {
        object.key("authorizedSessionTagValue").string(var_1.as_str());
    }
    if let Some(var_2) = &input.secure_namespace_info {
        #[allow(unused_mut)]
        let mut object_3 = object.key("secureNamespaceInfo").start_object();
        crate::protocol_serde::shape_secure_namespace_info::ser_secure_namespace_info(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.query_engine_role_arn {
        object.key("queryEngineRoleArn").string(var_4.as_str());
    }
    Ok(())
}

pub(crate) fn de_lake_formation_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::LakeFormationConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::LakeFormationConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "authorizedSessionTagValue" => {
                            builder = builder.set_authorized_session_tag_value(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "secureNamespaceInfo" => {
                            builder = builder
                                .set_secure_namespace_info(crate::protocol_serde::shape_secure_namespace_info::de_secure_namespace_info(tokens)?);
                        }
                        "queryEngineRoleArn" => {
                            builder = builder.set_query_engine_role_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
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
