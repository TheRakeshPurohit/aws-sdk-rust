// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_decode_authorization_message_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput,
    crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidAuthorizationMessageException" => {
            crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::InvalidAuthorizationMessageException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::InvalidAuthorizationMessageExceptionBuilder::default();
                    output =
                        crate::protocol_serde::shape_invalid_authorization_message_exception::de_invalid_authorization_message_exception_xml_err(
                            _response_body,
                            output,
                        )
                        .map_err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_decode_authorization_message_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::decode_authorization_message::DecodeAuthorizationMessageOutput,
    crate::operation::decode_authorization_message::DecodeAuthorizationMessageError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder::default();
        output = crate::protocol_serde::shape_decode_authorization_message::de_decode_authorization_message(_response_body, output)
            .map_err(crate::operation::decode_authorization_message::DecodeAuthorizationMessageError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_decode_authorization_message(
    inp: &[u8],
    mut builder: crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder,
) -> std::result::Result<
    crate::operation::decode_authorization_message::builders::DecodeAuthorizationMessageOutputBuilder,
    ::aws_smithy_xml::decode::XmlDecodeError,
> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("DecodeAuthorizationMessageResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected DecodeAuthorizationMessageResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("DecodeAuthorizationMessageResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected DecodeAuthorizationMessageResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("DecodedMessage") /* DecodedMessage com.amazonaws.sts.synthetic#DecodeAuthorizationMessageOutput$DecodedMessage */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_decoded_message(var_1);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(
            "expected DecodeAuthorizationMessageResult tag",
        ));
    };
    Ok(builder)
}
