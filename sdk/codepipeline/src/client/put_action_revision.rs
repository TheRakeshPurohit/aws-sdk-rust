// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`PutActionRevision`](crate::operation::put_action_revision::builders::PutActionRevisionFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`pipeline_name(impl Into<String>)`](crate::operation::put_action_revision::builders::PutActionRevisionFluentBuilder::pipeline_name) / [`set_pipeline_name(Option<String>)`](crate::operation::put_action_revision::builders::PutActionRevisionFluentBuilder::set_pipeline_name):<br>required: **true**<br><p>The name of the pipeline that starts processing the revision to the source.</p><br>
    ///   - [`stage_name(impl Into<String>)`](crate::operation::put_action_revision::builders::PutActionRevisionFluentBuilder::stage_name) / [`set_stage_name(Option<String>)`](crate::operation::put_action_revision::builders::PutActionRevisionFluentBuilder::set_stage_name):<br>required: **true**<br><p>The name of the stage that contains the action that acts on the revision.</p><br>
    ///   - [`action_name(impl Into<String>)`](crate::operation::put_action_revision::builders::PutActionRevisionFluentBuilder::action_name) / [`set_action_name(Option<String>)`](crate::operation::put_action_revision::builders::PutActionRevisionFluentBuilder::set_action_name):<br>required: **true**<br><p>The name of the action that processes the revision.</p><br>
    ///   - [`action_revision(ActionRevision)`](crate::operation::put_action_revision::builders::PutActionRevisionFluentBuilder::action_revision) / [`set_action_revision(Option<ActionRevision>)`](crate::operation::put_action_revision::builders::PutActionRevisionFluentBuilder::set_action_revision):<br>required: **true**<br><p>Represents information about the version (or revision) of an action.</p><br>
    /// - On success, responds with [`PutActionRevisionOutput`](crate::operation::put_action_revision::PutActionRevisionOutput) with field(s):
    ///   - [`new_revision(bool)`](crate::operation::put_action_revision::PutActionRevisionOutput::new_revision): <p>Indicates whether the artifact revision was previously used in an execution of the specified pipeline.</p>
    ///   - [`pipeline_execution_id(Option<String>)`](crate::operation::put_action_revision::PutActionRevisionOutput::pipeline_execution_id): <p>The ID of the current workflow state of the pipeline.</p>
    /// - On failure, responds with [`SdkError<PutActionRevisionError>`](crate::operation::put_action_revision::PutActionRevisionError)
    pub fn put_action_revision(&self) -> crate::operation::put_action_revision::builders::PutActionRevisionFluentBuilder {
        crate::operation::put_action_revision::builders::PutActionRevisionFluentBuilder::new(self.handle.clone())
    }
}
