// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_data_cells_filter_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::create_data_cells_filter::CreateDataCellsFilterInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.table_data {
        #[allow(unused_mut)]
        let mut object_2 = object.key("TableData").start_object();
        crate::protocol_serde::shape_data_cells_filter::ser_data_cells_filter(&mut object_2, var_1)?;
        object_2.finish();
    }
    Ok(())
}
