// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_user_pool_mfa_config_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigOutput,
    crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalErrorException" => crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::InternalErrorException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalErrorExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_error_exception::de_internal_error_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterException" => crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NotAuthorizedException" => crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::NotAuthorizedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NotAuthorizedExceptionBuilder::default();
                output = crate::protocol_serde::shape_not_authorized_exception::de_not_authorized_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TooManyRequestsException" => crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::TooManyRequestsException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TooManyRequestsExceptionBuilder::default();
                output = crate::protocol_serde::shape_too_many_requests_exception::de_too_many_requests_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_user_pool_mfa_config_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigOutput,
    crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_user_pool_mfa_config::builders::GetUserPoolMfaConfigOutputBuilder::default();
        output = crate::protocol_serde::shape_get_user_pool_mfa_config::de_get_user_pool_mfa_config(_response_body, output)
            .map_err(crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_get_user_pool_mfa_config_input(
    input: &crate::operation::get_user_pool_mfa_config::GetUserPoolMfaConfigInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_user_pool_mfa_config_input::ser_get_user_pool_mfa_config_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_get_user_pool_mfa_config(
    value: &[u8],
    mut builder: crate::operation::get_user_pool_mfa_config::builders::GetUserPoolMfaConfigOutputBuilder,
) -> ::std::result::Result<
    crate::operation::get_user_pool_mfa_config::builders::GetUserPoolMfaConfigOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "SmsMfaConfiguration" => {
                    builder = builder.set_sms_mfa_configuration(crate::protocol_serde::shape_sms_mfa_config_type::de_sms_mfa_config_type(tokens)?);
                }
                "SoftwareTokenMfaConfiguration" => {
                    builder = builder.set_software_token_mfa_configuration(
                        crate::protocol_serde::shape_software_token_mfa_config_type::de_software_token_mfa_config_type(tokens)?,
                    );
                }
                "EmailMfaConfiguration" => {
                    builder =
                        builder.set_email_mfa_configuration(crate::protocol_serde::shape_email_mfa_config_type::de_email_mfa_config_type(tokens)?);
                }
                "MfaConfiguration" => {
                    builder = builder.set_mfa_configuration(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::UserPoolMfaType::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "WebAuthnConfiguration" => {
                    builder = builder.set_web_authn_configuration(
                        crate::protocol_serde::shape_web_authn_configuration_type::de_web_authn_configuration_type(tokens)?,
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
