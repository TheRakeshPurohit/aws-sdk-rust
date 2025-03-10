// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_transform_job<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::TransformJob>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::TransformJobBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "TransformJobName" => {
                            builder = builder.set_transform_job_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TransformJobArn" => {
                            builder = builder.set_transform_job_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "TransformJobStatus" => {
                            builder = builder.set_transform_job_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::TransformJobStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "FailureReason" => {
                            builder = builder.set_failure_reason(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ModelName" => {
                            builder = builder.set_model_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "MaxConcurrentTransforms" => {
                            builder = builder.set_max_concurrent_transforms(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "ModelClientConfig" => {
                            builder =
                                builder.set_model_client_config(crate::protocol_serde::shape_model_client_config::de_model_client_config(tokens)?);
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
                        "DataCaptureConfig" => {
                            builder = builder.set_data_capture_config(
                                crate::protocol_serde::shape_batch_data_capture_config::de_batch_data_capture_config(tokens)?,
                            );
                        }
                        "TransformResources" => {
                            builder =
                                builder.set_transform_resources(crate::protocol_serde::shape_transform_resources::de_transform_resources(tokens)?);
                        }
                        "CreationTime" => {
                            builder = builder.set_creation_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "TransformStartTime" => {
                            builder = builder.set_transform_start_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "TransformEndTime" => {
                            builder = builder.set_transform_end_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "LabelingJobArn" => {
                            builder = builder.set_labeling_job_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "AutoMLJobArn" => {
                            builder = builder.set_auto_ml_job_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "DataProcessing" => {
                            builder = builder.set_data_processing(crate::protocol_serde::shape_data_processing::de_data_processing(tokens)?);
                        }
                        "ExperimentConfig" => {
                            builder = builder.set_experiment_config(crate::protocol_serde::shape_experiment_config::de_experiment_config(tokens)?);
                        }
                        "Tags" => {
                            builder = builder.set_tags(crate::protocol_serde::shape_tag_list::de_tag_list(tokens)?);
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
