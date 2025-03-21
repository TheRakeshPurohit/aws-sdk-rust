// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_caption_format(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CaptionFormat,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.format {
        object.key("Format").string(var_1.as_str());
    }
    if let Some(var_2) = &input.pattern {
        object.key("Pattern").string(var_2.as_str());
    }
    if let Some(var_3) = &input.encryption {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Encryption").start_object();
        crate::protocol_serde::shape_encryption::ser_encryption(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_caption_format<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::CaptionFormat>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::CaptionFormatBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Format" => {
                            builder = builder.set_format(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Pattern" => {
                            builder = builder.set_pattern(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Encryption" => {
                            builder = builder.set_encryption(crate::protocol_serde::shape_encryption::de_encryption(tokens)?);
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
