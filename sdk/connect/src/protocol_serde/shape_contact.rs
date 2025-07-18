// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_contact<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::Contact>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ContactBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Arn" => {
                            builder = builder.set_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Id" => {
                            builder = builder.set_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "InitialContactId" => {
                            builder = builder.set_initial_contact_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "PreviousContactId" => {
                            builder = builder.set_previous_contact_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ContactAssociationId" => {
                            builder = builder.set_contact_association_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "InitiationMethod" => {
                            builder = builder.set_initiation_method(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ContactInitiationMethod::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "Name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Description" => {
                            builder = builder.set_description(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Channel" => {
                            builder = builder.set_channel(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::Channel::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "QueueInfo" => {
                            builder = builder.set_queue_info(crate::protocol_serde::shape_queue_info::de_queue_info(tokens)?);
                        }
                        "AgentInfo" => {
                            builder = builder.set_agent_info(crate::protocol_serde::shape_agent_info::de_agent_info(tokens)?);
                        }
                        "InitiationTimestamp" => {
                            builder = builder.set_initiation_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "DisconnectTimestamp" => {
                            builder = builder.set_disconnect_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "LastUpdateTimestamp" => {
                            builder = builder.set_last_update_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "LastPausedTimestamp" => {
                            builder = builder.set_last_paused_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "LastResumedTimestamp" => {
                            builder = builder.set_last_resumed_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "TotalPauseCount" => {
                            builder = builder.set_total_pause_count(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "TotalPauseDurationInSeconds" => {
                            builder = builder.set_total_pause_duration_in_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "ScheduledTimestamp" => {
                            builder = builder.set_scheduled_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "RelatedContactId" => {
                            builder = builder.set_related_contact_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "WisdomInfo" => {
                            builder = builder.set_wisdom_info(crate::protocol_serde::shape_wisdom_info::de_wisdom_info(tokens)?);
                        }
                        "CustomerId" => {
                            builder = builder.set_customer_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "CustomerEndpoint" => {
                            builder = builder.set_customer_endpoint(crate::protocol_serde::shape_endpoint_info::de_endpoint_info(tokens)?);
                        }
                        "SystemEndpoint" => {
                            builder = builder.set_system_endpoint(crate::protocol_serde::shape_endpoint_info::de_endpoint_info(tokens)?);
                        }
                        "QueueTimeAdjustmentSeconds" => {
                            builder = builder.set_queue_time_adjustment_seconds(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "QueuePriority" => {
                            builder = builder.set_queue_priority(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i64::try_from)
                                    .transpose()?,
                            );
                        }
                        "Tags" => {
                            builder = builder.set_tags(crate::protocol_serde::shape_contact_tag_map::de_contact_tag_map(tokens)?);
                        }
                        "ConnectedToSystemTimestamp" => {
                            builder = builder.set_connected_to_system_timestamp(::aws_smithy_json::deserialize::token::expect_timestamp_or_null(
                                tokens.next(),
                                ::aws_smithy_types::date_time::Format::EpochSeconds,
                            )?);
                        }
                        "RoutingCriteria" => {
                            builder = builder.set_routing_criteria(crate::protocol_serde::shape_routing_criteria::de_routing_criteria(tokens)?);
                        }
                        "Customer" => {
                            builder = builder.set_customer(crate::protocol_serde::shape_customer::de_customer(tokens)?);
                        }
                        "Campaign" => {
                            builder = builder.set_campaign(crate::protocol_serde::shape_campaign::de_campaign(tokens)?);
                        }
                        "AnsweringMachineDetectionStatus" => {
                            builder = builder.set_answering_machine_detection_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::AnsweringMachineDetectionStatus::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "CustomerVoiceActivity" => {
                            builder = builder.set_customer_voice_activity(
                                crate::protocol_serde::shape_customer_voice_activity::de_customer_voice_activity(tokens)?,
                            );
                        }
                        "QualityMetrics" => {
                            builder = builder.set_quality_metrics(crate::protocol_serde::shape_quality_metrics::de_quality_metrics(tokens)?);
                        }
                        "ChatMetrics" => {
                            builder = builder.set_chat_metrics(crate::protocol_serde::shape_chat_metrics::de_chat_metrics(tokens)?);
                        }
                        "DisconnectDetails" => {
                            builder = builder.set_disconnect_details(crate::protocol_serde::shape_disconnect_details::de_disconnect_details(tokens)?);
                        }
                        "AdditionalEmailRecipients" => {
                            builder = builder.set_additional_email_recipients(
                                crate::protocol_serde::shape_additional_email_recipients::de_additional_email_recipients(tokens)?,
                            );
                        }
                        "SegmentAttributes" => {
                            builder = builder.set_segment_attributes(crate::protocol_serde::shape_segment_attributes::de_segment_attributes(tokens)?);
                        }
                        "Recordings" => {
                            builder = builder.set_recordings(crate::protocol_serde::shape_recordings::de_recordings(tokens)?);
                        }
                        "DisconnectReason" => {
                            builder = builder.set_disconnect_reason(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ContactEvaluations" => {
                            builder =
                                builder.set_contact_evaluations(crate::protocol_serde::shape_contact_evaluations::de_contact_evaluations(tokens)?);
                        }
                        "ContactDetails" => {
                            builder = builder.set_contact_details(crate::protocol_serde::shape_contact_details::de_contact_details(tokens)?);
                        }
                        "Attributes" => {
                            builder = builder.set_attributes(crate::protocol_serde::shape_attributes::de_attributes(tokens)?);
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
