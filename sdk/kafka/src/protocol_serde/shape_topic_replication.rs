// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_topic_replication(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TopicReplication,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.copy_access_control_lists_for_topics {
        object.key("copyAccessControlListsForTopics").boolean(*var_1);
    }
    if let Some(var_2) = &input.copy_topic_configurations {
        object.key("copyTopicConfigurations").boolean(*var_2);
    }
    if let Some(var_3) = &input.detect_and_copy_new_topics {
        object.key("detectAndCopyNewTopics").boolean(*var_3);
    }
    if let Some(var_4) = &input.starting_position {
        #[allow(unused_mut)]
        let mut object_5 = object.key("startingPosition").start_object();
        crate::protocol_serde::shape_replication_starting_position::ser_replication_starting_position(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.topic_name_configuration {
        #[allow(unused_mut)]
        let mut object_7 = object.key("topicNameConfiguration").start_object();
        crate::protocol_serde::shape_replication_topic_name_configuration::ser_replication_topic_name_configuration(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.topics_to_exclude {
        let mut array_9 = object.key("topicsToExclude").start_array();
        for item_10 in var_8 {
            {
                array_9.value().string(item_10.as_str());
            }
        }
        array_9.finish();
    }
    if let Some(var_11) = &input.topics_to_replicate {
        let mut array_12 = object.key("topicsToReplicate").start_array();
        for item_13 in var_11 {
            {
                array_12.value().string(item_13.as_str());
            }
        }
        array_12.finish();
    }
    Ok(())
}

pub(crate) fn de_topic_replication<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::TopicReplication>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::TopicReplicationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "copyAccessControlListsForTopics" => {
                            builder = builder
                                .set_copy_access_control_lists_for_topics(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "copyTopicConfigurations" => {
                            builder =
                                builder.set_copy_topic_configurations(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "detectAndCopyNewTopics" => {
                            builder =
                                builder.set_detect_and_copy_new_topics(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "startingPosition" => {
                            builder = builder.set_starting_position(
                                crate::protocol_serde::shape_replication_starting_position::de_replication_starting_position(tokens)?,
                            );
                        }
                        "topicNameConfiguration" => {
                            builder = builder.set_topic_name_configuration(
                                crate::protocol_serde::shape_replication_topic_name_configuration::de_replication_topic_name_configuration(tokens)?,
                            );
                        }
                        "topicsToExclude" => {
                            builder =
                                builder.set_topics_to_exclude(crate::protocol_serde::shape_list_of_string_max249::de_list_of_string_max249(tokens)?);
                        }
                        "topicsToReplicate" => {
                            builder = builder
                                .set_topics_to_replicate(crate::protocol_serde::shape_list_of_string_max249::de_list_of_string_max249(tokens)?);
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
            Ok(Some(crate::serde_util::topic_replication_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
