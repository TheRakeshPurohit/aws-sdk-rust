// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_s3_identifier(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::S3Identifier,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.s3_bucket_name {
        object.key("s3BucketName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.s3_object_key {
        object.key("s3ObjectKey").string(var_2.as_str());
    }
    Ok(())
}
