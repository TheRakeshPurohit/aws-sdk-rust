// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_spark_emr_properties_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::SparkEmrPropertiesInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.compute_arn {
        object.key("computeArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.instance_profile_arn {
        object.key("instanceProfileArn").string(var_2.as_str());
    }
    if let Some(var_3) = &input.java_virtual_env {
        object.key("javaVirtualEnv").string(var_3.as_str());
    }
    if let Some(var_4) = &input.log_uri {
        object.key("logUri").string(var_4.as_str());
    }
    if let Some(var_5) = &input.python_virtual_env {
        object.key("pythonVirtualEnv").string(var_5.as_str());
    }
    if let Some(var_6) = &input.runtime_role {
        object.key("runtimeRole").string(var_6.as_str());
    }
    if let Some(var_7) = &input.trusted_certificates_s3_uri {
        object.key("trustedCertificatesS3Uri").string(var_7.as_str());
    }
    Ok(())
}
