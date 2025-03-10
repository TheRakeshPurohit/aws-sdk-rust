// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_add_application_input_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::add_application_input::AddApplicationInputOutput,
    crate::operation::add_application_input::AddApplicationInputError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::add_application_input::AddApplicationInputError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::add_application_input::AddApplicationInputError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "CodeValidationException" => crate::operation::add_application_input::AddApplicationInputError::CodeValidationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CodeValidationExceptionBuilder::default();
                output = crate::protocol_serde::shape_code_validation_exception::de_code_validation_exception_json_err(_response_body, output)
                    .map_err(crate::operation::add_application_input::AddApplicationInputError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ConcurrentModificationException" => crate::operation::add_application_input::AddApplicationInputError::ConcurrentModificationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ConcurrentModificationExceptionBuilder::default();
                output = crate::protocol_serde::shape_concurrent_modification_exception::de_concurrent_modification_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::add_application_input::AddApplicationInputError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidArgumentException" => crate::operation::add_application_input::AddApplicationInputError::InvalidArgumentException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidArgumentExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_argument_exception::de_invalid_argument_exception_json_err(_response_body, output)
                    .map_err(crate::operation::add_application_input::AddApplicationInputError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceInUseException" => crate::operation::add_application_input::AddApplicationInputError::ResourceInUseException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceInUseExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_in_use_exception::de_resource_in_use_exception_json_err(_response_body, output)
                    .map_err(crate::operation::add_application_input::AddApplicationInputError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::add_application_input::AddApplicationInputError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::add_application_input::AddApplicationInputError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnsupportedOperationException" => crate::operation::add_application_input::AddApplicationInputError::UnsupportedOperationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnsupportedOperationExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_unsupported_operation_exception::de_unsupported_operation_exception_json_err(_response_body, output)
                        .map_err(crate::operation::add_application_input::AddApplicationInputError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::add_application_input::AddApplicationInputError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_add_application_input_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::add_application_input::AddApplicationInputOutput,
    crate::operation::add_application_input::AddApplicationInputError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::add_application_input::builders::AddApplicationInputOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_add_application_input_input(
    input: &crate::operation::add_application_input::AddApplicationInputInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_add_application_input_input::ser_add_application_input_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
