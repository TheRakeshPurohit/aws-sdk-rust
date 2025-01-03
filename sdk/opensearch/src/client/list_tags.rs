// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListTags`](crate::operation::list_tags::builders::ListTagsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::list_tags::builders::ListTagsFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::list_tags::builders::ListTagsFluentBuilder::set_arn):<br>required: **true**<br><p>Amazon Resource Name (ARN) for the domain, data source, or application to view tags for.</p><br>
    /// - On success, responds with [`ListTagsOutput`](crate::operation::list_tags::ListTagsOutput) with field(s):
    ///   - [`tag_list(Option<Vec::<Tag>>)`](crate::operation::list_tags::ListTagsOutput::tag_list): <p>List of resource tags associated with the specified domain, data source, or application.</p>
    /// - On failure, responds with [`SdkError<ListTagsError>`](crate::operation::list_tags::ListTagsError)
    pub fn list_tags(&self) -> crate::operation::list_tags::builders::ListTagsFluentBuilder {
        crate::operation::list_tags::builders::ListTagsFluentBuilder::new(self.handle.clone())
    }
}
