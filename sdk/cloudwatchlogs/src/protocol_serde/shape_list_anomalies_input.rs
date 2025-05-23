// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_anomalies_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::list_anomalies::ListAnomaliesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.anomaly_detector_arn {
        object.key("anomalyDetectorArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.suppression_state {
        object.key("suppressionState").string(var_2.as_str());
    }
    if let Some(var_3) = &input.limit {
        object.key("limit").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_3).into()),
        );
    }
    if let Some(var_4) = &input.next_token {
        object.key("nextToken").string(var_4.as_str());
    }
    Ok(())
}
