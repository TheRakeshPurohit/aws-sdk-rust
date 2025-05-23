// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_batch_associate_assessment_report_evidence_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::batch_associate_assessment_report_evidence::BatchAssociateAssessmentReportEvidenceInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.evidence_folder_id {
        object.key("evidenceFolderId").string(var_1.as_str());
    }
    if let Some(var_2) = &input.evidence_ids {
        let mut array_3 = object.key("evidenceIds").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    Ok(())
}
