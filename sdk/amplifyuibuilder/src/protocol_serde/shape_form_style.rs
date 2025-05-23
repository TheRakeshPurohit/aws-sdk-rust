// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_form_style(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::FormStyle,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.horizontal_gap {
        #[allow(unused_mut)]
        let mut object_2 = object.key("horizontalGap").start_object();
        crate::protocol_serde::shape_form_style_config::ser_form_style_config(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.vertical_gap {
        #[allow(unused_mut)]
        let mut object_4 = object.key("verticalGap").start_object();
        crate::protocol_serde::shape_form_style_config::ser_form_style_config(&mut object_4, var_3)?;
        object_4.finish();
    }
    if let Some(var_5) = &input.outer_padding {
        #[allow(unused_mut)]
        let mut object_6 = object.key("outerPadding").start_object();
        crate::protocol_serde::shape_form_style_config::ser_form_style_config(&mut object_6, var_5)?;
        object_6.finish();
    }
    Ok(())
}

pub(crate) fn de_form_style<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::FormStyle>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::FormStyleBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "horizontalGap" => {
                            builder = builder.set_horizontal_gap(crate::protocol_serde::shape_form_style_config::de_form_style_config(tokens)?);
                        }
                        "verticalGap" => {
                            builder = builder.set_vertical_gap(crate::protocol_serde::shape_form_style_config::de_form_style_config(tokens)?);
                        }
                        "outerPadding" => {
                            builder = builder.set_outer_padding(crate::protocol_serde::shape_form_style_config::de_form_style_config(tokens)?);
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
