// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_jobs_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_jobs::ListJobsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.array_job_id {
        object.key("arrayJobId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.filters {
        let mut array_3 = object.key("filters").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_key_values_pair::ser_key_values_pair(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.job_queue {
        object.key("jobQueue").string(var_6.as_str());
    }
    if let Some(var_7) = &input.job_status {
        object.key("jobStatus").string(var_7.as_str());
    }
    if let Some(var_8) = &input.max_results {
        object.key("maxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_8).into()),
        );
    }
    if let Some(var_9) = &input.multi_node_job_id {
        object.key("multiNodeJobId").string(var_9.as_str());
    }
    if let Some(var_10) = &input.next_token {
        object.key("nextToken").string(var_10.as_str());
    }
    Ok(())
}
