// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_revoke_security_group_egress_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressOutput,
    crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError::generic(
        generic,
    ))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_revoke_security_group_egress_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressOutput,
    crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressOutputBuilder::default();
        output = crate::protocol_serde::shape_revoke_security_group_egress::de_revoke_security_group_egress(_response_body, output)
            .map_err(crate::operation::revoke_security_group_egress::RevokeSecurityGroupEgressError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_revoke_security_group_egress(
    inp: &[u8],
    mut builder: crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressOutputBuilder,
) -> std::result::Result<
    crate::operation::revoke_security_group_egress::builders::RevokeSecurityGroupEgressOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("RevokeSecurityGroupEgressResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected RevokeSecurityGroupEgressResponse got {:?}",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("return") /* Return com.amazonaws.ec2.synthetic#RevokeSecurityGroupEgressOutput$Return */ =>  {
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
            s if s.matches("unknownIpPermissionSet") /* UnknownIpPermissions com.amazonaws.ec2.synthetic#RevokeSecurityGroupEgressOutput$UnknownIpPermissions */ =>  {
                let var_2 =
                    Some(
                        crate::protocol_serde::shape_ip_permission_list::de_ip_permission_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_unknown_ip_permissions(var_2);
            }
            ,
            s if s.matches("revokedSecurityGroupRuleSet") /* RevokedSecurityGroupRules com.amazonaws.ec2.synthetic#RevokeSecurityGroupEgressOutput$RevokedSecurityGroupRules */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_revoked_security_group_rule_list::de_revoked_security_group_rule_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_revoked_security_group_rules(var_3);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
