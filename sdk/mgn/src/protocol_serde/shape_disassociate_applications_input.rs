// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_disassociate_applications_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::disassociate_applications::DisassociateApplicationsInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.account_id {
        object.key("accountID").string(var_1.as_str());
    }
    if let Some(var_2) = &input.application_ids {
        let mut array_3 = object.key("applicationIDs").start_array();
        for item_4 in var_2 {
            {
                array_3.value().string(item_4.as_str());
            }
        }
        array_3.finish();
    }
    if let Some(var_5) = &input.wave_id {
        object.key("waveID").string(var_5.as_str());
    }
    Ok(())
}
