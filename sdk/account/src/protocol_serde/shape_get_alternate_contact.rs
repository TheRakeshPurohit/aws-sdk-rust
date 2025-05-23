// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_alternate_contact_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_alternate_contact::GetAlternateContactOutput,
    crate::operation::get_alternate_contact::GetAlternateContactError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_alternate_contact::GetAlternateContactError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_alternate_contact::GetAlternateContactError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => crate::operation::get_alternate_contact::GetAlternateContactError::AccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_alternate_contact::GetAlternateContactError::unhandled)?;
                output = output.set_error_type(
                    crate::protocol_serde::shape_access_denied_exception::de_error_type_header(_response_headers).map_err(|_| {
                        crate::operation::get_alternate_contact::GetAlternateContactError::unhandled(
                            "Failed to parse errorType from header `x-amzn-ErrorType",
                        )
                    })?,
                );
                let output = output.meta(generic);
                crate::serde_util::access_denied_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_alternate_contact::GetAlternateContactError::unhandled)?
            };
            tmp
        }),
        "InternalServerException" => crate::operation::get_alternate_contact::GetAlternateContactError::InternalServerException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServerExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_server_exception::de_internal_server_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_alternate_contact::GetAlternateContactError::unhandled)?;
                output = output.set_error_type(
                    crate::protocol_serde::shape_internal_server_exception::de_error_type_header(_response_headers).map_err(|_| {
                        crate::operation::get_alternate_contact::GetAlternateContactError::unhandled(
                            "Failed to parse errorType from header `x-amzn-ErrorType",
                        )
                    })?,
                );
                let output = output.meta(generic);
                crate::serde_util::internal_server_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_alternate_contact::GetAlternateContactError::unhandled)?
            };
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::get_alternate_contact::GetAlternateContactError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_alternate_contact::GetAlternateContactError::unhandled)?;
                output = output.set_error_type(
                    crate::protocol_serde::shape_resource_not_found_exception::de_error_type_header(_response_headers).map_err(|_| {
                        crate::operation::get_alternate_contact::GetAlternateContactError::unhandled(
                            "Failed to parse errorType from header `x-amzn-ErrorType",
                        )
                    })?,
                );
                let output = output.meta(generic);
                crate::serde_util::resource_not_found_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_alternate_contact::GetAlternateContactError::unhandled)?
            };
            tmp
        }),
        "TooManyRequestsException" => crate::operation::get_alternate_contact::GetAlternateContactError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_alternate_contact::GetAlternateContactError::unhandled)?;
                output = output.set_error_type(
                    crate::protocol_serde::shape_too_many_requests_exception::de_error_type_header(_response_headers).map_err(|_| {
                        crate::operation::get_alternate_contact::GetAlternateContactError::unhandled(
                            "Failed to parse errorType from header `x-amzn-ErrorType",
                        )
                    })?,
                );
                let output = output.meta(generic);
                crate::serde_util::too_many_requests_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_alternate_contact::GetAlternateContactError::unhandled)?
            };
            tmp
        }),
        "ValidationException" => crate::operation::get_alternate_contact::GetAlternateContactError::ValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_alternate_contact::GetAlternateContactError::unhandled)?;
                let output = output.meta(generic);
                crate::serde_util::validation_exception_correct_errors(output)
                    .build()
                    .map_err(crate::operation::get_alternate_contact::GetAlternateContactError::unhandled)?
            };
            tmp
        }),
        _ => crate::operation::get_alternate_contact::GetAlternateContactError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_alternate_contact_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_alternate_contact::GetAlternateContactOutput,
    crate::operation::get_alternate_contact::GetAlternateContactError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_alternate_contact::builders::GetAlternateContactOutputBuilder::default();
        output = crate::protocol_serde::shape_get_alternate_contact::de_get_alternate_contact(_response_body, output)
            .map_err(crate::operation::get_alternate_contact::GetAlternateContactError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_get_alternate_contact_input(
    input: &crate::operation::get_alternate_contact::GetAlternateContactInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_alternate_contact_input::ser_get_alternate_contact_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_get_alternate_contact(
    value: &[u8],
    mut builder: crate::operation::get_alternate_contact::builders::GetAlternateContactOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_alternate_contact::builders::GetAlternateContactOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "AlternateContact" => {
                    builder = builder.set_alternate_contact(crate::protocol_serde::shape_alternate_contact::de_alternate_contact(tokens)?);
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
