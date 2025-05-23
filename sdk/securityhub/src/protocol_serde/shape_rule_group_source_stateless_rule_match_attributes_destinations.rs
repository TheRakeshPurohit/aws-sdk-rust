// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_rule_group_source_stateless_rule_match_attributes_destinations(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::RuleGroupSourceStatelessRuleMatchAttributesDestinations,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.address_definition {
        object.key("AddressDefinition").string(var_1.as_str());
    }
    Ok(())
}

pub(crate) fn de_rule_group_source_stateless_rule_match_attributes_destinations<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<
    Option<crate::types::RuleGroupSourceStatelessRuleMatchAttributesDestinations>,
    ::aws_smithy_json::deserialize::error::DeserializeError,
>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::RuleGroupSourceStatelessRuleMatchAttributesDestinationsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "AddressDefinition" => {
                            builder = builder.set_address_definition(
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
