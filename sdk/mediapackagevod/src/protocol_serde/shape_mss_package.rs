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
                        "mssManifests" => {
                            builder = builder.set_mss_manifests(crate::protocol_serde::shape_list_of_mss_manifest::de_list_of_mss_manifest(tokens)?);
                        }
                        "segmentDurationSeconds" => {
                            builder = builder.set_segment_duration_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
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
            Ok(Some(crate::serde_util::mss_package_correct_errors(builder).build()))
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
    if let Some(var_3) = &input.mss_manifests {
        let mut array_4 = object.key("mssManifests").start_array();
        for item_5 in var_3 {
            {
                #[allow(unused_mut)]
                let mut object_6 = array_4.value().start_object();
                crate::protocol_serde::shape_mss_manifest::ser_mss_manifest(&mut object_6, item_5)?;
                object_6.finish();
            }
        }
        array_4.finish();
    }
    if let Some(var_7) = &input.segment_duration_seconds {
        object.key("segmentDurationSeconds").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    Ok(())
}
