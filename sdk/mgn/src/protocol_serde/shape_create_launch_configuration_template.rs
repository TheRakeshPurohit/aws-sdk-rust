// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_create_launch_configuration_template_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateOutput,
    crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateError,
> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "AccessDeniedException" => {
            crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateError::AccessDeniedException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::AccessDeniedExceptionBuilder::default();
                    output = crate::protocol_serde::shape_access_denied_exception::de_access_denied_exception_json_err(_response_body, output)
                        .map_err(crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "UninitializedAccountException" => {
            crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateError::UninitializedAccountException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::UninitializedAccountExceptionBuilder::default();
                    output = crate::protocol_serde::shape_uninitialized_account_exception::de_uninitialized_account_exception_json_err(
                        _response_body,
                        output,
                    )
                    .map_err(crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        "ValidationException" => {
            crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateError::ValidationException({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::types::error::builders::ValidationExceptionBuilder::default();
                    output = crate::protocol_serde::shape_validation_exception::de_validation_exception_json_err(_response_body, output)
                        .map_err(crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateError::unhandled)?;
                    let output = output.meta(generic);
                    output.build()
                };
                if tmp.message.is_none() {
                    tmp.message = _error_message;
                }
                tmp
            })
        }
        _ => crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_create_launch_configuration_template_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<
    crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateOutput,
    crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::create_launch_configuration_template::builders::CreateLaunchConfigurationTemplateOutputBuilder::default();
        output = crate::protocol_serde::shape_create_launch_configuration_template::de_create_launch_configuration_template(_response_body, output)
            .map_err(crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::create_launch_configuration_template_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateError::unhandled)?
    })
}

pub fn ser_create_launch_configuration_template_input(
    input: &crate::operation::create_launch_configuration_template::CreateLaunchConfigurationTemplateInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_create_launch_configuration_template_input::ser_create_launch_configuration_template_input_input(
        &mut object,
        input,
    )?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_create_launch_configuration_template(
    value: &[u8],
    mut builder: crate::operation::create_launch_configuration_template::builders::CreateLaunchConfigurationTemplateOutputBuilder,
) -> ::std::result::Result<
    crate::operation::create_launch_configuration_template::builders::CreateLaunchConfigurationTemplateOutputBuilder,
    ::aws_smithy_json::deserialize::error::DeserializeError,
> {
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "arn" => {
                    builder = builder.set_arn(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "associatePublicIpAddress" => {
                    builder = builder.set_associate_public_ip_address(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                }
                "bootMode" => {
                    builder = builder.set_boot_mode(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::BootMode::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "copyPrivateIp" => {
                    builder = builder.set_copy_private_ip(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                }
                "copyTags" => {
                    builder = builder.set_copy_tags(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                }
                "ec2LaunchTemplateID" => {
                    builder = builder.set_ec2_launch_template_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "enableMapAutoTagging" => {
                    builder = builder.set_enable_map_auto_tagging(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                }
                "largeVolumeConf" => {
                    builder = builder.set_large_volume_conf(crate::protocol_serde::shape_launch_template_disk_conf::de_launch_template_disk_conf(
                        tokens,
                    )?);
                }
                "launchConfigurationTemplateID" => {
                    builder = builder.set_launch_configuration_template_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "launchDisposition" => {
                    builder = builder.set_launch_disposition(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| crate::types::LaunchDisposition::from(u.as_ref())))
                            .transpose()?,
                    );
                }
                "licensing" => {
                    builder = builder.set_licensing(crate::protocol_serde::shape_licensing::de_licensing(tokens)?);
                }
                "mapAutoTaggingMpeID" => {
                    builder = builder.set_map_auto_tagging_mpe_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "postLaunchActions" => {
                    builder = builder.set_post_launch_actions(crate::protocol_serde::shape_post_launch_actions::de_post_launch_actions(tokens)?);
                }
                "smallVolumeConf" => {
                    builder = builder.set_small_volume_conf(crate::protocol_serde::shape_launch_template_disk_conf::de_launch_template_disk_conf(
                        tokens,
                    )?);
                }
                "smallVolumeMaxSize" => {
                    builder = builder.set_small_volume_max_size(
                        ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                            .map(i64::try_from)
                            .transpose()?,
                    );
                }
                "tags" => {
                    builder = builder.set_tags(crate::protocol_serde::shape_tags_map::de_tags_map(tokens)?);
                }
                "targetInstanceTypeRightSizingMethod" => {
                    builder = builder.set_target_instance_type_right_sizing_method(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| {
                                s.to_unescaped()
                                    .map(|u| crate::types::TargetInstanceTypeRightSizingMethod::from(u.as_ref()))
                            })
                            .transpose()?,
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
