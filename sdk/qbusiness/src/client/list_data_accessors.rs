// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListDataAccessors`](crate::operation::list_data_accessors::builders::ListDataAccessorsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_data_accessors::builders::ListDataAccessorsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`application_id(impl Into<String>)`](crate::operation::list_data_accessors::builders::ListDataAccessorsFluentBuilder::application_id) / [`set_application_id(Option<String>)`](crate::operation::list_data_accessors::builders::ListDataAccessorsFluentBuilder::set_application_id):<br>required: **true**<br><p>The unique identifier of the Amazon Q Business application.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_data_accessors::builders::ListDataAccessorsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_data_accessors::builders::ListDataAccessorsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of results. (You received this token from a previous call.)</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_data_accessors::builders::ListDataAccessorsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_data_accessors::builders::ListDataAccessorsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return in a single call.</p><br>
    /// - On success, responds with [`ListDataAccessorsOutput`](crate::operation::list_data_accessors::ListDataAccessorsOutput) with field(s):
    ///   - [`data_accessors(Option<Vec::<DataAccessor>>)`](crate::operation::list_data_accessors::ListDataAccessorsOutput::data_accessors): <p>The list of data accessors.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_data_accessors::ListDataAccessorsOutput::next_token): <p>The token to use to retrieve the next set of results, if there are any.</p>
    /// - On failure, responds with [`SdkError<ListDataAccessorsError>`](crate::operation::list_data_accessors::ListDataAccessorsError)
    pub fn list_data_accessors(&self) -> crate::operation::list_data_accessors::builders::ListDataAccessorsFluentBuilder {
        crate::operation::list_data_accessors::builders::ListDataAccessorsFluentBuilder::new(self.handle.clone())
    }
}
