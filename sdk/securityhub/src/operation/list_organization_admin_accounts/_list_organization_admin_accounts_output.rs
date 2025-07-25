// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ListOrganizationAdminAccountsOutput {
    /// <p>The list of Security Hub administrator accounts.</p>
    pub admin_accounts: ::std::option::Option<::std::vec::Vec<crate::types::AdminAccount>>,
    /// <p>The pagination token to use to request the next page of results.</p>
    pub next_token: ::std::option::Option<::std::string::String>,
    /// <p>The feature where the delegated administrator account is listed. Defaults to Security Hub CSPM if not specified.</p>
    pub feature: ::std::option::Option<crate::types::SecurityHubFeature>,
    _request_id: Option<String>,
}
impl ListOrganizationAdminAccountsOutput {
    /// <p>The list of Security Hub administrator accounts.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.admin_accounts.is_none()`.
    pub fn admin_accounts(&self) -> &[crate::types::AdminAccount] {
        self.admin_accounts.as_deref().unwrap_or_default()
    }
    /// <p>The pagination token to use to request the next page of results.</p>
    pub fn next_token(&self) -> ::std::option::Option<&str> {
        self.next_token.as_deref()
    }
    /// <p>The feature where the delegated administrator account is listed. Defaults to Security Hub CSPM if not specified.</p>
    pub fn feature(&self) -> ::std::option::Option<&crate::types::SecurityHubFeature> {
        self.feature.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for ListOrganizationAdminAccountsOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl ListOrganizationAdminAccountsOutput {
    /// Creates a new builder-style object to manufacture [`ListOrganizationAdminAccountsOutput`](crate::operation::list_organization_admin_accounts::ListOrganizationAdminAccountsOutput).
    pub fn builder() -> crate::operation::list_organization_admin_accounts::builders::ListOrganizationAdminAccountsOutputBuilder {
        crate::operation::list_organization_admin_accounts::builders::ListOrganizationAdminAccountsOutputBuilder::default()
    }
}

/// A builder for [`ListOrganizationAdminAccountsOutput`](crate::operation::list_organization_admin_accounts::ListOrganizationAdminAccountsOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ListOrganizationAdminAccountsOutputBuilder {
    pub(crate) admin_accounts: ::std::option::Option<::std::vec::Vec<crate::types::AdminAccount>>,
    pub(crate) next_token: ::std::option::Option<::std::string::String>,
    pub(crate) feature: ::std::option::Option<crate::types::SecurityHubFeature>,
    _request_id: Option<String>,
}
impl ListOrganizationAdminAccountsOutputBuilder {
    /// Appends an item to `admin_accounts`.
    ///
    /// To override the contents of this collection use [`set_admin_accounts`](Self::set_admin_accounts).
    ///
    /// <p>The list of Security Hub administrator accounts.</p>
    pub fn admin_accounts(mut self, input: crate::types::AdminAccount) -> Self {
        let mut v = self.admin_accounts.unwrap_or_default();
        v.push(input);
        self.admin_accounts = ::std::option::Option::Some(v);
        self
    }
    /// <p>The list of Security Hub administrator accounts.</p>
    pub fn set_admin_accounts(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::AdminAccount>>) -> Self {
        self.admin_accounts = input;
        self
    }
    /// <p>The list of Security Hub administrator accounts.</p>
    pub fn get_admin_accounts(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::AdminAccount>> {
        &self.admin_accounts
    }
    /// <p>The pagination token to use to request the next page of results.</p>
    pub fn next_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.next_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The pagination token to use to request the next page of results.</p>
    pub fn set_next_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.next_token = input;
        self
    }
    /// <p>The pagination token to use to request the next page of results.</p>
    pub fn get_next_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.next_token
    }
    /// <p>The feature where the delegated administrator account is listed. Defaults to Security Hub CSPM if not specified.</p>
    pub fn feature(mut self, input: crate::types::SecurityHubFeature) -> Self {
        self.feature = ::std::option::Option::Some(input);
        self
    }
    /// <p>The feature where the delegated administrator account is listed. Defaults to Security Hub CSPM if not specified.</p>
    pub fn set_feature(mut self, input: ::std::option::Option<crate::types::SecurityHubFeature>) -> Self {
        self.feature = input;
        self
    }
    /// <p>The feature where the delegated administrator account is listed. Defaults to Security Hub CSPM if not specified.</p>
    pub fn get_feature(&self) -> &::std::option::Option<crate::types::SecurityHubFeature> {
        &self.feature
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`ListOrganizationAdminAccountsOutput`](crate::operation::list_organization_admin_accounts::ListOrganizationAdminAccountsOutput).
    pub fn build(self) -> crate::operation::list_organization_admin_accounts::ListOrganizationAdminAccountsOutput {
        crate::operation::list_organization_admin_accounts::ListOrganizationAdminAccountsOutput {
            admin_accounts: self.admin_accounts,
            next_token: self.next_token,
            feature: self.feature,
            _request_id: self._request_id,
        }
    }
}
