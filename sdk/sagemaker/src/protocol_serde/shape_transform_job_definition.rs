// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_transform_job_definition(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TransformJobDefinition,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.max_concurrent_transforms {
        object.key("MaxConcurrentTransforms").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.max_payload_in_mb {
        object.key("MaxPayloadInMB").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    if let Some(var_3) = &input.batch_strategy {
        object.key("BatchStrategy").string(var_3.as_str());
    }
    if let Some(var_4) = &input.environment {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Environment").start_object();
        for (key_6, value_7) in var_4 {
            {
                object_5.key(key_6.as_str()).string(value_7.as_str());
            }
        }
        object_5.finish();
    }
    if let Some(var_8) = &input.transform_input {
        #[allow(unused_mut)]
        let mut object_9 = object.key("TransformInput").start_object();
        crate::protocol_serde::shape_transform_input::ser_transform_input(&mut object_9, var_8)?;
        object_9.finish();
    }
    if let Some(var_10) = &input.transform_output {
        #[allow(unused_mut)]
        let mut object_11 = object.key("TransformOutput").start_object();
        crate::protocol_serde::shape_transform_output::ser_transform_output(&mut object_11, var_10)?;
        object_11.finish();
    }
    if let Some(var_12) = &input.transform_resources {
        #[allow(unused_mut)]
        let mut object_13 = object.key("TransformResources").start_object();
        crate::protocol_serde::shape_transform_resources::ser_transform_resources(&mut object_13, var_12)?;
        object_13.finish();
    }
    Ok(())
}

pub(crate) fn de_transform_job_definition<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::TransformJobDefinition>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::TransformJobDefinitionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "MaxConcurrentTransforms" => {
                            builder = builder.set_max_concurrent_transforms(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "MaxPayloadInMB" => {
                            builder = builder.set_max_payload_in_mb(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "BatchStrategy" => {
                            builder = builder.set_batch_strategy(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::BatchStrategy::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "Environment" => {
                            builder = builder.set_environment(crate::protocol_serde::shape_transform_environment_map::de_transform_environment_map(
                                tokens,
                            )?);
                        }
                        "TransformInput" => {
                            builder = builder.set_transform_input(crate::protocol_serde::shape_transform_input::de_transform_input(tokens)?);
                        }
                        "TransformOutput" => {
                            builder = builder.set_transform_output(crate::protocol_serde::shape_transform_output::de_transform_output(tokens)?);
                        }
                        "TransformResources" => {
                            builder =
                                builder.set_transform_resources(crate::protocol_serde::shape_transform_resources::de_transform_resources(tokens)?);
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
            Ok(Some(crate::serde_util::transform_job_definition_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
