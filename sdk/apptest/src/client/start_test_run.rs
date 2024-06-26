// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartTestRun`](crate::operation::start_test_run::builders::StartTestRunFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`test_suite_id(impl Into<String>)`](crate::operation::start_test_run::builders::StartTestRunFluentBuilder::test_suite_id) / [`set_test_suite_id(Option<String>)`](crate::operation::start_test_run::builders::StartTestRunFluentBuilder::set_test_suite_id):<br>required: **true**<br><p>The test suite ID of the test run.</p><br>
    ///   - [`test_configuration_id(impl Into<String>)`](crate::operation::start_test_run::builders::StartTestRunFluentBuilder::test_configuration_id) / [`set_test_configuration_id(Option<String>)`](crate::operation::start_test_run::builders::StartTestRunFluentBuilder::set_test_configuration_id):<br>required: **false**<br><p>The configuration ID of the test run.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::start_test_run::builders::StartTestRunFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::start_test_run::builders::StartTestRunFluentBuilder::set_client_token):<br>required: **false**<br><p>The client token of the test run.</p><br>
    ///   - [`tags(impl Into<String>, impl Into<String>)`](crate::operation::start_test_run::builders::StartTestRunFluentBuilder::tags) / [`set_tags(Option<HashMap::<String, String>>)`](crate::operation::start_test_run::builders::StartTestRunFluentBuilder::set_tags):<br>required: **false**<br><p>The tags of the test run.</p><br>
    /// - On success, responds with [`StartTestRunOutput`](crate::operation::start_test_run::StartTestRunOutput) with field(s):
    ///   - [`test_run_id(String)`](crate::operation::start_test_run::StartTestRunOutput::test_run_id): <p>The test run ID of the test run.</p>
    ///   - [`test_run_status(TestRunStatus)`](crate::operation::start_test_run::StartTestRunOutput::test_run_status): <p>The test run status of the test run.</p>
    /// - On failure, responds with [`SdkError<StartTestRunError>`](crate::operation::start_test_run::StartTestRunError)
    pub fn start_test_run(&self) -> crate::operation::start_test_run::builders::StartTestRunFluentBuilder {
        crate::operation::start_test_run::builders::StartTestRunFluentBuilder::new(self.handle.clone())
    }
}
