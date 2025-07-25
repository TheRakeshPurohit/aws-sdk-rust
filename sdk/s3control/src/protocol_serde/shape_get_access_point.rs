// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_access_point_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_access_point::GetAccessPointOutput, crate::operation::get_access_point::GetAccessPointError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_access_point::GetAccessPointError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    Err(crate::operation::get_access_point::GetAccessPointError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_access_point_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_access_point::GetAccessPointOutput, crate::operation::get_access_point::GetAccessPointError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_access_point::builders::GetAccessPointOutputBuilder::default();
        output = crate::protocol_serde::shape_get_access_point::de_get_access_point(_response_body, output)
            .map_err(crate::operation::get_access_point::GetAccessPointError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        output.build()
    })
}

pub fn ser_get_access_point_headers(
    input: &crate::operation::get_access_point::GetAccessPointInput,
    mut builder: ::http::request::Builder,
) -> std::result::Result<::http::request::Builder, ::aws_smithy_types::error::operation::BuildError> {
    if let ::std::option::Option::Some(inner_1) = &input.account_id {
        let formatted_2 = inner_1.as_str();
        let header_value = formatted_2;
        let header_value: ::http::HeaderValue = header_value.parse().map_err(|err| {
            ::aws_smithy_types::error::operation::BuildError::invalid_field(
                "account_id",
                format!("`{}` cannot be used as a header value: {}", &header_value, err),
            )
        })?;
        builder = builder.header("x-amz-account-id", header_value);
    }
    Ok(builder)
}

#[allow(unused_mut)]
pub fn de_get_access_point(
    inp: &[u8],
    mut builder: crate::operation::get_access_point::builders::GetAccessPointOutputBuilder,
) -> std::result::Result<crate::operation::get_access_point::builders::GetAccessPointOutputBuilder, ::aws_smithy_xml::decode::XmlDecodeError> {
    let mut doc = ::aws_smithy_xml::decode::Document::try_from(inp)?;

    #[allow(unused_mut)]
    let mut decoder = doc.root_element()?;
    #[allow(unused_variables)]
    let start_el = decoder.start_el();
    if !start_el.matches("GetAccessPointResult") {
        return Err(::aws_smithy_xml::decode::XmlDecodeError::custom(format!(
            "encountered invalid XML root: expected GetAccessPointResult but got {:?}. This is likely a bug in the SDK.",
            start_el
        )));
    }
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("Endpoints") /* Endpoints com.amazonaws.s3control.synthetic#GetAccessPointOutput$Endpoints */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_endpoints::de_endpoints(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_endpoints(var_3);
            }
            ,
            s if s.matches("PublicAccessBlockConfiguration") /* PublicAccessBlockConfiguration com.amazonaws.s3control.synthetic#GetAccessPointOutput$PublicAccessBlockConfiguration */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_public_access_block_configuration::de_public_access_block_configuration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_public_access_block_configuration(var_4);
            }
            ,
            s if s.matches("CreationDate") /* CreationDate com.amazonaws.s3control.synthetic#GetAccessPointOutput$CreationDate */ =>  {
                let var_5 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.s3control#CreationDate`)"))
                        ?
                    )
                ;
                builder = builder.set_creation_date(var_5);
            }
            ,
            s if s.matches("Bucket") /* Bucket com.amazonaws.s3control.synthetic#GetAccessPointOutput$Bucket */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_bucket(var_6);
            }
            ,
            s if s.matches("Alias") /* Alias com.amazonaws.s3control.synthetic#GetAccessPointOutput$Alias */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_alias(var_7);
            }
            ,
            s if s.matches("AccessPointArn") /* AccessPointArn com.amazonaws.s3control.synthetic#GetAccessPointOutput$AccessPointArn */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_access_point_arn(var_8);
            }
            ,
            s if s.matches("DataSourceType") /* DataSourceType com.amazonaws.s3control.synthetic#GetAccessPointOutput$DataSourceType */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_data_source_type(var_9);
            }
            ,
            s if s.matches("BucketAccountId") /* BucketAccountId com.amazonaws.s3control.synthetic#GetAccessPointOutput$BucketAccountId */ =>  {
                let var_10 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_bucket_account_id(var_10);
            }
            ,
            s if s.matches("VpcConfiguration") /* VpcConfiguration com.amazonaws.s3control.synthetic#GetAccessPointOutput$VpcConfiguration */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_vpc_configuration::de_vpc_configuration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_vpc_configuration(var_11);
            }
            ,
            s if s.matches("NetworkOrigin") /* NetworkOrigin com.amazonaws.s3control.synthetic#GetAccessPointOutput$NetworkOrigin */ =>  {
                let var_12 =
                    Some(
                        Result::<crate::types::NetworkOrigin, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::NetworkOrigin::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_network_origin(var_12);
            }
            ,
            s if s.matches("DataSourceId") /* DataSourceId com.amazonaws.s3control.synthetic#GetAccessPointOutput$DataSourceId */ =>  {
                let var_13 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_data_source_id(var_13);
            }
            ,
            s if s.matches("Name") /* Name com.amazonaws.s3control.synthetic#GetAccessPointOutput$Name */ =>  {
                let var_14 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_14);
            }
            ,
            _ => {}
        }
    }
    Ok(builder)
}
