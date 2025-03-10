// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_code_repository(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::CodeRepository,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("RepositoryUrl").string(input.repository_url.as_str());
    }
    if let Some(var_1) = &input.source_code_version {
        #[allow(unused_mut)]
        let mut object_2 = object.key("SourceCodeVersion").start_object();
        crate::protocol_serde::shape_source_code_version::ser_source_code_version(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.code_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("CodeConfiguration").start_object();
        crate::protocol_serde::shape_code_configuration::ser_code_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.source_directory {
        object.key("SourceDirectory").string(var_5.as_str());
    }
    Ok(())
}

pub(crate) fn de_code_repository<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::CodeRepository>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::CodeRepositoryBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "RepositoryUrl" => {
                            builder = builder.set_repository_url(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "SourceCodeVersion" => {
                            builder =
                                builder.set_source_code_version(crate::protocol_serde::shape_source_code_version::de_source_code_version(tokens)?);
                        }
                        "CodeConfiguration" => {
                            builder = builder.set_code_configuration(crate::protocol_serde::shape_code_configuration::de_code_configuration(tokens)?);
                        }
                        "SourceDirectory" => {
                            builder = builder.set_source_directory(
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
            Ok(Some(crate::serde_util::code_repository_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
