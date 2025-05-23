// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_anomaly_detection_executions_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsOutput,
    crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                        .map_err(crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::access_denied_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::unhandled)?
                };
                tmp
            })
        }
        "InternalServerException" => {
            crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::InternalServerException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                        .map_err(crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::internal_server_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::unhandled)?
                };
                tmp
            })
        }
        "ResourceNotFoundException" => {
            crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::resource_not_found_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::unhandled)?
                };
                tmp
            })
        }
        "TooManyRequestsException" => {
            crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::TooManyRequestsException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                            .map_err(crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::too_many_requests_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::unhandled)?
                };
                tmp
            })
        }
        "ValidationException" => {
            crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                        .map_err(crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::unhandled)?;
                    let output = output.meta(generic);
                    crate::serde_util::validation_exception_correct_errors(output)
                        .build()
                        .map_err(crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::unhandled)?
                };
                tmp
            })
        }
        _ => crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_anomaly_detection_executions_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsOutput,
    crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::describe_anomaly_detection_executions::builders::DescribeAnomalyDetectionExecutionsOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_anomaly_detection_executions::de_describe_anomaly_detection_executions(_response_body, output)
            .map_err(crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_describe_anomaly_detection_executions_input(
    input: &crate::operation::describe_anomaly_detection_executions::DescribeAnomalyDetectionExecutionsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_anomaly_detection_executions_input::ser_describe_anomaly_detection_executions_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_describe_anomaly_detection_executions(
    value: &[u8],
    mut builder: crate::operation::describe_anomaly_detection_executions::builders::DescribeAnomalyDetectionExecutionsOutputBuilder,
) -> ::std::result::Result<
    crate::operation::describe_anomaly_detection_executions::builders::DescribeAnomalyDetectionExecutionsOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "ExecutionList" => {
                    builder = builder.set_execution_list(crate::protocol_serde::shape_execution_list::de_execution_list(tokens)?);
                }
                "NextToken" => {
                    builder = builder.set_next_token(
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
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
