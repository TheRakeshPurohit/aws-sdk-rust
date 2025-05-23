// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_export_key_cryptogram(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ExportKeyCryptogram,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        object
            .key("CertificateAuthorityPublicKeyIdentifier")
            .string(input.certificate_authority_public_key_identifier.as_str());
    }
    {
        object.key("WrappingKeyCertificate").string(input.wrapping_key_certificate.as_str());
    }
    if let Some(var_1) = &input.wrapping_spec {
        object.key("WrappingSpec").string(var_1.as_str());
    }
    Ok(())
}
