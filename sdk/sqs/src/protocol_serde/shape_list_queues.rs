// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_list_queues_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::list_queues::ListQueuesOutput, crate::operation::list_queues::ListQueuesError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::list_queues::ListQueuesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::list_queues::ListQueuesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidAddress" => crate::operation::list_queues::ListQueuesError::InvalidAddress({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidAddressBuilder::default();
                output = crate::protocol_serde::shape_invalid_address::de_invalid_address_json_err(_response_body, output)
                    .map_err(crate::operation::list_queues::ListQueuesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidSecurity" => crate::operation::list_queues::ListQueuesError::InvalidSecurity({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidSecurityBuilder::default();
                output = crate::protocol_serde::shape_invalid_security::de_invalid_security_json_err(_response_body, output)
                    .map_err(crate::operation::list_queues::ListQueuesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RequestThrottled" => crate::operation::list_queues::ListQueuesError::RequestThrottled({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RequestThrottledBuilder::default();
                output = crate::protocol_serde::shape_request_throttled::de_request_throttled_json_err(_response_body, output)
                    .map_err(crate::operation::list_queues::ListQueuesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "AWS.SimpleQueueService.UnsupportedOperation" => crate::operation::list_queues::ListQueuesError::UnsupportedOperation({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnsupportedOperationBuilder::default();
                output = crate::protocol_serde::shape_unsupported_operation::de_unsupported_operation_json_err(_response_body, output)
                    .map_err(crate::operation::list_queues::ListQueuesError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::list_queues::ListQueuesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_list_queues_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::list_queues::ListQueuesOutput, crate::operation::list_queues::ListQueuesError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::list_queues::builders::ListQueuesOutputBuilder::default();
        output = crate::protocol_serde::shape_list_queues::de_list_queues(_response_body, output)
            .map_err(crate::operation::list_queues::ListQueuesError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_list_queues_input(
    input: &crate::operation::list_queues::ListQueuesInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_list_queues_input::ser_list_queues_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_list_queues(
    value: &[u8],
    mut builder: crate::operation::list_queues::builders::ListQueuesOutputBuilder,
) -> ::std::result::Result<crate::operation::list_queues::builders::ListQueuesOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
{
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "QueueUrls" => {
                    builder = builder.set_queue_urls(crate::protocol_serde::shape_queue_url_list::de_queue_url_list(tokens)?);
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
