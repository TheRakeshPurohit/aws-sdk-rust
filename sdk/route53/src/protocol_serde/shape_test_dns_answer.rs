// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_test_dns_answer_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::test_dns_answer::TestDnsAnswerOutput, crate::operation::test_dns_answer::TestDNSAnswerError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::test_dns_answer::TestDNSAnswerError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::test_dns_answer::TestDNSAnswerError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidInput" => crate::operation::test_dns_answer::TestDNSAnswerError::InvalidInput({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidInputBuilder::default();
                output = crate::protocol_serde::shape_invalid_input::de_invalid_input_xml_err(_response_body, output)
                    .map_err(crate::operation::test_dns_answer::TestDNSAnswerError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NoSuchHostedZone" => crate::operation::test_dns_answer::TestDNSAnswerError::NoSuchHostedZone({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NoSuchHostedZoneBuilder::default();
                output = crate::protocol_serde::shape_no_such_hosted_zone::de_no_such_hosted_zone_xml_err(_response_body, output)
                    .map_err(crate::operation::test_dns_answer::TestDNSAnswerError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::test_dns_answer::TestDNSAnswerError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_test_dns_answer_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::test_dns_answer::TestDnsAnswerOutput, crate::operation::test_dns_answer::TestDNSAnswerError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::test_dns_answer::builders::TestDnsAnswerOutputBuilder::default();
        output = crate::protocol_serde::shape_test_dns_answer::de_test_dns_answer(_response_body, output)
            .map_err(crate::operation::test_dns_answer::TestDNSAnswerError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::test_dns_answer_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::test_dns_answer::TestDNSAnswerError::unhandled)?
    })
}

#[allow(unused_mut)]
pub fn de_test_dns_answer(
    inp: &[u8],
    mut builder: crate::operation::test_dns_answer::builders::TestDnsAnswerOutputBuilder,
) -> std::result::Result<crate::operation::test_dns_answer::builders::TestDnsAnswerOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("TestDNSAnswerResponse") {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "encountered invalid XML root: expected TestDNSAnswerResponse but got {:?}. This is likely a bug in the SDK.",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("RecordName") /* RecordName com.amazonaws.route53.synthetic#TestDNSAnswerOutput$RecordName */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_record_name(var_1);
            }
            ,
            s if s.matches("ResponseCode") /* ResponseCode com.amazonaws.route53.synthetic#TestDNSAnswerOutput$ResponseCode */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_response_code(var_2);
            }
            ,
            s if s.matches("RecordData") /* RecordData com.amazonaws.route53.synthetic#TestDNSAnswerOutput$RecordData */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_record_data::de_record_data(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_record_data(var_3);
            }
            ,
            s if s.matches("RecordType") /* RecordType com.amazonaws.route53.synthetic#TestDNSAnswerOutput$RecordType */ =>  {
                let var_4 =
                    Some(
                        Result::<crate::types::RrType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::RrType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_record_type(var_4);
            }
            ,
            s if s.matches("Nameserver") /* Nameserver com.amazonaws.route53.synthetic#TestDNSAnswerOutput$Nameserver */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_nameserver(var_5);
            }
            ,
            s if s.matches("Protocol") /* Protocol com.amazonaws.route53.synthetic#TestDNSAnswerOutput$Protocol */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_protocol(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
