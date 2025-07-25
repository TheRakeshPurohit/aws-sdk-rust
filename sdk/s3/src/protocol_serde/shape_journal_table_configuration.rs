// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_journal_table_configuration(
    input: &crate::types::JournalTableConfiguration,
    writer: ::aws_smithy_xml::encode::ElWriter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope = writer.finish();
    if let Some(var_1) = &input.record_expiration {
        let inner_writer = scope.start_el("RecordExpiration");
        crate::protocol_serde::shape_record_expiration::ser_record_expiration(var_1, inner_writer)?
    }
    if let Some(var_2) = &input.encryption_configuration {
        let inner_writer = scope.start_el("EncryptionConfiguration");
        crate::protocol_serde::shape_metadata_table_encryption_configuration::ser_metadata_table_encryption_configuration(var_2, inner_writer)?
    }
    scope.finish();
    Ok(())
}
