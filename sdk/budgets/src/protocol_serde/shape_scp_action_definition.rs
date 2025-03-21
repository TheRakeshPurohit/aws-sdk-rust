// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_scp_action_definition(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ScpActionDefinition,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("PolicyId").string(input.policy_id.as_str());
    }
    {
        let mut array_1 = object.key("TargetIds").start_array();
        for item_2 in &input.target_ids {
            {
                array_1.value().string(item_2.as_str());
            }
        }
        array_1.finish();
    }
    Ok(())
}

pub(crate) fn de_scp_action_definition<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ScpActionDefinition>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ScpActionDefinitionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "PolicyId" => {
                            builder = builder.set_policy_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TargetIds" => {
                            builder = builder.set_target_ids(crate::protocol_serde::shape_target_ids::de_target_ids(tokens)?);
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
            Ok(Some(crate::serde_util::scp_action_definition_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
