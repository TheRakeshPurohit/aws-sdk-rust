// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_topic_category_filter_constant(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::TopicCategoryFilterConstant,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.constant_type {
        object.key("ConstantType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.singular_constant {
        object.key("SingularConstant").string(var_2.as_str());
    }
    if let Some(var_3) = &input.collective_constant {
        #[allow(unused_mut)]
        let mut object_4 = object.key("CollectiveConstant").start_object();
        crate::protocol_serde::shape_collective_constant::ser_collective_constant(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}

pub(crate) fn de_topic_category_filter_constant<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::TopicCategoryFilterConstant>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::TopicCategoryFilterConstantBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "ConstantType" => {
                            builder = builder.set_constant_type(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::ConstantType::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "SingularConstant" => {
                            builder = builder.set_singular_constant(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "CollectiveConstant" => {
                            builder =
                                builder.set_collective_constant(crate::protocol_serde::shape_collective_constant::de_collective_constant(tokens)?);
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
