// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListHsms`](crate::operation::list_hsms::builders::ListHsmsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_hsms::builders::ListHsmsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_hsms::builders::ListHsmsFluentBuilder::set_next_token):<br>required: **false**<br><p>The <code>NextToken</code> value from a previous call to <code>ListHsms</code>. Pass null if this is the first call.</p><br>
    /// - On success, responds with [`ListHsmsOutput`](crate::operation::list_hsms::ListHsmsOutput) with field(s):
    ///   - [`hsm_list(Option<Vec::<String>>)`](crate::operation::list_hsms::ListHsmsOutput::hsm_list): <p>The list of ARNs that identify the HSMs.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_hsms::ListHsmsOutput::next_token): <p>If not null, more results are available. Pass this value to <code>ListHsms</code> to retrieve the next set of items.</p>
    /// - On failure, responds with [`SdkError<ListHsmsError>`](crate::operation::list_hsms::ListHsmsError)
    #[deprecated(note = "This API is deprecated.")]
    pub fn list_hsms(&self) -> crate::operation::list_hsms::builders::ListHsmsFluentBuilder {
        crate::operation::list_hsms::builders::ListHsmsFluentBuilder::new(self.handle.clone())
    }
}
