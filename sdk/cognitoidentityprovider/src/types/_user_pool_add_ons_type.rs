// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>User pool add-ons. Contains settings for activation of advanced security features. To log user security information but take no action, set to <code>AUDIT</code>. To configure automatic security responses to risky traffic to your user pool, set to <code>ENFORCED</code>.</p>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/cognito/latest/developerguide/cognito-user-pool-settings-advanced-security.html">Adding advanced security to a user pool</a>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UserPoolAddOnsType {
    /// <p>The operating mode of advanced security features in your user pool.</p>
    pub advanced_security_mode: crate::types::AdvancedSecurityModeType,
}
impl UserPoolAddOnsType {
    /// <p>The operating mode of advanced security features in your user pool.</p>
    pub fn advanced_security_mode(&self) -> &crate::types::AdvancedSecurityModeType {
        &self.advanced_security_mode
    }
}
impl UserPoolAddOnsType {
    /// Creates a new builder-style object to manufacture [`UserPoolAddOnsType`](crate::types::UserPoolAddOnsType).
    pub fn builder() -> crate::types::builders::UserPoolAddOnsTypeBuilder {
        crate::types::builders::UserPoolAddOnsTypeBuilder::default()
    }
}

/// A builder for [`UserPoolAddOnsType`](crate::types::UserPoolAddOnsType).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UserPoolAddOnsTypeBuilder {
    pub(crate) advanced_security_mode: ::std::option::Option<crate::types::AdvancedSecurityModeType>,
}
impl UserPoolAddOnsTypeBuilder {
    /// <p>The operating mode of advanced security features in your user pool.</p>
    /// This field is required.
    pub fn advanced_security_mode(mut self, input: crate::types::AdvancedSecurityModeType) -> Self {
        self.advanced_security_mode = ::std::option::Option::Some(input);
        self
    }
    /// <p>The operating mode of advanced security features in your user pool.</p>
    pub fn set_advanced_security_mode(mut self, input: ::std::option::Option<crate::types::AdvancedSecurityModeType>) -> Self {
        self.advanced_security_mode = input;
        self
    }
    /// <p>The operating mode of advanced security features in your user pool.</p>
    pub fn get_advanced_security_mode(&self) -> &::std::option::Option<crate::types::AdvancedSecurityModeType> {
        &self.advanced_security_mode
    }
    /// Consumes the builder and constructs a [`UserPoolAddOnsType`](crate::types::UserPoolAddOnsType).
    /// This method will fail if any of the following fields are not set:
    /// - [`advanced_security_mode`](crate::types::builders::UserPoolAddOnsTypeBuilder::advanced_security_mode)
    pub fn build(self) -> ::std::result::Result<crate::types::UserPoolAddOnsType, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::UserPoolAddOnsType {
            advanced_security_mode: self.advanced_security_mode.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "advanced_security_mode",
                    "advanced_security_mode was not specified but it is required when building UserPoolAddOnsType",
                )
            })?,
        })
    }
}
