// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeContext`](crate::operation::describe_context::builders::DescribeContextFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`context_name(impl Into<String>)`](crate::operation::describe_context::builders::DescribeContextFluentBuilder::context_name) / [`set_context_name(Option<String>)`](crate::operation::describe_context::builders::DescribeContextFluentBuilder::set_context_name):<br>required: **true**<br><p>The name of the context to describe.</p><br>
    /// - On success, responds with [`DescribeContextOutput`](crate::operation::describe_context::DescribeContextOutput) with field(s):
    ///   - [`context_name(Option<String>)`](crate::operation::describe_context::DescribeContextOutput::context_name): <p>The name of the context.</p>
    ///   - [`context_arn(Option<String>)`](crate::operation::describe_context::DescribeContextOutput::context_arn): <p>The Amazon Resource Name (ARN) of the context.</p>
    ///   - [`source(Option<ContextSource>)`](crate::operation::describe_context::DescribeContextOutput::source): <p>The source of the context.</p>
    ///   - [`context_type(Option<String>)`](crate::operation::describe_context::DescribeContextOutput::context_type): <p>The type of the context.</p>
    ///   - [`description(Option<String>)`](crate::operation::describe_context::DescribeContextOutput::description): <p>The description of the context.</p>
    ///   - [`properties(Option<HashMap::<String, String>>)`](crate::operation::describe_context::DescribeContextOutput::properties): <p>A list of the context's properties.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::describe_context::DescribeContextOutput::creation_time): <p>When the context was created.</p>
    ///   - [`created_by(Option<UserContext>)`](crate::operation::describe_context::DescribeContextOutput::created_by): <p>Information about the user who created or modified a SageMaker resource.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::operation::describe_context::DescribeContextOutput::last_modified_time): <p>When the context was last modified.</p>
    ///   - [`last_modified_by(Option<UserContext>)`](crate::operation::describe_context::DescribeContextOutput::last_modified_by): <p>Information about the user who created or modified a SageMaker resource.</p>
    ///   - [`lineage_group_arn(Option<String>)`](crate::operation::describe_context::DescribeContextOutput::lineage_group_arn): <p>The Amazon Resource Name (ARN) of the lineage group.</p>
    /// - On failure, responds with [`SdkError<DescribeContextError>`](crate::operation::describe_context::DescribeContextError)
    pub fn describe_context(&self) -> crate::operation::describe_context::builders::DescribeContextFluentBuilder {
        crate::operation::describe_context::builders::DescribeContextFluentBuilder::new(self.handle.clone())
    }
}
