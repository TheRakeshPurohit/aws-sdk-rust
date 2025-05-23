// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_optimization_job_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_optimization_job::DescribeOptimizationJobOutput,
    crate::operation::describe_optimization_job::DescribeOptimizationJobError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_optimization_job::DescribeOptimizationJobError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::describe_optimization_job::DescribeOptimizationJobError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFound" => crate::operation::describe_optimization_job::DescribeOptimizationJobError::ResourceNotFound({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found::de_resource_not_found_json_err(_response_body, output)
                    .map_err(crate::operation::describe_optimization_job::DescribeOptimizationJobError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::describe_optimization_job::DescribeOptimizationJobError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_optimization_job_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_optimization_job::DescribeOptimizationJobOutput,
    crate::operation::describe_optimization_job::DescribeOptimizationJobError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_optimization_job::builders::DescribeOptimizationJobOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_optimization_job::de_describe_optimization_job(_response_body, output)
            .map_err(crate::operation::describe_optimization_job::DescribeOptimizationJobError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::describe_optimization_job_output_output_correct_errors(output).build()
    })
}

pub fn ser_describe_optimization_job_input(
    input: &crate::operation::describe_optimization_job::DescribeOptimizationJobInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_optimization_job_input::ser_describe_optimization_job_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_describe_optimization_job(
    value: &[u8],
    mut builder: crate::operation::describe_optimization_job::builders::DescribeOptimizationJobOutputBuilder,
) -> ::std::result::Result<
    crate::operation::describe_optimization_job::builders::DescribeOptimizationJobOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "OptimizationJobArn" => {
                    builder = builder.set_optimization_job_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "OptimizationJobStatus" => {
                    builder = builder.set_optimization_job_status(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::OptimizationJobStatus::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "OptimizationStartTime" => {
                    builder = builder.set_optimization_start_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "OptimizationEndTime" => {
                    builder = builder.set_optimization_end_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "CreationTime" => {
                    builder = builder.set_creation_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "LastModifiedTime" => {
                    builder = builder.set_last_modified_time(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "FailureReason" => {
                    builder = builder.set_failure_reason(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "OptimizationJobName" => {
                    builder = builder.set_optimization_job_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "ModelSource" => {
                    builder = builder
                        .set_model_source(crate::protocol_serde::shape_optimization_job_model_source::de_optimization_job_model_source(tokens)?);
                }
                "OptimizationEnvironment" => {
                    builder = builder.set_optimization_environment(
                        crate::protocol_serde::shape_optimization_job_environment_variables::de_optimization_job_environment_variables(tokens)?,
                    );
                }
                "DeploymentInstanceType" => {
                    builder = builder.set_deployment_instance_type(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| {
                                s.to_unescaped()
                                    .map(|u| crate::types::OptimizationJobDeploymentInstanceType::from(u.as_ref()))
                            })
                            .transpose()?,
                    );
                }
                "OptimizationConfigs" => {
                    builder = builder.set_optimization_configs(crate::protocol_serde::shape_optimization_configs::de_optimization_configs(tokens)?);
                }
                "OutputConfig" => {
                    builder = builder
                        .set_output_config(crate::protocol_serde::shape_optimization_job_output_config::de_optimization_job_output_config(tokens)?);
                }
                "OptimizationOutput" => {
                    builder = builder.set_optimization_output(crate::protocol_serde::shape_optimization_output::de_optimization_output(tokens)?);
                }
                "RoleArn" => {
                    builder = builder.set_role_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "StoppingCondition" => {
                    builder = builder.set_stopping_condition(crate::protocol_serde::shape_stopping_condition::de_stopping_condition(tokens)?);
                }
                "VpcConfig" => {
                    builder = builder.set_vpc_config(crate::protocol_serde::shape_optimization_vpc_config::de_optimization_vpc_config(tokens)?);
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
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
