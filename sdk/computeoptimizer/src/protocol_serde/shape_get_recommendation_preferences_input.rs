// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_recommendation_preferences_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_recommendation_preferences::GetRecommendationPreferencesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.resource_type {
        object.key("resourceType").string(var_1.as_str());
    }
    if let Some(var_2) = &input.scope {
        #[allow(unused_mut)]
        let mut object_3 = object.key("scope").start_object();
        crate::protocol_serde::shape_scope::ser_scope(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.next_token {
        object.key("nextToken").string(var_4.as_str());
    }
    if let Some(var_5) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_5).into()),
        );
    }
    Ok(())
}
