// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_update_replication_job_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_replication_job::UpdateReplicationJobOutput,
    crate::operation::update_replication_job::UpdateReplicationJobError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::update_replication_job::UpdateReplicationJobError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::update_replication_job::UpdateReplicationJobError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalError" => crate::operation::update_replication_job::UpdateReplicationJobError::InternalError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalErrorBuilder::default();
                output = crate::protocol_serde::shape_internal_error::de_internal_error_json_err(_response_body, output)
                    .map_err(crate::operation::update_replication_job::UpdateReplicationJobError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParameterException" => crate::operation::update_replication_job::UpdateReplicationJobError::InvalidParameterException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParameterExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parameter_exception::de_invalid_parameter_exception_json_err(_response_body, output)
                    .map_err(crate::operation::update_replication_job::UpdateReplicationJobError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "MissingRequiredParameterException" => {
            crate::operation::update_replication_job::UpdateReplicationJobError::MissingRequiredParameterException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::MissingRequiredParameterExceptionBuilder::default();
                    output = crate::protocol_serde::shape_missing_required_parameter_exception::de_missing_required_parameter_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::update_replication_job::UpdateReplicationJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "OperationNotPermittedException" => crate::operation::update_replication_job::UpdateReplicationJobError::OperationNotPermittedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::OperationNotPermittedExceptionBuilder::default();
                output = crate::protocol_serde::shape_operation_not_permitted_exception::de_operation_not_permitted_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::update_replication_job::UpdateReplicationJobError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ReplicationJobNotFoundException" => crate::operation::update_replication_job::UpdateReplicationJobError::ReplicationJobNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ReplicationJobNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_replication_job_not_found_exception::de_replication_job_not_found_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::update_replication_job::UpdateReplicationJobError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ServerCannotBeReplicatedException" => {
            crate::operation::update_replication_job::UpdateReplicationJobError::ServerCannotBeReplicatedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ServerCannotBeReplicatedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_server_cannot_be_replicated_exception::de_server_cannot_be_replicated_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::update_replication_job::UpdateReplicationJobError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "TemporarilyUnavailableException" => crate::operation::update_replication_job::UpdateReplicationJobError::TemporarilyUnavailableException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TemporarilyUnavailableExceptionBuilder::default();
                output = crate::protocol_serde::shape_temporarily_unavailable_exception::de_temporarily_unavailable_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::update_replication_job::UpdateReplicationJobError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnauthorizedOperationException" => crate::operation::update_replication_job::UpdateReplicationJobError::UnauthorizedOperationException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnauthorizedOperationExceptionBuilder::default();
                output = crate::protocol_serde::shape_unauthorized_operation_exception::de_unauthorized_operation_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::update_replication_job::UpdateReplicationJobError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::update_replication_job::UpdateReplicationJobError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_update_replication_job_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::update_replication_job::UpdateReplicationJobOutput,
    crate::operation::update_replication_job::UpdateReplicationJobError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::update_replication_job::builders::UpdateReplicationJobOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_update_replication_job_input(
    input: &crate::operation::update_replication_job::UpdateReplicationJobInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_update_replication_job_input::ser_update_replication_job_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
