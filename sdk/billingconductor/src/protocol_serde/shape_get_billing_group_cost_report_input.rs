// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_get_billing_group_cost_report_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::get_billing_group_cost_report::GetBillingGroupCostReportInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.arn {
        object.key("Arn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.billing_period_range {
        #[allow(unused_mut)]
        let mut object_3 = object.key("BillingPeriodRange").start_object();
        crate::protocol_serde::shape_billing_period_range::ser_billing_period_range(&mut object_3, var_2)?;
        object_3.finish();
    }
    if let Some(var_4) = &input.group_by {
        let mut array_5 = object.key("GroupBy").start_array();
        for item_6 in var_4 {
            {
                array_5.value().string(item_6.as_str());
            }
        }
        array_5.finish();
    }
    if let Some(var_7) = &input.max_results {
        object.key("MaxResults").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_7).into()),
        );
    }
    if let Some(var_8) = &input.next_token {
        object.key("NextToken").string(var_8.as_str());
    }
    Ok(())
}
