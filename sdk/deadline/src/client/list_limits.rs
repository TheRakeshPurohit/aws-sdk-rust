// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListLimits`](crate::operation::list_limits::builders::ListLimitsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_limits::builders::ListLimitsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`farm_id(impl Into<String>)`](crate::operation::list_limits::builders::ListLimitsFluentBuilder::farm_id) / [`set_farm_id(Option<String>)`](crate::operation::list_limits::builders::ListLimitsFluentBuilder::set_farm_id):<br>required: **true**<br><p>The unique identifier of the farm that contains the limits.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_limits::builders::ListLimitsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_limits::builders::ListLimitsFluentBuilder::set_next_token):<br>required: **false**<br><p>The token for the next set of results, or <code>null</code> to start from the beginning.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_limits::builders::ListLimitsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_limits::builders::ListLimitsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of limits to return in each page of results.</p><br>
    /// - On success, responds with [`ListLimitsOutput`](crate::operation::list_limits::ListLimitsOutput) with field(s):
    ///   - [`limits(Vec::<LimitSummary>)`](crate::operation::list_limits::ListLimitsOutput::limits): <p>A list of limits that the farm contains.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_limits::ListLimitsOutput::next_token): <p>If Deadline Cloud returns <code>nextToken</code>, then there are more results available. The value of <code>nextToken</code> is a unique pagination token for each page. To retrieve the next page, call the operation again using the returned token. Keep all other arguments unchanged. If no results remain, then <code>nextToken</code> is set to <code>null</code>. Each pagination token expires after 24 hours. If you provide a token that isn't valid, then you receive an HTTP 400 <code>ValidationException</code> error.</p>
    /// - On failure, responds with [`SdkError<ListLimitsError>`](crate::operation::list_limits::ListLimitsError)
    pub fn list_limits(&self) -> crate::operation::list_limits::builders::ListLimitsFluentBuilder {
        crate::operation::list_limits::builders::ListLimitsFluentBuilder::new(self.handle.clone())
    }
}
