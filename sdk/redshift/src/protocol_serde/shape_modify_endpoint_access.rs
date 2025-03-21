// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_endpoint_access_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_endpoint_access::ModifyEndpointAccessOutput,
    crate::operation::modify_endpoint_access::ModifyEndpointAccessError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::modify_endpoint_access::ModifyEndpointAccessError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::modify_endpoint_access::ModifyEndpointAccessError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ClusterNotFound" => crate::operation::modify_endpoint_access::ModifyEndpointAccessError::ClusterNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ClusterNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_cluster_not_found_fault::de_cluster_not_found_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::modify_endpoint_access::ModifyEndpointAccessError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EndpointNotFound" => crate::operation::modify_endpoint_access::ModifyEndpointAccessError::EndpointNotFoundFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EndpointNotFoundFaultBuilder::default();
                output = crate::protocol_serde::shape_endpoint_not_found_fault::de_endpoint_not_found_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::modify_endpoint_access::ModifyEndpointAccessError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidClusterSecurityGroupState" => {
            crate::operation::modify_endpoint_access::ModifyEndpointAccessError::InvalidClusterSecurityGroupStateFault({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidClusterSecurityGroupStateFaultBuilder::default();
                    output = crate::protocol_serde::shape_invalid_cluster_security_group_state_fault::de_invalid_cluster_security_group_state_fault_xml_err(_response_body, output).map_err(crate::operation::modify_endpoint_access::ModifyEndpointAccessError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "InvalidClusterState" => crate::operation::modify_endpoint_access::ModifyEndpointAccessError::InvalidClusterStateFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidClusterStateFaultBuilder::default();
                output = crate::protocol_serde::shape_invalid_cluster_state_fault::de_invalid_cluster_state_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::modify_endpoint_access::ModifyEndpointAccessError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidEndpointState" => crate::operation::modify_endpoint_access::ModifyEndpointAccessError::InvalidEndpointStateFault({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidEndpointStateFaultBuilder::default();
                output = crate::protocol_serde::shape_invalid_endpoint_state_fault::de_invalid_endpoint_state_fault_xml_err(_response_body, output)
                    .map_err(crate::operation::modify_endpoint_access::ModifyEndpointAccessError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "UnauthorizedOperation" => crate::operation::modify_endpoint_access::ModifyEndpointAccessError::UnauthorizedOperation({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::UnauthorizedOperationBuilder::default();
                output = crate::protocol_serde::shape_unauthorized_operation::de_unauthorized_operation_xml_err(_response_body, output)
                    .map_err(crate::operation::modify_endpoint_access::ModifyEndpointAccessError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::modify_endpoint_access::ModifyEndpointAccessError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_modify_endpoint_access_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::modify_endpoint_access::ModifyEndpointAccessOutput,
    crate::operation::modify_endpoint_access::ModifyEndpointAccessError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::modify_endpoint_access::builders::ModifyEndpointAccessOutputBuilder::default();
        output = crate::protocol_serde::shape_modify_endpoint_access::de_modify_endpoint_access(_response_body, output)
            .map_err(crate::operation::modify_endpoint_access::ModifyEndpointAccessError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_modify_endpoint_access(
    inp: &[u8],
    mut builder: crate::operation::modify_endpoint_access::builders::ModifyEndpointAccessOutputBuilder,
) -> std::result::Result<
    crate::operation::modify_endpoint_access::builders::ModifyEndpointAccessOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ModifyEndpointAccessResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ModifyEndpointAccessResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ModifyEndpointAccessResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ModifyEndpointAccessResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("ClusterIdentifier") /* ClusterIdentifier com.amazonaws.redshift.synthetic#ModifyEndpointAccessOutput$ClusterIdentifier */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cluster_identifier(var_1);
            }
            ,
            s if s.matches("ResourceOwner") /* ResourceOwner com.amazonaws.redshift.synthetic#ModifyEndpointAccessOutput$ResourceOwner */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_resource_owner(var_2);
            }
            ,
            s if s.matches("SubnetGroupName") /* SubnetGroupName com.amazonaws.redshift.synthetic#ModifyEndpointAccessOutput$SubnetGroupName */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_subnet_group_name(var_3);
            }
            ,
            s if s.matches("EndpointStatus") /* EndpointStatus com.amazonaws.redshift.synthetic#ModifyEndpointAccessOutput$EndpointStatus */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_endpoint_status(var_4);
            }
            ,
            s if s.matches("EndpointName") /* EndpointName com.amazonaws.redshift.synthetic#ModifyEndpointAccessOutput$EndpointName */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_endpoint_name(var_5);
            }
            ,
            s if s.matches("EndpointCreateTime") /* EndpointCreateTime com.amazonaws.redshift.synthetic#ModifyEndpointAccessOutput$EndpointCreateTime */ =>  {
                let var_6 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.redshift#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_endpoint_create_time(var_6);
            }
            ,
            s if s.matches("Port") /* Port com.amazonaws.redshift.synthetic#ModifyEndpointAccessOutput$Port */ =>  {
                let var_7 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.redshift#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_port(var_7);
            }
            ,
            s if s.matches("Address") /* Address com.amazonaws.redshift.synthetic#ModifyEndpointAccessOutput$Address */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_address(var_8);
            }
            ,
            s if s.matches("VpcSecurityGroups") /* VpcSecurityGroups com.amazonaws.redshift.synthetic#ModifyEndpointAccessOutput$VpcSecurityGroups */ =>  {
                let var_9 =
                    Some(
                        crate::protocol_serde::shape_vpc_security_group_membership_list::de_vpc_security_group_membership_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_vpc_security_groups(var_9);
            }
            ,
            s if s.matches("VpcEndpoint") /* VpcEndpoint com.amazonaws.redshift.synthetic#ModifyEndpointAccessOutput$VpcEndpoint */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_vpc_endpoint::de_vpc_endpoint(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_vpc_endpoint(var_10);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected ModifyEndpointAccessResult tag",
        ));
    };
    Ok(builder)
}
