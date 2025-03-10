// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_contribution_analysis_time_ranges(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ContributionAnalysisTimeRanges,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.start_range {
        #[allow(unused_mut)]
        let mut object_2 = object.key("StartRange").start_object();
        crate::protocol_serde::shape_topic_ir_filter_option::ser_topic_ir_filter_option(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.end_range {
        #[allow(unused_mut)]
        let mut object_4 = object.key("EndRange").start_object();
        crate::protocol_serde::shape_topic_ir_filter_option::ser_topic_ir_filter_option(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_contribution_analysis_time_ranges<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ContributionAnalysisTimeRanges>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ContributionAnalysisTimeRangesBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "StartRange" => {
                            builder =
                                builder.set_start_range(crate::protocol_serde::shape_topic_ir_filter_option::de_topic_ir_filter_option(tokens)?);
                        }
                        "EndRange" => {
                            builder = builder.set_end_range(crate::protocol_serde::shape_topic_ir_filter_option::de_topic_ir_filter_option(tokens)?);
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
