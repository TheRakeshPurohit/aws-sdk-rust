// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_metric_attribution(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::MetricAttribution,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object.key("eventAttributionSource").string(input.event_attribution_source.as_str());
    }
    Ok(())
}
