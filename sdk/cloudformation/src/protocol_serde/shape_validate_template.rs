// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_validate_template_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::validate_template::ValidateTemplateOutput, crate::operation::validate_template::ValidateTemplateError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::validate_template::ValidateTemplateError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::validate_template::ValidateTemplateError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_validate_template_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::validate_template::ValidateTemplateOutput, crate::operation::validate_template::ValidateTemplateError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::validate_template::builders::ValidateTemplateOutputBuilder::default();
        output = crate::protocol_serde::shape_validate_template::de_validate_template(_response_body, output)
            .map_err(crate::operation::validate_template::ValidateTemplateError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

#[allow(unused_mut)]
pub fn de_validate_template(
    inp: &[u8],
    mut builder: crate::operation::validate_template::builders::ValidateTemplateOutputBuilder,
) -> std::result::Result<crate::operation::validate_template::builders::ValidateTemplateOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !(start_el.matches("ValidateTemplateResponse")) {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "invalid root, expected ValidateTemplateResponse got {:?}",
            start_el
        )));
    }
    if let Some(mut result_tag) = decoder.next_tag() {
        let start_el = result_tag.start_el();
        if !(start_el.matches("ValidateTemplateResult")) {
            return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
                "invalid result, expected ValidateTemplateResult got {:?}",
                start_el
            )));
        }
        while let Some(mut tag) = result_tag.next_tag() {
            match tag.start_el() {
            s if s.matches("Parameters") /* Parameters com.amazonaws.cloudformation.synthetic#ValidateTemplateOutput$Parameters */ =>  {
                let var_1 =
                    Some(
                        crate::protocol_serde::shape_template_parameters::de_template_parameters(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_parameters(var_1);
            }
            ,
            s if s.matches("Description") /* Description com.amazonaws.cloudformation.synthetic#ValidateTemplateOutput$Description */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_2);
            }
            ,
            s if s.matches("Capabilities") /* Capabilities com.amazonaws.cloudformation.synthetic#ValidateTemplateOutput$Capabilities */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_capabilities::de_capabilities(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_capabilities(var_3);
            }
            ,
            s if s.matches("CapabilitiesReason") /* CapabilitiesReason com.amazonaws.cloudformation.synthetic#ValidateTemplateOutput$CapabilitiesReason */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_capabilities_reason(var_4);
            }
            ,
            s if s.matches("DeclaredTransforms") /* DeclaredTransforms com.amazonaws.cloudformation.synthetic#ValidateTemplateOutput$DeclaredTransforms */ =>  {
                let var_5 =
                    Some(
                        crate::protocol_serde::shape_transforms_list::de_transforms_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_declared_transforms(var_5);
            }
            ,
            _ => {}
        }
        }
    } else {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom("expected ValidateTemplateResult tag"));
    };
    Ok(builder)
}
