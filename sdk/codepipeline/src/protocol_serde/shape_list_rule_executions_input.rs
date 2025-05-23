// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_rule_executions_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_rule_executions::ListRuleExecutionsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.pipeline_name {
        object.key("pipelineName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filter {
        #[allow(unused_mut)]
        let mut object_3 = object.key("filter").start_object();
        crate::protocol_serde::shape_rule_execution_filter::ser_rule_execution_filter(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_4).into()),
        );
    }
    if let Some(var_5) = &input.next_token {
        object.key("nextToken").string(var_5.as_str());
    }
    Ok(())
}
