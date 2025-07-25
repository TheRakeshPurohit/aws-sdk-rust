// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SetTokenVaultCmkOutput {
    /// <p>The ID of the token vault.</p>
    pub token_vault_id: ::std::string::String,
    /// <p>The KMS configuration for the token vault.</p>
    pub kms_configuration: ::std::option::Option<crate::types::KmsConfiguration>,
    /// <p>The timestamp when the token vault was last modified.</p>
    pub last_modified_date: ::aws_smithy_types::DateTime,
    _request_id: Option<String>,
}
impl SetTokenVaultCmkOutput {
    /// <p>The ID of the token vault.</p>
    pub fn token_vault_id(&self) -> &str {
        use std::ops::Deref;
        self.token_vault_id.deref()
    }
    /// <p>The KMS configuration for the token vault.</p>
    pub fn kms_configuration(&self) -> ::std::option::Option<&crate::types::KmsConfiguration> {
        self.kms_configuration.as_ref()
    }
    /// <p>The timestamp when the token vault was last modified.</p>
    pub fn last_modified_date(&self) -> &::aws_smithy_types::DateTime {
        &self.last_modified_date
    }
}
impl ::aws_types::request_id::RequestId for SetTokenVaultCmkOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl SetTokenVaultCmkOutput {
    /// Creates a new builder-style object to manufacture [`SetTokenVaultCmkOutput`](crate::operation::set_token_vault_cmk::SetTokenVaultCmkOutput).
    pub fn builder() -> crate::operation::set_token_vault_cmk::builders::SetTokenVaultCmkOutputBuilder {
        crate::operation::set_token_vault_cmk::builders::SetTokenVaultCmkOutputBuilder::default()
    }
}

/// A builder for [`SetTokenVaultCmkOutput`](crate::operation::set_token_vault_cmk::SetTokenVaultCmkOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SetTokenVaultCmkOutputBuilder {
    pub(crate) token_vault_id: ::std::option::Option<::std::string::String>,
    pub(crate) kms_configuration: ::std::option::Option<crate::types::KmsConfiguration>,
    pub(crate) last_modified_date: ::std::option::Option<::aws_smithy_types::DateTime>,
    _request_id: Option<String>,
}
impl SetTokenVaultCmkOutputBuilder {
    /// <p>The ID of the token vault.</p>
    /// This field is required.
    pub fn token_vault_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.token_vault_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the token vault.</p>
    pub fn set_token_vault_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.token_vault_id = input;
        self
    }
    /// <p>The ID of the token vault.</p>
    pub fn get_token_vault_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.token_vault_id
    }
    /// <p>The KMS configuration for the token vault.</p>
    /// This field is required.
    pub fn kms_configuration(mut self, input: crate::types::KmsConfiguration) -> Self {
        self.kms_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The KMS configuration for the token vault.</p>
    pub fn set_kms_configuration(mut self, input: ::std::option::Option<crate::types::KmsConfiguration>) -> Self {
        self.kms_configuration = input;
        self
    }
    /// <p>The KMS configuration for the token vault.</p>
    pub fn get_kms_configuration(&self) -> &::std::option::Option<crate::types::KmsConfiguration> {
        &self.kms_configuration
    }
    /// <p>The timestamp when the token vault was last modified.</p>
    /// This field is required.
    pub fn last_modified_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.last_modified_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The timestamp when the token vault was last modified.</p>
    pub fn set_last_modified_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.last_modified_date = input;
        self
    }
    /// <p>The timestamp when the token vault was last modified.</p>
    pub fn get_last_modified_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.last_modified_date
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`SetTokenVaultCmkOutput`](crate::operation::set_token_vault_cmk::SetTokenVaultCmkOutput).
    /// This method will fail if any of the following fields are not set:
    /// - [`token_vault_id`](crate::operation::set_token_vault_cmk::builders::SetTokenVaultCmkOutputBuilder::token_vault_id)
    /// - [`last_modified_date`](crate::operation::set_token_vault_cmk::builders::SetTokenVaultCmkOutputBuilder::last_modified_date)
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::set_token_vault_cmk::SetTokenVaultCmkOutput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::set_token_vault_cmk::SetTokenVaultCmkOutput {
            token_vault_id: self.token_vault_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "token_vault_id",
                    "token_vault_id was not specified but it is required when building SetTokenVaultCmkOutput",
                )
            })?,
            kms_configuration: self.kms_configuration,
            last_modified_date: self.last_modified_date.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "last_modified_date",
                    "last_modified_date was not specified but it is required when building SetTokenVaultCmkOutput",
                )
            })?,
            _request_id: self._request_id,
        })
    }
}
