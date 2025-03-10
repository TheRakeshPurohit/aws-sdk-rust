// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_salesforce_source_properties(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SalesforceSourceProperties,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("object").string(input.object.as_str());
    }
    if input.enable_dynamic_field_update {
        object.key("enableDynamicFieldUpdate").boolean(input.enable_dynamic_field_update);
    }
    if input.include_deleted_records {
        object.key("includeDeletedRecords").boolean(input.include_deleted_records);
    }
    if let Some(var_1) = &input.data_transfer_api {
        object.key("dataTransferApi").string(var_1.as_str());
    }
    Ok(())
}

pub(crate) fn de_salesforce_source_properties<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::SalesforceSourceProperties>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::SalesforceSourcePropertiesBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "object" => {
                            builder = builder.set_object(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "enableDynamicFieldUpdate" => {
                            builder =
                                builder.set_enable_dynamic_field_update(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "includeDeletedRecords" => {
                            builder = builder.set_include_deleted_records(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "dataTransferApi" => {
                            builder = builder.set_data_transfer_api(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::SalesforceDataTransferApi::from(u.as_ref())))
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
            Ok(Some(
                crate::serde_util::salesforce_source_properties_correct_errors(builder)
                    .build()
                    .map_err(|err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err))?,
            ))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
