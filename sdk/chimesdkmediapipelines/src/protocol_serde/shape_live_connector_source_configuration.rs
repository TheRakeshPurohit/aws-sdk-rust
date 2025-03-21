// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_live_connector_source_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::LiveConnectorSourceConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("SourceType").string(input.source_type.as_str());
    }
    if let Some(var_1) = &input.chime_sdk_meeting_live_connector_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("ChimeSdkMeetingLiveConnectorConfiguration").start_object();
        crate::protocol_serde::shape_chime_sdk_meeting_live_connector_configuration::ser_chime_sdk_meeting_live_connector_configuration(
            &mut object_2,
            var_1,
        )?;
        object_2.finish();
    }
    Ok(())
}

pub(crate) fn de_live_connector_source_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::LiveConnectorSourceConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::LiveConnectorSourceConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "SourceType" => {
                            builder = builder.set_source_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::LiveConnectorSourceType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "ChimeSdkMeetingLiveConnectorConfiguration" => {
                            builder = builder.set_chime_sdk_meeting_live_connector_configuration(
                                    crate::protocol_serde::shape_chime_sdk_meeting_live_connector_configuration::de_chime_sdk_meeting_live_connector_configuration(tokens)?
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
            Ok(Some(
                crate::serde_util::live_connector_source_configuration_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
