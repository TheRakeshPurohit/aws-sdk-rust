// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_inline_custom_document_enrichment_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::InlineCustomDocumentEnrichmentConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.condition {
        #[allow(unused_mut)]
        let mut object_2 = object.key("Condition").start_object();
        crate::protocol_serde::shape_document_attribute_condition::ser_document_attribute_condition(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.target {
        #[allow(unused_mut)]
        let mut object_4 = object.key("Target").start_object();
        crate::protocol_serde::shape_document_attribute_target::ser_document_attribute_target(&mut object_4, var_3)?;
        object_4.finish();
    }
    if input.document_content_deletion {
        object.key("DocumentContentDeletion").boolean(input.document_content_deletion);
    }
    Ok(())
}

pub(crate) fn de_inline_custom_document_enrichment_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::InlineCustomDocumentEnrichmentConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::InlineCustomDocumentEnrichmentConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "Condition" => {
                            builder = builder
                                .set_condition(crate::protocol_serde::shape_document_attribute_condition::de_document_attribute_condition(tokens)?);
                        }
                        "Target" => {
                            builder = builder.set_target(crate::protocol_serde::shape_document_attribute_target::de_document_attribute_target(
                                tokens,
                            )?);
                        }
                        "DocumentContentDeletion" => {
                            builder =
                                builder.set_document_content_deletion(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
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
