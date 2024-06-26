// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteTestRun`](crate::operation::delete_test_run::builders::DeleteTestRunFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`test_run_id(impl Into<String>)`](crate::operation::delete_test_run::builders::DeleteTestRunFluentBuilder::test_run_id) / [`set_test_run_id(Option<String>)`](crate::operation::delete_test_run::builders::DeleteTestRunFluentBuilder::set_test_run_id):<br>required: **true**<br><p>The run ID of the test run.</p><br>
    /// - On success, responds with [`DeleteTestRunOutput`](crate::operation::delete_test_run::DeleteTestRunOutput)
    /// - On failure, responds with [`SdkError<DeleteTestRunError>`](crate::operation::delete_test_run::DeleteTestRunError)
    pub fn delete_test_run(&self) -> crate::operation::delete_test_run::builders::DeleteTestRunFluentBuilder {
        crate::operation::delete_test_run::builders::DeleteTestRunFluentBuilder::new(self.handle.clone())
    }
}
