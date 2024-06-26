// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTestSuites`](crate::operation::list_test_suites::builders::ListTestSuitesFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_test_suites::builders::ListTestSuitesFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`test_suite_ids(impl Into<String>)`](crate::operation::list_test_suites::builders::ListTestSuitesFluentBuilder::test_suite_ids) / [`set_test_suite_ids(Option<Vec::<String>>)`](crate::operation::list_test_suites::builders::ListTestSuitesFluentBuilder::set_test_suite_ids):<br>required: **false**<br><p>The suite ID of the test suites.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_test_suites::builders::ListTestSuitesFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_test_suites::builders::ListTestSuitesFluentBuilder::set_next_token):<br>required: **false**<br><p>The token from a previous request to retrieve the next page of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_test_suites::builders::ListTestSuitesFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_test_suites::builders::ListTestSuitesFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of test suites to return in one page of results.</p><br>
    /// - On success, responds with [`ListTestSuitesOutput`](crate::operation::list_test_suites::ListTestSuitesOutput) with field(s):
    ///   - [`test_suites(Vec::<TestSuiteSummary>)`](crate::operation::list_test_suites::ListTestSuitesOutput::test_suites): <p>The test suites returned with the response query.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_test_suites::ListTestSuitesOutput::next_token): <p>The token from a previous request to retrieve the next page of test suites results.</p>
    /// - On failure, responds with [`SdkError<ListTestSuitesError>`](crate::operation::list_test_suites::ListTestSuitesError)
    pub fn list_test_suites(&self) -> crate::operation::list_test_suites::builders::ListTestSuitesFluentBuilder {
        crate::operation::list_test_suites::builders::ListTestSuitesFluentBuilder::new(self.handle.clone())
    }
}
