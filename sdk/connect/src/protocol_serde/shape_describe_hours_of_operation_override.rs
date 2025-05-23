// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_hours_of_operation_override_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideOutput,
    crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceException" => {
            crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::InternalServiceException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InternalServiceExceptionBuilder::default();
                    output = crate::protocol_serde::shape_internal_service_exception::de_internal_service_exception_json_err(_response_body, output)
                        .map_err(crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidParameterException" => {
            crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::InvalidParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                            .map_err(crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidRequestException" => {
            crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::InvalidRequestException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                    output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                        .map_err(crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ResourceNotFoundException" => {
            crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::ResourceNotFoundException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                            .map_err(crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ThrottlingException" => {
            crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::ThrottlingException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                    output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                        .map_err(crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_hours_of_operation_override_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideOutput,
    crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_hours_of_operation_override::builders::DescribeHoursOfOperationOverrideOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_hours_of_operation_override::de_describe_hours_of_operation_override(_response_body, output)
            .map_err(crate::operation::describe_hours_of_operation_override::DescribeHoursOfOperationOverrideError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub(crate) fn de_describe_hours_of_operation_override(
    value: &[u8],
    mut builder: crate::operation::describe_hours_of_operation_override::builders::DescribeHoursOfOperationOverrideOutputBuilder,
) -> ::std::result::Result<
    crate::operation::describe_hours_of_operation_override::builders::DescribeHoursOfOperationOverrideOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "HoursOfOperationOverride" => {
                    builder = builder.set_hours_of_operation_override(
                        crate::protocol_serde::shape_hours_of_operation_override::de_hours_of_operation_override(tokens)?,
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
