// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_tabular_job_config(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TabularJobConfig,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.candidate_generation_config {
        #[allow(unused_mut)]
        let mut object_2 = object.key("CandidateGenerationConfig").start_object();
        crate::protocol_serde::shape_candidate_generation_config::ser_candidate_generation_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.completion_criteria {
        #[allow(unused_mut)]
        let mut object_4 = object.key("CompletionCriteria").start_object();
        crate::protocol_serde::shape_auto_ml_job_completion_criteria::ser_auto_ml_job_completion_criteria(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.feature_specification_s3_uri {
        object.key("FeatureSpecificationS3Uri").string(var_5.as_str());
    }
    if let Some(var_6) = &input.mode {
        object.key("Mode").string(var_6.as_str());
    }
    if let Some(var_7) = &input.generate_candidate_definitions_only {
        object.key("GenerateCandidateDefinitionsOnly").boolean(*var_7);
    }
    if let Some(var_8) = &input.problem_type {
        object.key("ProblemType").string(var_8.as_str());
    }
    if let Some(var_9) = &input.target_attribute_name {
        object.key("TargetAttributeName").string(var_9.as_str());
    }
    if let Some(var_10) = &input.sample_weight_attribute_name {
        object.key("SampleWeightAttributeName").string(var_10.as_str());
    }
    Ok(())
}

pub(crate) fn de_tabular_job_config<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::TabularJobConfig>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::TabularJobConfigBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "CandidateGenerationConfig" => {
                            builder = builder.set_candidate_generation_config(
                                crate::protocol_serde::shape_candidate_generation_config::de_candidate_generation_config(tokens)?,
                            );
                        }
                        "CompletionCriteria" => {
                            builder = builder.set_completion_criteria(
                                crate::protocol_serde::shape_auto_ml_job_completion_criteria::de_auto_ml_job_completion_criteria(tokens)?,
                            );
                        }
                        "FeatureSpecificationS3Uri" => {
                            builder = builder.set_feature_specification_s3_uri(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Mode" => {
                            builder = builder.set_mode(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::AutoMlMode::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "GenerateCandidateDefinitionsOnly" => {
                            builder = builder
                                .set_generate_candidate_definitions_only(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "ProblemType" => {
                            builder = builder.set_problem_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ProblemType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "TargetAttributeName" => {
                            builder = builder.set_target_attribute_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "SampleWeightAttributeName" => {
                            builder = builder.set_sample_weight_attribute_name(
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
            Ok(Some(crate::serde_util::tabular_job_config_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
