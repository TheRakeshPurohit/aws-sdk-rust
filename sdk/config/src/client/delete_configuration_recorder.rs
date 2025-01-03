// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteConfigurationRecorder`](crate::operation::delete_configuration_recorder::builders::DeleteConfigurationRecorderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`configuration_recorder_name(impl Into<String>)`](crate::operation::delete_configuration_recorder::builders::DeleteConfigurationRecorderFluentBuilder::configuration_recorder_name) / [`set_configuration_recorder_name(Option<String>)`](crate::operation::delete_configuration_recorder::builders::DeleteConfigurationRecorderFluentBuilder::set_configuration_recorder_name):<br>required: **true**<br><p>The name of the customer managed configuration recorder that you want to delete. You can retrieve the name of your configuration recorders by using the <a href="https://docs.aws.amazon.com/config/latest/APIReference/API_DescribeConfigurationRecorders.html">DescribeConfigurationRecorders</a> operation.</p><br>
    /// - On success, responds with [`DeleteConfigurationRecorderOutput`](crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderOutput)
    /// - On failure, responds with [`SdkError<DeleteConfigurationRecorderError>`](crate::operation::delete_configuration_recorder::DeleteConfigurationRecorderError)
    pub fn delete_configuration_recorder(
        &self,
    ) -> crate::operation::delete_configuration_recorder::builders::DeleteConfigurationRecorderFluentBuilder {
        crate::operation::delete_configuration_recorder::builders::DeleteConfigurationRecorderFluentBuilder::new(self.handle.clone())
    }
}
