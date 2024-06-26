// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListReadSetActivationJobs`](crate::operation::list_read_set_activation_jobs::builders::ListReadSetActivationJobsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_read_set_activation_jobs::builders::ListReadSetActivationJobsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`sequence_store_id(impl Into<String>)`](crate::operation::list_read_set_activation_jobs::builders::ListReadSetActivationJobsFluentBuilder::sequence_store_id) / [`set_sequence_store_id(Option<String>)`](crate::operation::list_read_set_activation_jobs::builders::ListReadSetActivationJobsFluentBuilder::set_sequence_store_id):<br>required: **true**<br><p>The read set's sequence store ID.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_read_set_activation_jobs::builders::ListReadSetActivationJobsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_read_set_activation_jobs::builders::ListReadSetActivationJobsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of read set activation jobs to return in one page of results.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_read_set_activation_jobs::builders::ListReadSetActivationJobsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_read_set_activation_jobs::builders::ListReadSetActivationJobsFluentBuilder::set_next_token):<br>required: **false**<br><p>Specify the pagination token from a previous request to retrieve the next page of results.</p><br>
    ///   - [`filter(ActivateReadSetFilter)`](crate::operation::list_read_set_activation_jobs::builders::ListReadSetActivationJobsFluentBuilder::filter) / [`set_filter(Option<ActivateReadSetFilter>)`](crate::operation::list_read_set_activation_jobs::builders::ListReadSetActivationJobsFluentBuilder::set_filter):<br>required: **false**<br><p>A filter to apply to the list.</p><br>
    /// - On success, responds with [`ListReadSetActivationJobsOutput`](crate::operation::list_read_set_activation_jobs::ListReadSetActivationJobsOutput) with field(s):
    ///   - [`next_token(Option<String>)`](crate::operation::list_read_set_activation_jobs::ListReadSetActivationJobsOutput::next_token): <p>A pagination token that's included if more results are available.</p>
    ///   - [`activation_jobs(Option<Vec::<ActivateReadSetJobItem>>)`](crate::operation::list_read_set_activation_jobs::ListReadSetActivationJobsOutput::activation_jobs): <p>A list of jobs.</p>
    /// - On failure, responds with [`SdkError<ListReadSetActivationJobsError>`](crate::operation::list_read_set_activation_jobs::ListReadSetActivationJobsError)
    pub fn list_read_set_activation_jobs(&self) -> crate::operation::list_read_set_activation_jobs::builders::ListReadSetActivationJobsFluentBuilder {
        crate::operation::list_read_set_activation_jobs::builders::ListReadSetActivationJobsFluentBuilder::new(self.handle.clone())
    }
}
