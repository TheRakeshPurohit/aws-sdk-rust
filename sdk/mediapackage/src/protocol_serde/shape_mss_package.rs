// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_mss_package<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::MssPackage>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::MssPackageBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "encryption" => {
                            builder = builder.set_encryption(crate::protocol_serde::shape_mss_encryption::de_mss_encryption(tokens)?);
                        }
                        "manifestWindowSeconds" => {
                            builder = builder.set_manifest_window_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "segmentDurationSeconds" => {
                            builder = builder.set_segment_duration_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "streamSelection" => {
                            builder = builder.set_stream_selection(crate::protocol_serde::shape_stream_selection::de_stream_selection(tokens)?);
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

pub fn ser_mss_package(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MssPackage,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.encryption {
        #[allow(unused_mut)]
        let mut object_2 = object.key("encryption").start_object();
        crate::protocol_serde::shape_mss_encryption::ser_mss_encryption(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.manifest_window_seconds {
        object.key("manifestWindowSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.segment_duration_seconds {
        object.key("segmentDurationSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.stream_selection {
        #[allow(unused_mut)]
        let mut object_6 = object.key("streamSelection").start_object();
        crate::protocol_serde::shape_stream_selection::ser_stream_selection(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}
