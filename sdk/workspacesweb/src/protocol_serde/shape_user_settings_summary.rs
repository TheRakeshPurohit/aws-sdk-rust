// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_user_settings_summary<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::UserSettingsSummary>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::UserSettingsSummaryBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "userSettingsArn" => {
                            builder = builder.set_user_settings_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "copyAllowed" => {
                            builder = builder.set_copy_allowed(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EnabledType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "pasteAllowed" => {
                            builder = builder.set_paste_allowed(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EnabledType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "downloadAllowed" => {
                            builder = builder.set_download_allowed(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EnabledType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "uploadAllowed" => {
                            builder = builder.set_upload_allowed(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EnabledType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "printAllowed" => {
                            builder = builder.set_print_allowed(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EnabledType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "disconnectTimeoutInMinutes" => {
                            builder = builder.set_disconnect_timeout_in_minutes(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "idleDisconnectTimeoutInMinutes" => {
                            builder = builder.set_idle_disconnect_timeout_in_minutes(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "cookieSynchronizationConfiguration" => {
                            builder = builder.set_cookie_synchronization_configuration(
                                crate::protocol_serde::shape_cookie_synchronization_configuration::de_cookie_synchronization_configuration(tokens)?,
                            );
                        }
                        "deepLinkAllowed" => {
                            builder = builder.set_deep_link_allowed(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::EnabledType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "toolbarConfiguration" => {
                            builder = builder
                                .set_toolbar_configuration(crate::protocol_serde::shape_toolbar_configuration::de_toolbar_configuration(tokens)?);
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
            Ok(Some(crate::serde_util::user_settings_summary_correct_errors(builder).build().map_err(
                |err| ::aws_smithy_json::deserialize::error::DeserializeError::custom_source("Response was invalid", err),
            )?))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
