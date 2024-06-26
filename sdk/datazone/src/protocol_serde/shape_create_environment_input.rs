// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_environment_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_environment::CreateEnvironmentInput,
) -> Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.environment_account_identifier {
        object.key("environmentAccountIdentifier").string(var_2.as_str());
    }
    if let Some(var_3) = &input.environment_account_region {
        object.key("environmentAccountRegion").string(var_3.as_str());
    }
    if let Some(var_4) = &input.environment_blueprint_identifier {
        object.key("environmentBlueprintIdentifier").string(var_4.as_str());
    }
    if let Some(var_5) = &input.environment_profile_identifier {
        object.key("environmentProfileIdentifier").string(var_5.as_str());
    }
    if let Some(var_6) = &input.glossary_terms {
        let mut array_7 = object.key("glossaryTerms").start_array();
        for item_8 in var_6 {
            {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    if let Some(var_9) = &input.name {
        object.key("name").string(var_9.as_str());
    }
    if let Some(var_10) = &input.project_identifier {
        object.key("projectIdentifier").string(var_10.as_str());
    }
    if let Some(var_11) = &input.user_parameters {
        let mut array_12 = object.key("userParameters").start_array();
        for item_13 in var_11 {
            {
                #[allow(unused_mut)]
                let mut object_14 = array_12.value().start_object();
                crate::protocol_serde::shape_environment_parameter::ser_environment_parameter(&mut object_14, item_13)?;
                object_14.finish();
            }
        }
        array_12.finish();
    }
    Ok(())
}
