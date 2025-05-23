// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribePartnerApp`](crate::operation::describe_partner_app::builders::DescribePartnerAppFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`arn(impl Into<String>)`](crate::operation::describe_partner_app::builders::DescribePartnerAppFluentBuilder::arn) / [`set_arn(Option<String>)`](crate::operation::describe_partner_app::builders::DescribePartnerAppFluentBuilder::set_arn):<br>required: **true**<br><p>The ARN of the SageMaker Partner AI App to describe.</p><br>
    /// - On success, responds with [`DescribePartnerAppOutput`](crate::operation::describe_partner_app::DescribePartnerAppOutput) with field(s):
    ///   - [`arn(Option<String>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::arn): <p>The ARN of the SageMaker Partner AI App that was described.</p>
    ///   - [`name(Option<String>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::name): <p>The name of the SageMaker Partner AI App.</p>
    ///   - [`r#type(Option<PartnerAppType>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::type): <p>The type of SageMaker Partner AI App. Must be one of the following: <code>lakera-guard</code>, <code>comet</code>, <code>deepchecks-llm-evaluation</code>, or <code>fiddler</code>.</p>
    ///   - [`status(Option<PartnerAppStatus>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::status): <p>The status of the SageMaker Partner AI App.</p>
    ///   - [`creation_time(Option<DateTime>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::creation_time): <p>The time that the SageMaker Partner AI App was created.</p>
    ///   - [`last_modified_time(Option<DateTime>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::last_modified_time): <p>The time that the SageMaker Partner AI App was last modified.</p>
    ///   - [`execution_role_arn(Option<String>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::execution_role_arn): <p>The ARN of the IAM role associated with the SageMaker Partner AI App.</p>
    ///   - [`kms_key_id(Option<String>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::kms_key_id): <p>The Amazon Web Services KMS customer managed key used to encrypt the data at rest associated with SageMaker Partner AI Apps.</p>
    ///   - [`base_url(Option<String>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::base_url): <p>The URL of the SageMaker Partner AI App that the Application SDK uses to support in-app calls for the user.</p>
    ///   - [`maintenance_config(Option<PartnerAppMaintenanceConfig>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::maintenance_config): <p>Maintenance configuration settings for the SageMaker Partner AI App.</p>
    ///   - [`tier(Option<String>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::tier): <p>The instance type and size of the cluster attached to the SageMaker Partner AI App.</p>
    ///   - [`version(Option<String>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::version): <p>The version of the SageMaker Partner AI App.</p>
    ///   - [`application_config(Option<PartnerAppConfig>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::application_config): <p>Configuration settings for the SageMaker Partner AI App.</p>
    ///   - [`auth_type(Option<PartnerAppAuthType>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::auth_type): <p>The authorization type that users use to access the SageMaker Partner AI App.</p>
    ///   - [`enable_iam_session_based_identity(Option<bool>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::enable_iam_session_based_identity): <p>When set to <code>TRUE</code>, the SageMaker Partner AI App sets the Amazon Web Services IAM session name or the authenticated IAM user as the identity of the SageMaker Partner AI App user.</p>
    ///   - [`error(Option<ErrorInfo>)`](crate::operation::describe_partner_app::DescribePartnerAppOutput::error): <p>This is an error field object that contains the error code and the reason for an operation failure.</p>
    /// - On failure, responds with [`SdkError<DescribePartnerAppError>`](crate::operation::describe_partner_app::DescribePartnerAppError)
    pub fn describe_partner_app(&self) -> crate::operation::describe_partner_app::builders::DescribePartnerAppFluentBuilder {
        crate::operation::describe_partner_app::builders::DescribePartnerAppFluentBuilder::new(self.handle.clone())
    }
}
