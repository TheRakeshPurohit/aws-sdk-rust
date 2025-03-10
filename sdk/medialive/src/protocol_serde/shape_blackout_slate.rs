// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_blackout_slate(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::BlackoutSlate,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.blackout_slate_image {
        #[allow(unused_mut)]
        let mut object_2 = object.key("blackoutSlateImage").start_object();
        crate::protocol_serde::shape_input_location::ser_input_location(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.network_end_blackout {
        object.key("networkEndBlackout").string(var_3.as_str());
    }
    if let Some(var_4) = &input.network_end_blackout_image {
        #[allow(unused_mut)]
        let mut object_5 = object.key("networkEndBlackoutImage").start_object();
        crate::protocol_serde::shape_input_location::ser_input_location(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.network_id {
        object.key("networkId").string(var_6.as_str());
    }
    if let Some(var_7) = &input.state {
        object.key("state").string(var_7.as_str());
    }
    Ok(())
}

pub(crate) fn de_blackout_slate<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::BlackoutSlate>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::BlackoutSlateBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "blackoutSlateImage" => {
                            builder = builder.set_blackout_slate_image(crate::protocol_serde::shape_input_location::de_input_location(tokens)?);
                        }
                        "networkEndBlackout" => {
                            builder = builder.set_network_end_blackout(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::BlackoutSlateNetworkEndBlackout::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "networkEndBlackoutImage" => {
                            builder = builder.set_network_end_blackout_image(crate::protocol_serde::shape_input_location::de_input_location(tokens)?);
                        }
                        "networkId" => {
                            builder = builder.set_network_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "state" => {
                            builder = builder.set_state(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::BlackoutSlateState::from(u.as_ref())))
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
