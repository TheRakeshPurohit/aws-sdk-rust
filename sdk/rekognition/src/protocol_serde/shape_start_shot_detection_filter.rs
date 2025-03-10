// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_start_shot_detection_filter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::StartShotDetectionFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.min_segment_confidence {
        object.key("MinSegmentConfidence").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_1).into()),
        );
    }
    Ok(())
}
