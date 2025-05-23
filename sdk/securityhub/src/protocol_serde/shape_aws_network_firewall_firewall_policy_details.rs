// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_network_firewall_firewall_policy_details(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsNetworkFirewallFirewallPolicyDetails,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.firewall_policy {
        #[allow(unused_mut)]
        let mut object_2 = object.key("FirewallPolicy").start_object();
        crate::protocol_serde::shape_firewall_policy_details::ser_firewall_policy_details(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.firewall_policy_arn {
        object.key("FirewallPolicyArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.firewall_policy_id {
        object.key("FirewallPolicyId").string(var_4.as_str());
    }
    if let Some(var_5) = &input.firewall_policy_name {
        object.key("FirewallPolicyName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.description {
        object.key("Description").string(var_6.as_str());
    }
    Ok(())
}

pub(crate) fn de_aws_network_firewall_firewall_policy_details<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AwsNetworkFirewallFirewallPolicyDetails>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AwsNetworkFirewallFirewallPolicyDetailsBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "FirewallPolicy" => {
                            builder = builder
                                .set_firewall_policy(crate::protocol_serde::shape_firewall_policy_details::de_firewall_policy_details(tokens)?);
                        }
                        "FirewallPolicyArn" => {
                            builder = builder.set_firewall_policy_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "FirewallPolicyId" => {
                            builder = builder.set_firewall_policy_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "FirewallPolicyName" => {
                            builder = builder.set_firewall_policy_name(
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
