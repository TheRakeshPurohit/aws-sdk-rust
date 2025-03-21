// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_intent_closing_setting<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::IntentClosingSetting>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::IntentClosingSettingBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "closingResponse" => {
                            builder =
                                builder.set_closing_response(crate::protocol_serde::shape_response_specification::de_response_specification(tokens)?);
                        }
                        "active" => {
                            builder = builder.set_active(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "nextStep" => {
                            builder = builder.set_next_step(crate::protocol_serde::shape_dialog_state::de_dialog_state(tokens)?);
                        }
                        "conditional" => {
                            builder = builder.set_conditional(crate::protocol_serde::shape_conditional_specification::de_conditional_specification(
                                tokens,
                            )?);
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

pub fn ser_intent_closing_setting(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::IntentClosingSetting,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.closing_response {
        #[allow(unused_mut)]
        let mut object_2 = object.key("closingResponse").start_object();
        crate::protocol_serde::shape_response_specification::ser_response_specification(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.active {
        object.key("active").boolean(*var_3);
    }
    if let Some(var_4) = &input.next_step {
        #[allow(unused_mut)]
        let mut object_5 = object.key("nextStep").start_object();
        crate::protocol_serde::shape_dialog_state::ser_dialog_state(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.conditional {
        #[allow(unused_mut)]
        let mut object_7 = object.key("conditional").start_object();
        crate::protocol_serde::shape_conditional_specification::ser_conditional_specification(&mut object_7, var_6)?;
        object_7.finish();
    }
    Ok(())
}
