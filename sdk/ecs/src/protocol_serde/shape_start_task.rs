// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_start_task_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::start_task::StartTaskOutput, crate::operation::start_task::StartTaskError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::start_task::StartTaskError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::start_task::StartTaskError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ClientException" => crate::operation::start_task::StartTaskError::ClientException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ClientExceptionBuilder::default();
                output = crate::protocol_serde::shape_client_exception::de_client_exception_json_err(_response_body, output)
                    .map_err(crate::operation::start_task::StartTaskError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ClusterNotFoundException" => crate::operation::start_task::StartTaskError::ClusterNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ClusterNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_cluster_not_found_exception::de_cluster_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::start_task::StartTaskError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterException" => crate::operation::start_task::StartTaskError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                    .map_err(crate::operation::start_task::StartTaskError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServerException" => crate::operation::start_task::StartTaskError::ServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_server_exception::de_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::start_task::StartTaskError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnsupportedFeatureException" => crate::operation::start_task::StartTaskError::UnsupportedFeatureException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnsupportedFeatureExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_unsupported_feature_exception::de_unsupported_feature_exception_json_err(_response_body, output)
                        .map_err(crate::operation::start_task::StartTaskError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::start_task::StartTaskError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_start_task_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::start_task::StartTaskOutput, crate::operation::start_task::StartTaskError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::start_task::builders::StartTaskOutputBuilder::default();
        output = crate::protocol_serde::shape_start_task::de_start_task(_response_body, output)
            .map_err(crate::operation::start_task::StartTaskError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_start_task_input(
    input: &crate::operation::start_task::StartTaskInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_start_task_input::ser_start_task_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_start_task(
    value: &[u8],
    mut builder: crate::operation::start_task::builders::StartTaskOutputBuilder,
) -> ::std::result::Result<crate::operation::start_task::builders::StartTaskOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "tasks" => {
                    builder = builder.set_tasks(crate::protocol_serde::shape_tasks::de_tasks(tokens)?);
                }
                "failures" => {
                    builder = builder.set_failures(crate::protocol_serde::shape_failures::de_failures(tokens)?);
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
