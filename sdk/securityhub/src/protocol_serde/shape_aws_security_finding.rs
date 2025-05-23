// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_aws_security_finding(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AwsSecurityFinding,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.schema_version {
        object.key("SchemaVersion").string(var_1.as_str());
    }
    if let Some(var_2) = &input.id {
        object.key("Id").string(var_2.as_str());
    }
    if let Some(var_3) = &input.product_arn {
        object.key("ProductArn").string(var_3.as_str());
    }
    if let Some(var_4) = &input.product_name {
        object.key("ProductName").string(var_4.as_str());
    }
    if let Some(var_5) = &input.company_name {
        object.key("CompanyName").string(var_5.as_str());
    }
    if let Some(var_6) = &input.region {
        object.key("Region").string(var_6.as_str());
    }
    if let Some(var_7) = &input.generator_id {
        object.key("GeneratorId").string(var_7.as_str());
    }
    if let Some(var_8) = &input.aws_account_id {
        object.key("AwsAccountId").string(var_8.as_str());
    }
    if let Some(var_9) = &input.types {
        let mut array_10 = object.key("Types").start_array();
        for item_11 in var_9 {
            {
                array_10.value().string(item_11.as_str());
            }
        }
        array_10.finish();
    }
    if let Some(var_12) = &input.first_observed_at {
        object.key("FirstObservedAt").string(var_12.as_str());
    }
    if let Some(var_13) = &input.last_observed_at {
        object.key("LastObservedAt").string(var_13.as_str());
    }
    if let Some(var_14) = &input.created_at {
        object.key("CreatedAt").string(var_14.as_str());
    }
    if let Some(var_15) = &input.updated_at {
        object.key("UpdatedAt").string(var_15.as_str());
    }
    if let Some(var_16) = &input.severity {
        #[allow(unused_mut)]
        let mut object_17 = object.key("Severity").start_object();
        crate::protocol_serde::shape_severity::ser_severity(&mut object_17, var_16)?;
        object_17.finish();
    }
    if let Some(var_18) = &input.confidence {
        object.key("Confidence").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_18).into()),
        );
    }
    if let Some(var_19) = &input.criticality {
        object.key("Criticality").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_19).into()),
        );
    }
    if let Some(var_20) = &input.title {
        object.key("Title").string(var_20.as_str());
    }
    if let Some(var_21) = &input.description {
        object.key("Description").string(var_21.as_str());
    }
    if let Some(var_22) = &input.remediation {
        #[allow(unused_mut)]
        let mut object_23 = object.key("Remediation").start_object();
        crate::protocol_serde::shape_remediation::ser_remediation(&mut object_23, var_22)?;
        object_23.finish();
    }
    if let Some(var_24) = &input.source_url {
        object.key("SourceUrl").string(var_24.as_str());
    }
    if let Some(var_25) = &input.product_fields {
        #[allow(unused_mut)]
        let mut object_26 = object.key("ProductFields").start_object();
        for (key_27, value_28) in var_25 {
            {
                object_26.key(key_27.as_str()).string(value_28.as_str());
            }
        }
        object_26.finish();
    }
    if let Some(var_29) = &input.user_defined_fields {
        #[allow(unused_mut)]
        let mut object_30 = object.key("UserDefinedFields").start_object();
        for (key_31, value_32) in var_29 {
            {
                object_30.key(key_31.as_str()).string(value_32.as_str());
            }
        }
        object_30.finish();
    }
    if let Some(var_33) = &input.malware {
        let mut array_34 = object.key("Malware").start_array();
        for item_35 in var_33 {
            {
                #[allow(unused_mut)]
                let mut object_36 = array_34.value().start_object();
                crate::protocol_serde::shape_malware::ser_malware(&mut object_36, item_35)?;
                object_36.finish();
            }
        }
        array_34.finish();
    }
    if let Some(var_37) = &input.network {
        #[allow(unused_mut)]
        let mut object_38 = object.key("Network").start_object();
        crate::protocol_serde::shape_network::ser_network(&mut object_38, var_37)?;
        object_38.finish();
    }
    if let Some(var_39) = &input.network_path {
        let mut array_40 = object.key("NetworkPath").start_array();
        for item_41 in var_39 {
            {
                #[allow(unused_mut)]
                let mut object_42 = array_40.value().start_object();
                crate::protocol_serde::shape_network_path_component::ser_network_path_component(&mut object_42, item_41)?;
                object_42.finish();
            }
        }
        array_40.finish();
    }
    if let Some(var_43) = &input.process {
        #[allow(unused_mut)]
        let mut object_44 = object.key("Process").start_object();
        crate::protocol_serde::shape_process_details::ser_process_details(&mut object_44, var_43)?;
        object_44.finish();
    }
    if let Some(var_45) = &input.threats {
        let mut array_46 = object.key("Threats").start_array();
        for item_47 in var_45 {
            {
                #[allow(unused_mut)]
                let mut object_48 = array_46.value().start_object();
                crate::protocol_serde::shape_threat::ser_threat(&mut object_48, item_47)?;
                object_48.finish();
            }
        }
        array_46.finish();
    }
    if let Some(var_49) = &input.threat_intel_indicators {
        let mut array_50 = object.key("ThreatIntelIndicators").start_array();
        for item_51 in var_49 {
            {
                #[allow(unused_mut)]
                let mut object_52 = array_50.value().start_object();
                crate::protocol_serde::shape_threat_intel_indicator::ser_threat_intel_indicator(&mut object_52, item_51)?;
                object_52.finish();
            }
        }
        array_50.finish();
    }
    if let Some(var_53) = &input.resources {
        let mut array_54 = object.key("Resources").start_array();
        for item_55 in var_53 {
            {
                #[allow(unused_mut)]
                let mut object_56 = array_54.value().start_object();
                crate::protocol_serde::shape_resource::ser_resource(&mut object_56, item_55)?;
                object_56.finish();
            }
        }
        array_54.finish();
    }
    if let Some(var_57) = &input.compliance {
        #[allow(unused_mut)]
        let mut object_58 = object.key("Compliance").start_object();
        crate::protocol_serde::shape_compliance::ser_compliance(&mut object_58, var_57)?;
        object_58.finish();
    }
    if let Some(var_59) = &input.verification_state {
        object.key("VerificationState").string(var_59.as_str());
    }
    if let Some(var_60) = &input.workflow_state {
        object.key("WorkflowState").string(var_60.as_str());
    }
    if let Some(var_61) = &input.workflow {
        #[allow(unused_mut)]
        let mut object_62 = object.key("Workflow").start_object();
        crate::protocol_serde::shape_workflow::ser_workflow(&mut object_62, var_61)?;
        object_62.finish();
    }
    if let Some(var_63) = &input.record_state {
        object.key("RecordState").string(var_63.as_str());
    }
    if let Some(var_64) = &input.related_findings {
        let mut array_65 = object.key("RelatedFindings").start_array();
        for item_66 in var_64 {
            {
                #[allow(unused_mut)]
                let mut object_67 = array_65.value().start_object();
                crate::protocol_serde::shape_related_finding::ser_related_finding(&mut object_67, item_66)?;
                object_67.finish();
            }
        }
        array_65.finish();
    }
    if let Some(var_68) = &input.note {
        #[allow(unused_mut)]
        let mut object_69 = object.key("Note").start_object();
        crate::protocol_serde::shape_note::ser_note(&mut object_69, var_68)?;
        object_69.finish();
    }
    if let Some(var_70) = &input.vulnerabilities {
        let mut array_71 = object.key("Vulnerabilities").start_array();
        for item_72 in var_70 {
            {
                #[allow(unused_mut)]
                let mut object_73 = array_71.value().start_object();
                crate::protocol_serde::shape_vulnerability::ser_vulnerability(&mut object_73, item_72)?;
                object_73.finish();
            }
        }
        array_71.finish();
    }
    if let Some(var_74) = &input.patch_summary {
        #[allow(unused_mut)]
        let mut object_75 = object.key("PatchSummary").start_object();
        crate::protocol_serde::shape_patch_summary::ser_patch_summary(&mut object_75, var_74)?;
        object_75.finish();
    }
    if let Some(var_76) = &input.action {
        #[allow(unused_mut)]
        let mut object_77 = object.key("Action").start_object();
        crate::protocol_serde::shape_action::ser_action(&mut object_77, var_76)?;
        object_77.finish();
    }
    if let Some(var_78) = &input.finding_provider_fields {
        #[allow(unused_mut)]
        let mut object_79 = object.key("FindingProviderFields").start_object();
        crate::protocol_serde::shape_finding_provider_fields::ser_finding_provider_fields(&mut object_79, var_78)?;
        object_79.finish();
    }
    if let Some(var_80) = &input.sample {
        object.key("Sample").boolean(*var_80);
    }
    if let Some(var_81) = &input.generator_details {
        #[allow(unused_mut)]
        let mut object_82 = object.key("GeneratorDetails").start_object();
        crate::protocol_serde::shape_generator_details::ser_generator_details(&mut object_82, var_81)?;
        object_82.finish();
    }
    if let Some(var_83) = &input.processed_at {
        object.key("ProcessedAt").string(var_83.as_str());
    }
    if let Some(var_84) = &input.aws_account_name {
        object.key("AwsAccountName").string(var_84.as_str());
    }
    if let Some(var_85) = &input.detection {
        #[allow(unused_mut)]
        let mut object_86 = object.key("Detection").start_object();
        crate::protocol_serde::shape_detection::ser_detection(&mut object_86, var_85)?;
        object_86.finish();
    }
    Ok(())
}

pub(crate) fn de_aws_security_finding<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::AwsSecurityFinding>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::AwsSecurityFindingBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "SchemaVersion" => {
                            builder = builder.set_schema_version(
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
                        "ProductArn" => {
                            builder = builder.set_product_arn(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ProductName" => {
                            builder = builder.set_product_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "CompanyName" => {
                            builder = builder.set_company_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Region" => {
                            builder = builder.set_region(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "GeneratorId" => {
                            builder = builder.set_generator_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "AwsAccountId" => {
                            builder = builder.set_aws_account_id(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Types" => {
                            builder = builder.set_types(crate::protocol_serde::shape_type_list::de_type_list(tokens)?);
                        }
                        "FirstObservedAt" => {
                            builder = builder.set_first_observed_at(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "LastObservedAt" => {
                            builder = builder.set_last_observed_at(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "CreatedAt" => {
                            builder = builder.set_created_at(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "UpdatedAt" => {
                            builder = builder.set_updated_at(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Severity" => {
                            builder = builder.set_severity(crate::protocol_serde::shape_severity::de_severity(tokens)?);
                        }
                        "Confidence" => {
                            builder = builder.set_confidence(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "Criticality" => {
                            builder = builder.set_criticality(
                                ::aws_smithy_json::deserialize::token::expect_number_or_null(tokens.next())?
                                    .map(i32::try_from)
                                    .transpose()?,
                            );
                        }
                        "Title" => {
                            builder = builder.set_title(
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
                        "Remediation" => {
                            builder = builder.set_remediation(crate::protocol_serde::shape_remediation::de_remediation(tokens)?);
                        }
                        "SourceUrl" => {
                            builder = builder.set_source_url(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "ProductFields" => {
                            builder = builder.set_product_fields(crate::protocol_serde::shape_field_map::de_field_map(tokens)?);
                        }
                        "UserDefinedFields" => {
                            builder = builder.set_user_defined_fields(crate::protocol_serde::shape_field_map::de_field_map(tokens)?);
                        }
                        "Malware" => {
                            builder = builder.set_malware(crate::protocol_serde::shape_malware_list::de_malware_list(tokens)?);
                        }
                        "Network" => {
                            builder = builder.set_network(crate::protocol_serde::shape_network::de_network(tokens)?);
                        }
                        "NetworkPath" => {
                            builder = builder.set_network_path(crate::protocol_serde::shape_network_path_list::de_network_path_list(tokens)?);
                        }
                        "Process" => {
                            builder = builder.set_process(crate::protocol_serde::shape_process_details::de_process_details(tokens)?);
                        }
                        "Threats" => {
                            builder = builder.set_threats(crate::protocol_serde::shape_threat_list::de_threat_list(tokens)?);
                        }
                        "ThreatIntelIndicators" => {
                            builder = builder.set_threat_intel_indicators(
                                crate::protocol_serde::shape_threat_intel_indicator_list::de_threat_intel_indicator_list(tokens)?,
                            );
                        }
                        "Resources" => {
                            builder = builder.set_resources(crate::protocol_serde::shape_resource_list::de_resource_list(tokens)?);
                        }
                        "Compliance" => {
                            builder = builder.set_compliance(crate::protocol_serde::shape_compliance::de_compliance(tokens)?);
                        }
                        "VerificationState" => {
                            builder = builder.set_verification_state(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::VerificationState::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "WorkflowState" => {
                            builder = builder.set_workflow_state(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::WorkflowState::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "Workflow" => {
                            builder = builder.set_workflow(crate::protocol_serde::shape_workflow::de_workflow(tokens)?);
                        }
                        "RecordState" => {
                            builder = builder.set_record_state(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| crate::types::RecordState::from(u.as_ref())))
                                    .transpose()?,
                            );
                        }
                        "RelatedFindings" => {
                            builder =
                                builder.set_related_findings(crate::protocol_serde::shape_related_finding_list::de_related_finding_list(tokens)?);
                        }
                        "Note" => {
                            builder = builder.set_note(crate::protocol_serde::shape_note::de_note(tokens)?);
                        }
                        "Vulnerabilities" => {
                            builder = builder.set_vulnerabilities(crate::protocol_serde::shape_vulnerability_list::de_vulnerability_list(tokens)?);
                        }
                        "PatchSummary" => {
                            builder = builder.set_patch_summary(crate::protocol_serde::shape_patch_summary::de_patch_summary(tokens)?);
                        }
                        "Action" => {
                            builder = builder.set_action(crate::protocol_serde::shape_action::de_action(tokens)?);
                        }
                        "FindingProviderFields" => {
                            builder = builder.set_finding_provider_fields(
                                crate::protocol_serde::shape_finding_provider_fields::de_finding_provider_fields(tokens)?,
                            );
                        }
                        "Sample" => {
                            builder = builder.set_sample(::aws_smithy_json::deserialize::token::expect_bool_or_null(tokens.next())?);
                        }
                        "GeneratorDetails" => {
                            builder = builder.set_generator_details(crate::protocol_serde::shape_generator_details::de_generator_details(tokens)?);
                        }
                        "ProcessedAt" => {
                            builder = builder.set_processed_at(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "AwsAccountName" => {
                            builder = builder.set_aws_account_name(
                                ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                                    .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                                    .transpose()?,
                            );
                        }
                        "Detection" => {
                            builder = builder.set_detection(crate::protocol_serde::shape_detection::de_detection(tokens)?);
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
            Ok(Some(crate::serde_util::aws_security_finding_correct_errors(builder).build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}
