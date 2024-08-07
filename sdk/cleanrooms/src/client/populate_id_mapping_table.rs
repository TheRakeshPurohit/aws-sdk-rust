// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PopulateIdMappingTable`](crate::operation::populate_id_mapping_table::builders::PopulateIdMappingTableFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`id_mapping_table_identifier(impl Into<String>)`](crate::operation::populate_id_mapping_table::builders::PopulateIdMappingTableFluentBuilder::id_mapping_table_identifier) / [`set_id_mapping_table_identifier(Option<String>)`](crate::operation::populate_id_mapping_table::builders::PopulateIdMappingTableFluentBuilder::set_id_mapping_table_identifier):<br>required: **true**<br><p>The unique identifier of the ID mapping table that you want to populate.</p><br>
    ///   - [`membership_identifier(impl Into<String>)`](crate::operation::populate_id_mapping_table::builders::PopulateIdMappingTableFluentBuilder::membership_identifier) / [`set_membership_identifier(Option<String>)`](crate::operation::populate_id_mapping_table::builders::PopulateIdMappingTableFluentBuilder::set_membership_identifier):<br>required: **true**<br><p>The unique identifier of the membership that contains the ID mapping table that you want to populate.</p><br>
    /// - On success, responds with [`PopulateIdMappingTableOutput`](crate::operation::populate_id_mapping_table::PopulateIdMappingTableOutput) with field(s):
    ///   - [`id_mapping_job_id(String)`](crate::operation::populate_id_mapping_table::PopulateIdMappingTableOutput::id_mapping_job_id): <p>The unique identifier of the mapping job that will populate the ID mapping table.</p>
    /// - On failure, responds with [`SdkError<PopulateIdMappingTableError>`](crate::operation::populate_id_mapping_table::PopulateIdMappingTableError)
    pub fn populate_id_mapping_table(&self) -> crate::operation::populate_id_mapping_table::builders::PopulateIdMappingTableFluentBuilder {
        crate::operation::populate_id_mapping_table::builders::PopulateIdMappingTableFluentBuilder::new(self.handle.clone())
    }
}
