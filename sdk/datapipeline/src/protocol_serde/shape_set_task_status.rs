// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_set_task_status_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::set_task_status::SetTaskStatusOutput, crate::operation::set_task_status::SetTaskStatusError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::set_task_status::SetTaskStatusError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::set_task_status::SetTaskStatusError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InternalServiceError" => crate::operation::set_task_status::SetTaskStatusError::InternalServiceError({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InternalServiceErrorBuilder::default();
                output = crate::protocol_serde::shape_internal_service_error::de_internal_service_error_json_err(_response_body, output)
                    .map_err(crate::operation::set_task_status::SetTaskStatusError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidRequestException" => crate::operation::set_task_status::SetTaskStatusError::InvalidRequestException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRequestExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_request_exception::de_invalid_request_exception_json_err(_response_body, output)
                    .map_err(crate::operation::set_task_status::SetTaskStatusError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "PipelineDeletedException" => crate::operation::set_task_status::SetTaskStatusError::PipelineDeletedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::PipelineDeletedExceptionBuilder::default();
                output = crate::protocol_serde::shape_pipeline_deleted_exception::de_pipeline_deleted_exception_json_err(_response_body, output)
                    .map_err(crate::operation::set_task_status::SetTaskStatusError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "PipelineNotFoundException" => crate::operation::set_task_status::SetTaskStatusError::PipelineNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::PipelineNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_pipeline_not_found_exception::de_pipeline_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::set_task_status::SetTaskStatusError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "TaskNotFoundException" => crate::operation::set_task_status::SetTaskStatusError::TaskNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::TaskNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_task_not_found_exception::de_task_not_found_exception_json_err(_response_body, output)
                    .map_err(crate::operation::set_task_status::SetTaskStatusError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::set_task_status::SetTaskStatusError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_set_task_status_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::set_task_status::SetTaskStatusOutput, crate::operation::set_task_status::SetTaskStatusError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::set_task_status::builders::SetTaskStatusOutputBuilder::default();
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_set_task_status_input(
    input: &crate::operation::set_task_status::SetTaskStatusInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_set_task_status_input::ser_set_task_status_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
