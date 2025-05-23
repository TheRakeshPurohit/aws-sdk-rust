// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_distribution_tenant_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_distribution_tenant::GetDistributionTenantOutput,
    crate::operation::get_distribution_tenant::GetDistributionTenantError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_distribution_tenant::GetDistributionTenantError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_distribution_tenant::GetDistributionTenantError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDenied" => crate::operation::get_distribution_tenant::GetDistributionTenantError::AccessDenied({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::AccessDeniedBuilder::default();
                output = crate::protocol_serde::shape_access_denied::de_access_denied_xml_err(_response_body, output)
                    .map_err(crate::operation::get_distribution_tenant::GetDistributionTenantError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EntityNotFound" => crate::operation::get_distribution_tenant::GetDistributionTenantError::EntityNotFound({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EntityNotFoundBuilder::default();
                output = crate::protocol_serde::shape_entity_not_found::de_entity_not_found_xml_err(_response_body, output)
                    .map_err(crate::operation::get_distribution_tenant::GetDistributionTenantError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_distribution_tenant::GetDistributionTenantError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_distribution_tenant_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_distribution_tenant::GetDistributionTenantOutput,
    crate::operation::get_distribution_tenant::GetDistributionTenantError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_distribution_tenant::builders::GetDistributionTenantOutputBuilder::default();
        output = output
            .set_distribution_tenant(crate::protocol_serde::shape_get_distribution_tenant_output::de_distribution_tenant_payload(_response_body)?);
        output = output.set_e_tag(
            crate::protocol_serde::shape_get_distribution_tenant_output::de_e_tag_header(_response_headers).map_err(|_| {
                crate::operation::get_distribution_tenant::GetDistributionTenantError::unhandled("Failed to parse ETag from header `ETag")
            })?,
        );
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}
