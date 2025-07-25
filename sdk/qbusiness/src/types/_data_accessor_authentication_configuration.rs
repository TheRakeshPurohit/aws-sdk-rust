// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A union type that contains the specific authentication configuration based on the authentication type selected.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub enum DataAccessorAuthenticationConfiguration {
    /// <p>Configuration for IAM Identity Center Trusted Token Issuer (TTI) authentication used when the authentication type is <code>AWS_IAM_IDC_TTI</code>.</p>
    IdcTrustedTokenIssuerConfiguration(crate::types::DataAccessorIdcTrustedTokenIssuerConfiguration),
    /// The `Unknown` variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.
    /// An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has not been updated.
    /// To investigate this, consider turning on debug logging to print the raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl DataAccessorAuthenticationConfiguration {
    #[allow(irrefutable_let_patterns)]
    /// Tries to convert the enum instance into [`IdcTrustedTokenIssuerConfiguration`](crate::types::DataAccessorAuthenticationConfiguration::IdcTrustedTokenIssuerConfiguration), extracting the inner [`DataAccessorIdcTrustedTokenIssuerConfiguration`](crate::types::DataAccessorIdcTrustedTokenIssuerConfiguration).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_idc_trusted_token_issuer_configuration(
        &self,
    ) -> ::std::result::Result<&crate::types::DataAccessorIdcTrustedTokenIssuerConfiguration, &Self> {
        if let DataAccessorAuthenticationConfiguration::IdcTrustedTokenIssuerConfiguration(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`IdcTrustedTokenIssuerConfiguration`](crate::types::DataAccessorAuthenticationConfiguration::IdcTrustedTokenIssuerConfiguration).
    pub fn is_idc_trusted_token_issuer_configuration(&self) -> bool {
        self.as_idc_trusted_token_issuer_configuration().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}
