// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// An AWS Chatbot configuration for Microsoft Teams.
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct TeamsChannelConfiguration {
    /// The ID of the Microsoft Teams channel.
    pub channel_id: ::std::string::String,
    /// The name of the Microsoft Teams channel.
    pub channel_name: ::std::option::Option<::std::string::String>,
    /// The ID of the Microsoft Team authorized with AWS Chatbot. To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console. For more details, see steps 1-4 in Get started with Microsoft Teams in the AWS Chatbot Administrator Guide.
    pub team_id: ::std::string::String,
    /// The name of the Microsoft Teams Team.
    pub team_name: ::std::option::Option<::std::string::String>,
    /// The ID of the Microsoft Teams tenant.
    pub tenant_id: ::std::string::String,
    /// The ARN of the MicrosoftTeamsChannelConfiguration.
    pub chat_configuration_arn: ::std::string::String,
    /// The ARN of the IAM role that defines the permissions for AWS Chatbot. This is a user-defined role that AWS Chatbot will assume. This is not the service-linked role. For more information, see IAM Policies for AWS Chatbot.
    pub iam_role_arn: ::std::string::String,
    /// The ARNs of the SNS topics that deliver notifications to AWS Chatbot.
    pub sns_topic_arns: ::std::vec::Vec<::std::string::String>,
    /// The name of the configuration.
    pub configuration_name: ::std::option::Option<::std::string::String>,
    /// Logging levels include ERROR, INFO, or NONE.
    pub logging_level: ::std::option::Option<::std::string::String>,
    /// The list of IAM policy ARNs that are applied as channel guardrails. The AWS managed 'AdministratorAccess' policy is applied by default if this is not set.
    pub guardrail_policy_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    /// Enables use of a user role requirement in your chat configuration.
    pub user_authorization_required: ::std::option::Option<bool>,
    /// A list of tags applied to the configuration.
    pub tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl TeamsChannelConfiguration {
    /// The ID of the Microsoft Teams channel.
    pub fn channel_id(&self) -> &str {
        use std::ops::Deref;
        self.channel_id.deref()
    }
    /// The name of the Microsoft Teams channel.
    pub fn channel_name(&self) -> ::std::option::Option<&str> {
        self.channel_name.as_deref()
    }
    /// The ID of the Microsoft Team authorized with AWS Chatbot. To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console. For more details, see steps 1-4 in Get started with Microsoft Teams in the AWS Chatbot Administrator Guide.
    pub fn team_id(&self) -> &str {
        use std::ops::Deref;
        self.team_id.deref()
    }
    /// The name of the Microsoft Teams Team.
    pub fn team_name(&self) -> ::std::option::Option<&str> {
        self.team_name.as_deref()
    }
    /// The ID of the Microsoft Teams tenant.
    pub fn tenant_id(&self) -> &str {
        use std::ops::Deref;
        self.tenant_id.deref()
    }
    /// The ARN of the MicrosoftTeamsChannelConfiguration.
    pub fn chat_configuration_arn(&self) -> &str {
        use std::ops::Deref;
        self.chat_configuration_arn.deref()
    }
    /// The ARN of the IAM role that defines the permissions for AWS Chatbot. This is a user-defined role that AWS Chatbot will assume. This is not the service-linked role. For more information, see IAM Policies for AWS Chatbot.
    pub fn iam_role_arn(&self) -> &str {
        use std::ops::Deref;
        self.iam_role_arn.deref()
    }
    /// The ARNs of the SNS topics that deliver notifications to AWS Chatbot.
    pub fn sns_topic_arns(&self) -> &[::std::string::String] {
        use std::ops::Deref;
        self.sns_topic_arns.deref()
    }
    /// The name of the configuration.
    pub fn configuration_name(&self) -> ::std::option::Option<&str> {
        self.configuration_name.as_deref()
    }
    /// Logging levels include ERROR, INFO, or NONE.
    pub fn logging_level(&self) -> ::std::option::Option<&str> {
        self.logging_level.as_deref()
    }
    /// The list of IAM policy ARNs that are applied as channel guardrails. The AWS managed 'AdministratorAccess' policy is applied by default if this is not set.
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.guardrail_policy_arns.is_none()`.
    pub fn guardrail_policy_arns(&self) -> &[::std::string::String] {
        self.guardrail_policy_arns.as_deref().unwrap_or_default()
    }
    /// Enables use of a user role requirement in your chat configuration.
    pub fn user_authorization_required(&self) -> ::std::option::Option<bool> {
        self.user_authorization_required
    }
    /// A list of tags applied to the configuration.
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.tags.is_none()`.
    pub fn tags(&self) -> &[crate::types::Tag] {
        self.tags.as_deref().unwrap_or_default()
    }
}
impl TeamsChannelConfiguration {
    /// Creates a new builder-style object to manufacture [`TeamsChannelConfiguration`](crate::types::TeamsChannelConfiguration).
    pub fn builder() -> crate::types::builders::TeamsChannelConfigurationBuilder {
        crate::types::builders::TeamsChannelConfigurationBuilder::default()
    }
}

/// A builder for [`TeamsChannelConfiguration`](crate::types::TeamsChannelConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct TeamsChannelConfigurationBuilder {
    pub(crate) channel_id: ::std::option::Option<::std::string::String>,
    pub(crate) channel_name: ::std::option::Option<::std::string::String>,
    pub(crate) team_id: ::std::option::Option<::std::string::String>,
    pub(crate) team_name: ::std::option::Option<::std::string::String>,
    pub(crate) tenant_id: ::std::option::Option<::std::string::String>,
    pub(crate) chat_configuration_arn: ::std::option::Option<::std::string::String>,
    pub(crate) iam_role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) sns_topic_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) configuration_name: ::std::option::Option<::std::string::String>,
    pub(crate) logging_level: ::std::option::Option<::std::string::String>,
    pub(crate) guardrail_policy_arns: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub(crate) user_authorization_required: ::std::option::Option<bool>,
    pub(crate) tags: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>,
}
impl TeamsChannelConfigurationBuilder {
    /// The ID of the Microsoft Teams channel.
    /// This field is required.
    pub fn channel_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_id = ::std::option::Option::Some(input.into());
        self
    }
    /// The ID of the Microsoft Teams channel.
    pub fn set_channel_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_id = input;
        self
    }
    /// The ID of the Microsoft Teams channel.
    pub fn get_channel_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.channel_id
    }
    /// The name of the Microsoft Teams channel.
    pub fn channel_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_name = ::std::option::Option::Some(input.into());
        self
    }
    /// The name of the Microsoft Teams channel.
    pub fn set_channel_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_name = input;
        self
    }
    /// The name of the Microsoft Teams channel.
    pub fn get_channel_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.channel_name
    }
    /// The ID of the Microsoft Team authorized with AWS Chatbot. To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console. For more details, see steps 1-4 in Get started with Microsoft Teams in the AWS Chatbot Administrator Guide.
    /// This field is required.
    pub fn team_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.team_id = ::std::option::Option::Some(input.into());
        self
    }
    /// The ID of the Microsoft Team authorized with AWS Chatbot. To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console. For more details, see steps 1-4 in Get started with Microsoft Teams in the AWS Chatbot Administrator Guide.
    pub fn set_team_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.team_id = input;
        self
    }
    /// The ID of the Microsoft Team authorized with AWS Chatbot. To get the team ID, you must perform the initial authorization flow with Microsoft Teams in the AWS Chatbot console. Then you can copy and paste the team ID from the console. For more details, see steps 1-4 in Get started with Microsoft Teams in the AWS Chatbot Administrator Guide.
    pub fn get_team_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.team_id
    }
    /// The name of the Microsoft Teams Team.
    pub fn team_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.team_name = ::std::option::Option::Some(input.into());
        self
    }
    /// The name of the Microsoft Teams Team.
    pub fn set_team_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.team_name = input;
        self
    }
    /// The name of the Microsoft Teams Team.
    pub fn get_team_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.team_name
    }
    /// The ID of the Microsoft Teams tenant.
    /// This field is required.
    pub fn tenant_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.tenant_id = ::std::option::Option::Some(input.into());
        self
    }
    /// The ID of the Microsoft Teams tenant.
    pub fn set_tenant_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.tenant_id = input;
        self
    }
    /// The ID of the Microsoft Teams tenant.
    pub fn get_tenant_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.tenant_id
    }
    /// The ARN of the MicrosoftTeamsChannelConfiguration.
    /// This field is required.
    pub fn chat_configuration_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.chat_configuration_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// The ARN of the MicrosoftTeamsChannelConfiguration.
    pub fn set_chat_configuration_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.chat_configuration_arn = input;
        self
    }
    /// The ARN of the MicrosoftTeamsChannelConfiguration.
    pub fn get_chat_configuration_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.chat_configuration_arn
    }
    /// The ARN of the IAM role that defines the permissions for AWS Chatbot. This is a user-defined role that AWS Chatbot will assume. This is not the service-linked role. For more information, see IAM Policies for AWS Chatbot.
    /// This field is required.
    pub fn iam_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.iam_role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// The ARN of the IAM role that defines the permissions for AWS Chatbot. This is a user-defined role that AWS Chatbot will assume. This is not the service-linked role. For more information, see IAM Policies for AWS Chatbot.
    pub fn set_iam_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.iam_role_arn = input;
        self
    }
    /// The ARN of the IAM role that defines the permissions for AWS Chatbot. This is a user-defined role that AWS Chatbot will assume. This is not the service-linked role. For more information, see IAM Policies for AWS Chatbot.
    pub fn get_iam_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.iam_role_arn
    }
    /// Appends an item to `sns_topic_arns`.
    ///
    /// To override the contents of this collection use [`set_sns_topic_arns`](Self::set_sns_topic_arns).
    ///
    /// The ARNs of the SNS topics that deliver notifications to AWS Chatbot.
    pub fn sns_topic_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.sns_topic_arns.unwrap_or_default();
        v.push(input.into());
        self.sns_topic_arns = ::std::option::Option::Some(v);
        self
    }
    /// The ARNs of the SNS topics that deliver notifications to AWS Chatbot.
    pub fn set_sns_topic_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.sns_topic_arns = input;
        self
    }
    /// The ARNs of the SNS topics that deliver notifications to AWS Chatbot.
    pub fn get_sns_topic_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.sns_topic_arns
    }
    /// The name of the configuration.
    pub fn configuration_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.configuration_name = ::std::option::Option::Some(input.into());
        self
    }
    /// The name of the configuration.
    pub fn set_configuration_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.configuration_name = input;
        self
    }
    /// The name of the configuration.
    pub fn get_configuration_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.configuration_name
    }
    /// Logging levels include ERROR, INFO, or NONE.
    pub fn logging_level(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.logging_level = ::std::option::Option::Some(input.into());
        self
    }
    /// Logging levels include ERROR, INFO, or NONE.
    pub fn set_logging_level(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.logging_level = input;
        self
    }
    /// Logging levels include ERROR, INFO, or NONE.
    pub fn get_logging_level(&self) -> &::std::option::Option<::std::string::String> {
        &self.logging_level
    }
    /// Appends an item to `guardrail_policy_arns`.
    ///
    /// To override the contents of this collection use [`set_guardrail_policy_arns`](Self::set_guardrail_policy_arns).
    ///
    /// The list of IAM policy ARNs that are applied as channel guardrails. The AWS managed 'AdministratorAccess' policy is applied by default if this is not set.
    pub fn guardrail_policy_arns(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        let mut v = self.guardrail_policy_arns.unwrap_or_default();
        v.push(input.into());
        self.guardrail_policy_arns = ::std::option::Option::Some(v);
        self
    }
    /// The list of IAM policy ARNs that are applied as channel guardrails. The AWS managed 'AdministratorAccess' policy is applied by default if this is not set.
    pub fn set_guardrail_policy_arns(mut self, input: ::std::option::Option<::std::vec::Vec<::std::string::String>>) -> Self {
        self.guardrail_policy_arns = input;
        self
    }
    /// The list of IAM policy ARNs that are applied as channel guardrails. The AWS managed 'AdministratorAccess' policy is applied by default if this is not set.
    pub fn get_guardrail_policy_arns(&self) -> &::std::option::Option<::std::vec::Vec<::std::string::String>> {
        &self.guardrail_policy_arns
    }
    /// Enables use of a user role requirement in your chat configuration.
    pub fn user_authorization_required(mut self, input: bool) -> Self {
        self.user_authorization_required = ::std::option::Option::Some(input);
        self
    }
    /// Enables use of a user role requirement in your chat configuration.
    pub fn set_user_authorization_required(mut self, input: ::std::option::Option<bool>) -> Self {
        self.user_authorization_required = input;
        self
    }
    /// Enables use of a user role requirement in your chat configuration.
    pub fn get_user_authorization_required(&self) -> &::std::option::Option<bool> {
        &self.user_authorization_required
    }
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// A list of tags applied to the configuration.
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        let mut v = self.tags.unwrap_or_default();
        v.push(input);
        self.tags = ::std::option::Option::Some(v);
        self
    }
    /// A list of tags applied to the configuration.
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.tags = input;
        self
    }
    /// A list of tags applied to the configuration.
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        &self.tags
    }
    /// Consumes the builder and constructs a [`TeamsChannelConfiguration`](crate::types::TeamsChannelConfiguration).
    /// This method will fail if any of the following fields are not set:
    /// - [`channel_id`](crate::types::builders::TeamsChannelConfigurationBuilder::channel_id)
    /// - [`team_id`](crate::types::builders::TeamsChannelConfigurationBuilder::team_id)
    /// - [`tenant_id`](crate::types::builders::TeamsChannelConfigurationBuilder::tenant_id)
    /// - [`chat_configuration_arn`](crate::types::builders::TeamsChannelConfigurationBuilder::chat_configuration_arn)
    /// - [`iam_role_arn`](crate::types::builders::TeamsChannelConfigurationBuilder::iam_role_arn)
    /// - [`sns_topic_arns`](crate::types::builders::TeamsChannelConfigurationBuilder::sns_topic_arns)
    pub fn build(self) -> ::std::result::Result<crate::types::TeamsChannelConfiguration, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::TeamsChannelConfiguration {
            channel_id: self.channel_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "channel_id",
                    "channel_id was not specified but it is required when building TeamsChannelConfiguration",
                )
            })?,
            channel_name: self.channel_name,
            team_id: self.team_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "team_id",
                    "team_id was not specified but it is required when building TeamsChannelConfiguration",
                )
            })?,
            team_name: self.team_name,
            tenant_id: self.tenant_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "tenant_id",
                    "tenant_id was not specified but it is required when building TeamsChannelConfiguration",
                )
            })?,
            chat_configuration_arn: self.chat_configuration_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "chat_configuration_arn",
                    "chat_configuration_arn was not specified but it is required when building TeamsChannelConfiguration",
                )
            })?,
            iam_role_arn: self.iam_role_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "iam_role_arn",
                    "iam_role_arn was not specified but it is required when building TeamsChannelConfiguration",
                )
            })?,
            sns_topic_arns: self.sns_topic_arns.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "sns_topic_arns",
                    "sns_topic_arns was not specified but it is required when building TeamsChannelConfiguration",
                )
            })?,
            configuration_name: self.configuration_name,
            logging_level: self.logging_level,
            guardrail_policy_arns: self.guardrail_policy_arns,
            user_authorization_required: self.user_authorization_required,
            tags: self.tags,
        })
    }
}
