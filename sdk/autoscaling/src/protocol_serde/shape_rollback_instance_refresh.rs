// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_rollback_instance_refresh_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::rollback_instance_refresh::RollbackInstanceRefreshOutput,
    crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ActiveInstanceRefreshNotFound" => {
            crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError::ActiveInstanceRefreshNotFoundFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ActiveInstanceRefreshNotFoundFaultBuilder::default();
                    output =
                        crate::protocol_serde::shape_active_instance_refresh_not_found_fault::de_active_instance_refresh_not_found_fault_xml_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "IrreversibleInstanceRefresh" => {
            crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError::IrreversibleInstanceRefreshFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::IrreversibleInstanceRefreshFaultBuilder::default();
                    output = crate::protocol_serde::shape_irreversible_instance_refresh_fault::de_irreversible_instance_refresh_fault_xml_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "LimitExceeded" => crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError::LimitExceededFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::LimitExceededFaultBuilder::default();
                output = crate::protocol_serde::shape_limit_exceeded_fault::de_limit_exceeded_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ResourceContention" => crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError::ResourceContentionFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ResourceContentionFaultBuilder::default();
                output = crate::protocol_serde::shape_resource_contention_fault::de_resource_contention_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_rollback_instance_refresh_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::rollback_instance_refresh::RollbackInstanceRefreshOutput,
    crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::rollback_instance_refresh::builders::RollbackInstanceRefreshOutputBuilder::default();
        output = crate::protocol_serde::shape_rollback_instance_refresh::de_rollback_instance_refresh(_response_body, output)
            .map_err(crate::operation::rollback_instance_refresh::RollbackInstanceRefreshError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_rollback_instance_refresh(
    inp: &[u8],
    mut builder: crate::operation::rollback_instance_refresh::builders::RollbackInstanceRefreshOutputBuilder,
) -> std::result::Result<
    crate::operation::rollback_instance_refresh::builders::RollbackInstanceRefreshOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("RollbackInstanceRefreshResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected RollbackInstanceRefreshResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("RollbackInstanceRefreshResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected RollbackInstanceRefreshResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("InstanceRefreshId") /* InstanceRefreshId com.amazonaws.autoscaling.synthetic#RollbackInstanceRefreshOutput$InstanceRefreshId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_instance_refresh_id(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected RollbackInstanceRefreshResult tag",
        ));
    };
    Ok(builder)
}
