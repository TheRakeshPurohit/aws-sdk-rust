// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_cache_subnet_group_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupOutput,
    crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => {
            return Err(crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::unhandled(
                generic,
            ))
        }
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "CacheSubnetGroupNotFoundFault" => crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::CacheSubnetGroupNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CacheSubnetGroupNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_cache_subnet_group_not_found_fault::de_cache_subnet_group_not_found_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "CacheSubnetQuotaExceededFault" => crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::CacheSubnetQuotaExceededFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CacheSubnetQuotaExceededFaultBuilder::default();
                output = crate::protocol_serde::shape_cache_subnet_quota_exceeded_fault::de_cache_subnet_quota_exceeded_fault_xml_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidSubnet" => crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::InvalidSubnet({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidSubnetBuilder::default();
                output = crate::protocol_serde::shape_invalid_subnet::de_invalid_subnet_xml_err(_response_body, output)
                    .map_err(crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "SubnetInUse" => crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::SubnetInUse({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::SubnetInUseBuilder::default();
                output = crate::protocol_serde::shape_subnet_in_use::de_subnet_in_use_xml_err(_response_body, output)
                    .map_err(crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "SubnetNotAllowedFault" => crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::SubnetNotAllowedFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::SubnetNotAllowedFaultBuilder::default();
                output = crate::protocol_serde::shape_subnet_not_allowed_fault::de_subnet_not_allowed_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_cache_subnet_group_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupOutput,
    crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_cache_subnet_group::builders::ModifyCacheSubnetGroupOutputBuilder::default();
        output = crate::protocol_serde::shape_modify_cache_subnet_group::de_modify_cache_subnet_group(_response_body, output)
            .map_err(crate::operation::modify_cache_subnet_group::ModifyCacheSubnetGroupError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_cache_subnet_group(
    inp: &[u8],
    mut builder: crate::operation::modify_cache_subnet_group::builders::ModifyCacheSubnetGroupOutputBuilder,
) -> std::result::Result<
    crate::operation::modify_cache_subnet_group::builders::ModifyCacheSubnetGroupOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyCacheSubnetGroupResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyCacheSubnetGroupResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ModifyCacheSubnetGroupResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ModifyCacheSubnetGroupResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("CacheSubnetGroup") /* CacheSubnetGroup com.amazonaws.elasticache.synthetic#ModifyCacheSubnetGroupOutput$CacheSubnetGroup */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_cache_subnet_group::de_cache_subnet_group(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_cache_subnet_group(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected ModifyCacheSubnetGroupResult tag",
        ));
    };
    Ok(builder)
}
