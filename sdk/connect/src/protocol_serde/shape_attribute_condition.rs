// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_attribute_condition(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AttributeCondition,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.name {
        object.key("Name").string(var_1.as_str());
    }
    if let Some(var_2) = &input.value {
        object.key("Value").string(var_2.as_str());
    }
    if let Some(var_3) = &input.proficiency_level {
        object.key("ProficiencyLevel").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.range {
        #[allow(unused_mut)]
        let mut object_5 = object.key("Range").start_object();
        crate::protocol_serde::shape_range::ser_range(&mut object_5, var_4)?;
        object_5.finish();
    }
    if let Some(var_6) = &input.match_criteria {
        #[allow(unused_mut)]
        let mut object_7 = object.key("MatchCriteria").start_object();
        crate::protocol_serde::shape_match_criteria::ser_match_criteria(&mut object_7, var_6)?;
        object_7.finish();
    }
    if let Some(var_8) = &input.comparison_operator {
        object.key("ComparisonOperator").string(var_8.as_str());
    }
    Ok(())
}

pub(crate) fn de_attribute_condition<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AttributeCondition>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AttributeConditionBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Name" => {
                            builder = builder.set_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Value" => {
                            builder = builder.set_value(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ProficiencyLevel" => {
                            builder = builder.set_proficiency_level(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?.map(|v| v.to_f32_lossy()),
                            );
                        }
                        "Range" => {
                            builder = builder.set_range(crate::protocol_serde::shape_range::de_range(tokens)?);
                        }
                        "MatchCriteria" => {
                            builder = builder.set_match_criteria(crate::protocol_serde::shape_match_criteria::de_match_criteria(tokens)?);
                        }
                        "ComparisonOperator" => {
                            builder = builder.set_comparison_operator(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
