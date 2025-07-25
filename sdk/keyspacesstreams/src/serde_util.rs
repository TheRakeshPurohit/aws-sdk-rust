// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn get_stream_output_output_correct_errors(
    mut builder: crate::operation::get_stream::builders::GetStreamOutputBuilder,
) -> crate::operation::get_stream::builders::GetStreamOutputBuilder {
    if builder.stream_arn.is_none() {
        builder.stream_arn = Some(Default::default())
    }
    if builder.stream_label.is_none() {
        builder.stream_label = Some(Default::default())
    }
    if builder.stream_status.is_none() {
        builder.stream_status = "no value was set".parse::<crate::types::StreamStatus>().ok()
    }
    if builder.stream_view_type.is_none() {
        builder.stream_view_type = "no value was set".parse::<crate::types::StreamViewType>().ok()
    }
    if builder.creation_request_date_time.is_none() {
        builder.creation_request_date_time = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.keyspace_name.is_none() {
        builder.keyspace_name = Some(Default::default())
    }
    if builder.table_name.is_none() {
        builder.table_name = Some(Default::default())
    }
    builder
}

pub(crate) fn stream_correct_errors(mut builder: crate::types::builders::StreamBuilder) -> crate::types::builders::StreamBuilder {
    if builder.stream_arn.is_none() {
        builder.stream_arn = Some(Default::default())
    }
    if builder.keyspace_name.is_none() {
        builder.keyspace_name = Some(Default::default())
    }
    if builder.table_name.is_none() {
        builder.table_name = Some(Default::default())
    }
    if builder.stream_label.is_none() {
        builder.stream_label = Some(Default::default())
    }
    builder
}
