// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_eks_cluster_details(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsEksClusterDetails,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.arn {
        object.key("Arn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.certificate_authority_data {
        object.key("CertificateAuthorityData").string(var_2.as_str());
    }
    if let Some(var_3) = &input.cluster_status {
        object.key("ClusterStatus").string(var_3.as_str());
    }
    if let Some(var_4) = &input.endpoint {
        object.key("Endpoint").string(var_4.as_str());
    }
    if let Some(var_5) = &input.name {
        object.key("Name").string(var_5.as_str());
    }
    if let Some(var_6) = &input.resources_vpc_config {
        #[allow(unused_mut)]
        let mut object_7 = object.key("ResourcesVpcConfig").start_object();
        crate::protocol_serde::shape_aws_eks_cluster_resources_vpc_config_details::ser_aws_eks_cluster_resources_vpc_config_details(
            &mut object_7,
            var_6,
        )?;
        object_7.finish();
    }
    if let Some(var_8) = &input.role_arn {
        object.key("RoleArn").string(var_8.as_str());
    }
    if let Some(var_9) = &input.version {
        object.key("Version").string(var_9.as_str());
    }
    if let Some(var_10) = &input.logging {
        #[allow(unused_mut)]
        let mut object_11 = object.key("Logging").start_object();
        crate::protocol_serde::shape_aws_eks_cluster_logging_details::ser_aws_eks_cluster_logging_details(&mut object_11, var_10)?;
        object_11.finish();
    }
    Ok(())
}

pub(crate) fn de_aws_eks_cluster_details<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AwsEksClusterDetails>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AwsEksClusterDetailsBuilder::default();
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
                        "CertificateAuthorityData" => {
                            builder = builder.set_certificate_authority_data(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ClusterStatus" => {
                            builder = builder.set_cluster_status(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Endpoint" => {
                            builder = builder.set_endpoint(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
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
                        "ResourcesVpcConfig" => {
                            builder = builder.set_resources_vpc_config(
                                    crate::protocol_serde::shape_aws_eks_cluster_resources_vpc_config_details::de_aws_eks_cluster_resources_vpc_config_details(tokens)?
                                );
                        }
                        "RoleArn" => {
                            builder = builder.set_role_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Version" => {
                            builder = builder.set_version(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Logging" => {
                            builder = builder.set_logging(
                                crate::protocol_serde::shape_aws_eks_cluster_logging_details::de_aws_eks_cluster_logging_details(tokens)?,
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
