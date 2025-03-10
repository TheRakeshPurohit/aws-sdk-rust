// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_delete_relational_database_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::delete_relational_database::DeleteRelationalDatabaseInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.relational_database_name {
        object.key("relationalDatabaseName").string(var_1.as_str());
    }
    if let Some(var_2) = &input.skip_final_snapshot {
        object.key("skipFinalSnapshot").boolean(*var_2);
    }
    if let Some(var_3) = &input.final_relational_database_snapshot_name {
        object.key("finalRelationalDatabaseSnapshotName").string(var_3.as_str());
    }
    Ok(())
}
