// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DisableAWSOrganizationsAccess`](crate::operation::disable_aws_organizations_access::builders::DisableAWSOrganizationsAccessFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::disable_aws_organizations_access::builders::DisableAWSOrganizationsAccessFluentBuilder::send) it.
    /// - On success, responds with [`DisableAwsOrganizationsAccessOutput`](crate::operation::disable_aws_organizations_access::DisableAwsOrganizationsAccessOutput)
    /// - On failure, responds with [`SdkError<DisableAWSOrganizationsAccessError>`](crate::operation::disable_aws_organizations_access::DisableAWSOrganizationsAccessError)
    pub fn disable_aws_organizations_access(
        &self,
    ) -> crate::operation::disable_aws_organizations_access::builders::DisableAWSOrganizationsAccessFluentBuilder {
        crate::operation::disable_aws_organizations_access::builders::DisableAWSOrganizationsAccessFluentBuilder::new(self.handle.clone())
    }
}
