// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_activity_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::describe_activity::DescribeActivityOutput, crate::operation::describe_activity::DescribeActivityError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::describe_activity::DescribeActivityError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::describe_activity::DescribeActivityError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ActivityDoesNotExist" => crate::operation::describe_activity::DescribeActivityError::ActivityDoesNotExist({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ActivityDoesNotExistBuilder::default();
                output = crate::protocol_serde::shape_activity_does_not_exist::de_activity_does_not_exist_json_err(_response_body, output)
                    .map_err(crate::operation::describe_activity::DescribeActivityError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidArn" => crate::operation::describe_activity::DescribeActivityError::InvalidArn({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidArnBuilder::default();
                output = crate::protocol_serde::shape_invalid_arn::de_invalid_arn_json_err(_response_body, output)
                    .map_err(crate::operation::describe_activity::DescribeActivityError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::describe_activity::DescribeActivityError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_describe_activity_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::describe_activity::DescribeActivityOutput, crate::operation::describe_activity::DescribeActivityError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::describe_activity::builders::DescribeActivityOutputBuilder::default();
        output = crate::protocol_serde::shape_describe_activity::de_describe_activity(_response_body, output)
            .map_err(crate::operation::describe_activity::DescribeActivityError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::describe_activity_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::describe_activity::DescribeActivityError::unhandled)?
    })
}

pub fn ser_describe_activity_input(
    input: &crate::operation::describe_activity::DescribeActivityInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_describe_activity_input::ser_describe_activity_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_describe_activity(
    value: &[u8],
    mut builder: crate::operation::describe_activity::builders::DescribeActivityOutputBuilder,
) -> ::std::result::Result<
    crate::operation::describe_activity::builders::DescribeActivityOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "activityArn" => {
                    builder = builder.set_activity_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "name" => {
                    builder = builder.set_name(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "creationDate" => {
                    builder = builder.set_creation_date(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                        tokens.next(),
                        ::aws_smithy_types::date_time::Format::EpochSeconds,
                    )?);
                }
                "encryptionConfiguration" => {
                    builder = builder.set_encryption_configuration(
                        crate::protocol_serde::shape_encryption_configuration::de_encryption_configuration(tokens)?,
                    );
                }
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
