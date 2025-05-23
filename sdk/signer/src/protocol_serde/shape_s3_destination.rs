// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_destination(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::S3Destination,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.bucket_name {
        object.key("bucketName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.prefix {
        object.key("prefix").string(var_2.as_str());
    }
    Ok(())
}
