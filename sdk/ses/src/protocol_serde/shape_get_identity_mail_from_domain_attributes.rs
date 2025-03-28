// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_identity_mail_from_domain_attributes_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesOutput,
    crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_identity_mail_from_domain_attributes_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesOutput,
    crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output =
            crate::operation::get_identity_mail_from_domain_attributes::builders::GetIdentityMailFromDomainAttributesOutputBuilder::default();
        output = crate::protocol_serde::shape_get_identity_mail_from_domain_attributes::de_get_identity_mail_from_domain_attributes(
            _response_body,
            output,
        )
        .map_err(crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::get_identity_mail_from_domain_attributes_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::get_identity_mail_from_domain_attributes::GetIdentityMailFromDomainAttributesError::unhandled)?
    })
}

#[allow(unused_mut)]
pub fn de_get_identity_mail_from_domain_attributes(
    inp: &[u8],
    mut builder: crate::operation::get_identity_mail_from_domain_attributes::builders::GetIdentityMailFromDomainAttributesOutputBuilder,
) -> std::result::Result<
    crate::operation::get_identity_mail_from_domain_attributes::builders::GetIdentityMailFromDomainAttributesOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("GetIdentityMailFromDomainAttributesResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected GetIdentityMailFromDomainAttributesResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("GetIdentityMailFromDomainAttributesResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected GetIdentityMailFromDomainAttributesResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("MailFromDomainAttributes") /* MailFromDomainAttributes com.amazonaws.ses.synthetic#GetIdentityMailFromDomainAttributesOutput$MailFromDomainAttributes */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_mail_from_domain_attributes::de_mail_from_domain_attributes(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_mail_from_domain_attributes(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected GetIdentityMailFromDomainAttributesResult tag",
        ));
    };
    Ok(builder)
}
