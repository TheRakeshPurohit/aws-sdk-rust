// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_disable_vpc_classic_link_dns_support_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::disable_vpc_classic_link_dns_support::DisableVpcClassicLinkDnsSupportOutput,
    crate::operation::disable_vpc_classic_link_dns_support::DisableVpcClassicLinkDnsSupportError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::disable_vpc_classic_link_dns_support::DisableVpcClassicLinkDnsSupportError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::disable_vpc_classic_link_dns_support::DisableVpcClassicLinkDnsSupportError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_disable_vpc_classic_link_dns_support_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::disable_vpc_classic_link_dns_support::DisableVpcClassicLinkDnsSupportOutput,
    crate::operation::disable_vpc_classic_link_dns_support::DisableVpcClassicLinkDnsSupportError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::disable_vpc_classic_link_dns_support::builders::DisableVpcClassicLinkDnsSupportOutputBuilder::default();
        output = crate::protocol_serde::shape_disable_vpc_classic_link_dns_support::de_disable_vpc_classic_link_dns_support(_response_body, output)
            .map_err(crate::operation::disable_vpc_classic_link_dns_support::DisableVpcClassicLinkDnsSupportError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_disable_vpc_classic_link_dns_support(
    inp: &[u8],
    mut builder: crate::operation::disable_vpc_classic_link_dns_support::builders::DisableVpcClassicLinkDnsSupportOutputBuilder,
) -> std::result::Result<
    crate::operation::disable_vpc_classic_link_dns_support::builders::DisableVpcClassicLinkDnsSupportOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DisableVpcClassicLinkDnsSupportResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DisableVpcClassicLinkDnsSupportResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("return") /* Return com.amazonaws.ec2.synthetic#DisableVpcClassicLinkDnsSupportOutput$Return */ =>  {
                let var_1 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_return(var_1);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
