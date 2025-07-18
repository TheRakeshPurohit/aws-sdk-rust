// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartAccountAssociationRefresh`](crate::operation::start_account_association_refresh::builders::StartAccountAssociationRefreshFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`account_association_id(impl Into<String>)`](crate::operation::start_account_association_refresh::builders::StartAccountAssociationRefreshFluentBuilder::account_association_id) / [`set_account_association_id(Option<String>)`](crate::operation::start_account_association_refresh::builders::StartAccountAssociationRefreshFluentBuilder::set_account_association_id):<br>required: **true**<br><p>The unique identifier of the account association to refresh.</p><br>
    /// - On success, responds with [`StartAccountAssociationRefreshOutput`](crate::operation::start_account_association_refresh::StartAccountAssociationRefreshOutput) with field(s):
    ///   - [`o_auth_authorization_url(String)`](crate::operation::start_account_association_refresh::StartAccountAssociationRefreshOutput::o_auth_authorization_url): <p>Third-party IoT platform OAuth authorization server URL with all required parameters to perform end-user authentication during the refresh process.</p>
    /// - On failure, responds with [`SdkError<StartAccountAssociationRefreshError>`](crate::operation::start_account_association_refresh::StartAccountAssociationRefreshError)
    pub fn start_account_association_refresh(
        &self,
    ) -> crate::operation::start_account_association_refresh::builders::StartAccountAssociationRefreshFluentBuilder {
        crate::operation::start_account_association_refresh::builders::StartAccountAssociationRefreshFluentBuilder::new(self.handle.clone())
    }
}
