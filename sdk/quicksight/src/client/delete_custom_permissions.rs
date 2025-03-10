// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteCustomPermissions`](crate::operation::delete_custom_permissions::builders::DeleteCustomPermissionsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`aws_account_id(impl Into<String>)`](crate::operation::delete_custom_permissions::builders::DeleteCustomPermissionsFluentBuilder::aws_account_id) / [`set_aws_account_id(Option<String>)`](crate::operation::delete_custom_permissions::builders::DeleteCustomPermissionsFluentBuilder::set_aws_account_id):<br>required: **true**<br><p>The ID of the Amazon Web Services account that contains the custom permissions profile that you want to delete.</p><br>
    ///   - [`custom_permissions_name(impl Into<String>)`](crate::operation::delete_custom_permissions::builders::DeleteCustomPermissionsFluentBuilder::custom_permissions_name) / [`set_custom_permissions_name(Option<String>)`](crate::operation::delete_custom_permissions::builders::DeleteCustomPermissionsFluentBuilder::set_custom_permissions_name):<br>required: **true**<br><p>The name of the custom permissions profile that you want to delete.</p><br>
    /// - On success, responds with [`DeleteCustomPermissionsOutput`](crate::operation::delete_custom_permissions::DeleteCustomPermissionsOutput) with field(s):
    ///   - [`status(i32)`](crate::operation::delete_custom_permissions::DeleteCustomPermissionsOutput::status): <p>The HTTP status of the request.</p>
    ///   - [`arn(Option<String>)`](crate::operation::delete_custom_permissions::DeleteCustomPermissionsOutput::arn): <p>The Amazon Resource Name (ARN) of the custom permissions profile.</p>
    ///   - [`request_id(Option<String>)`](crate::operation::delete_custom_permissions::DeleteCustomPermissionsOutput::request_id): <p>The Amazon Web Services request ID for this operation.</p>
    /// - On failure, responds with [`SdkError<DeleteCustomPermissionsError>`](crate::operation::delete_custom_permissions::DeleteCustomPermissionsError)
    pub fn delete_custom_permissions(&self) -> crate::operation::delete_custom_permissions::builders::DeleteCustomPermissionsFluentBuilder {
        crate::operation::delete_custom_permissions::builders::DeleteCustomPermissionsFluentBuilder::new(self.handle.clone())
    }
}
