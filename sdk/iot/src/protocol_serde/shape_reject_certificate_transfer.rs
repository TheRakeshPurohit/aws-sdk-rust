// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_reject_certificate_transfer_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::reject_certificate_transfer::RejectCertificateTransferOutput,
    crate::operation::reject_certificate_transfer::RejectCertificateTransferError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::reject_certificate_transfer::RejectCertificateTransferError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::reject_certificate_transfer::RejectCertificateTransferError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalFailureException" => crate::operation::reject_certificate_transfer::RejectCertificateTransferError::InternalFailureException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalFailureExceptionBuilder::default();
                output = crate::protocol_serde::shape_internal_failure_exception::de_internal_failure_exception_json_err(_response_body, output)
                    .map_err(crate::operation::reject_certificate_transfer::RejectCertificateTransferError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidRequestException" => crate::operation::reject_certificate_transfer::RejectCertificateTransferError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::reject_certificate_transfer::RejectCertificateTransferError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceNotFoundException" => crate::operation::reject_certificate_transfer::RejectCertificateTransferError::ResourceNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_resource_not_found_exception::de_resource_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::reject_certificate_transfer::RejectCertificateTransferError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServiceUnavailableException" => {
            crate::operation::reject_certificate_transfer::RejectCertificateTransferError::ServiceUnavailableException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServiceUnavailableExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_service_unavailable_exception::de_service_unavailable_exception_json_err(_response_body, output)
                            .map_err(crate::operation::reject_certificate_transfer::RejectCertificateTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ThrottlingException" => crate::operation::reject_certificate_transfer::RejectCertificateTransferError::ThrottlingException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ThrottlingExceptionBuilder::default();
                output = crate::protocol_serde::shape_throttling_exception::de_throttling_exception_json_err(_response_body, output)
                    .map_err(crate::operation::reject_certificate_transfer::RejectCertificateTransferError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TransferAlreadyCompletedException" => {
            crate::operation::reject_certificate_transfer::RejectCertificateTransferError::TransferAlreadyCompletedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::TransferAlreadyCompletedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_transfer_already_completed_exception::de_transfer_already_completed_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::reject_certificate_transfer::RejectCertificateTransferError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UnauthorizedException" => crate::operation::reject_certificate_transfer::RejectCertificateTransferError::UnauthorizedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnauthorizedExceptionBuilder::default();
                output = crate::protocol_serde::shape_unauthorized_exception::de_unauthorized_exception_json_err(_response_body, output)
                    .map_err(crate::operation::reject_certificate_transfer::RejectCertificateTransferError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::reject_certificate_transfer::RejectCertificateTransferError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_reject_certificate_transfer_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::reject_certificate_transfer::RejectCertificateTransferOutput,
    crate::operation::reject_certificate_transfer::RejectCertificateTransferError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::reject_certificate_transfer::builders::RejectCertificateTransferOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_reject_certificate_transfer_input(
    input: &crate::operation::reject_certificate_transfer::RejectCertificateTransferInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_reject_certificate_transfer_input::ser_reject_certificate_transfer_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
