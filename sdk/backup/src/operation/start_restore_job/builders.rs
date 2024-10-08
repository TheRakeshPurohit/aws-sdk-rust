// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_restore_job::_start_restore_job_output::StartRestoreJobOutputBuilder;

pub use crate::operation::start_restore_job::_start_restore_job_input::StartRestoreJobInputBuilder;

impl crate::operation::start_restore_job::builders::StartRestoreJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_restore_job::StartRestoreJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_restore_job::StartRestoreJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_restore_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartRestoreJob`.
///
/// <p>Recovers the saved resource identified by an Amazon Resource Name (ARN).</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartRestoreJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_restore_job::builders::StartRestoreJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_restore_job::StartRestoreJobOutput,
        crate::operation::start_restore_job::StartRestoreJobError,
    > for StartRestoreJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_restore_job::StartRestoreJobOutput,
            crate::operation::start_restore_job::StartRestoreJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartRestoreJobFluentBuilder {
    /// Creates a new `StartRestoreJobFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartRestoreJob as a reference.
    pub fn as_input(&self) -> &crate::operation::start_restore_job::builders::StartRestoreJobInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::start_restore_job::StartRestoreJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_restore_job::StartRestoreJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_restore_job::StartRestoreJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_restore_job::StartRestoreJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_restore_job::StartRestoreJobOutput,
        crate::operation::start_restore_job::StartRestoreJobError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    pub fn recovery_point_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.recovery_point_arn(input.into());
        self
    }
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    pub fn set_recovery_point_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_recovery_point_arn(input);
        self
    }
    /// <p>An ARN that uniquely identifies a recovery point; for example, <code>arn:aws:backup:us-east-1:123456789012:recovery-point:1EB3B5E7-9EB0-435A-A80B-108B488B0D45</code>.</p>
    pub fn get_recovery_point_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_recovery_point_arn()
    }
    ///
    /// Adds a key-value pair to `Metadata`.
    ///
    /// To override the contents of this collection use [`set_metadata`](Self::set_metadata).
    ///
    /// <p>A set of metadata key-value pairs.</p>
    /// <p>You can get configuration metadata about a resource at the time it was backed up by calling <code>GetRecoveryPointRestoreMetadata</code>. However, values in addition to those provided by <code>GetRecoveryPointRestoreMetadata</code> might be required to restore a resource. For example, you might need to provide a new resource name if the original already exists.</p>
    /// <p>For more information about the metadata for each resource, see the following:</p>
    /// <ul>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-aur.html#aur-restore-cli">Metadata for Amazon Aurora</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-docdb.html#docdb-restore-cli">Metadata for Amazon DocumentDB</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restore-application-stacks.html#restoring-cfn-cli">Metadata for CloudFormation</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-dynamodb.html#ddb-restore-cli">Metadata for Amazon DynamoDB</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-ebs.html#ebs-restore-cli"> Metadata for Amazon EBS</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-ec2.html#restoring-ec2-cli">Metadata for Amazon EC2</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-efs.html#efs-restore-cli">Metadata for Amazon EFS</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-fsx.html#fsx-restore-cli">Metadata for Amazon FSx</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-nep.html#nep-restore-cli">Metadata for Amazon Neptune</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-rds.html#rds-restore-cli">Metadata for Amazon RDS</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/redshift-restores.html#redshift-restore-api">Metadata for Amazon Redshift</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-storage-gateway.html#restoring-sgw-cli">Metadata for Storage Gateway</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-s3.html#s3-restore-cli">Metadata for Amazon S3</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/timestream-restore.html#timestream-restore-api">Metadata for Amazon Timestream</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-vm.html#vm-restore-cli">Metadata for virtual machines</a></p></li>
    /// </ul>
    pub fn metadata(mut self, k: impl ::std::convert::Into<::std::string::String>, v: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.metadata(k.into(), v.into());
        self
    }
    /// <p>A set of metadata key-value pairs.</p>
    /// <p>You can get configuration metadata about a resource at the time it was backed up by calling <code>GetRecoveryPointRestoreMetadata</code>. However, values in addition to those provided by <code>GetRecoveryPointRestoreMetadata</code> might be required to restore a resource. For example, you might need to provide a new resource name if the original already exists.</p>
    /// <p>For more information about the metadata for each resource, see the following:</p>
    /// <ul>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-aur.html#aur-restore-cli">Metadata for Amazon Aurora</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-docdb.html#docdb-restore-cli">Metadata for Amazon DocumentDB</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restore-application-stacks.html#restoring-cfn-cli">Metadata for CloudFormation</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-dynamodb.html#ddb-restore-cli">Metadata for Amazon DynamoDB</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-ebs.html#ebs-restore-cli"> Metadata for Amazon EBS</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-ec2.html#restoring-ec2-cli">Metadata for Amazon EC2</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-efs.html#efs-restore-cli">Metadata for Amazon EFS</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-fsx.html#fsx-restore-cli">Metadata for Amazon FSx</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-nep.html#nep-restore-cli">Metadata for Amazon Neptune</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-rds.html#rds-restore-cli">Metadata for Amazon RDS</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/redshift-restores.html#redshift-restore-api">Metadata for Amazon Redshift</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-storage-gateway.html#restoring-sgw-cli">Metadata for Storage Gateway</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-s3.html#s3-restore-cli">Metadata for Amazon S3</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/timestream-restore.html#timestream-restore-api">Metadata for Amazon Timestream</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-vm.html#vm-restore-cli">Metadata for virtual machines</a></p></li>
    /// </ul>
    pub fn set_metadata(mut self, input: ::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>>) -> Self {
        self.inner = self.inner.set_metadata(input);
        self
    }
    /// <p>A set of metadata key-value pairs.</p>
    /// <p>You can get configuration metadata about a resource at the time it was backed up by calling <code>GetRecoveryPointRestoreMetadata</code>. However, values in addition to those provided by <code>GetRecoveryPointRestoreMetadata</code> might be required to restore a resource. For example, you might need to provide a new resource name if the original already exists.</p>
    /// <p>For more information about the metadata for each resource, see the following:</p>
    /// <ul>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-aur.html#aur-restore-cli">Metadata for Amazon Aurora</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-docdb.html#docdb-restore-cli">Metadata for Amazon DocumentDB</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restore-application-stacks.html#restoring-cfn-cli">Metadata for CloudFormation</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-dynamodb.html#ddb-restore-cli">Metadata for Amazon DynamoDB</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-ebs.html#ebs-restore-cli"> Metadata for Amazon EBS</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-ec2.html#restoring-ec2-cli">Metadata for Amazon EC2</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-efs.html#efs-restore-cli">Metadata for Amazon EFS</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-fsx.html#fsx-restore-cli">Metadata for Amazon FSx</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-nep.html#nep-restore-cli">Metadata for Amazon Neptune</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-rds.html#rds-restore-cli">Metadata for Amazon RDS</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/redshift-restores.html#redshift-restore-api">Metadata for Amazon Redshift</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-storage-gateway.html#restoring-sgw-cli">Metadata for Storage Gateway</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-s3.html#s3-restore-cli">Metadata for Amazon S3</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/timestream-restore.html#timestream-restore-api">Metadata for Amazon Timestream</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/aws-backup/latest/devguide/restoring-vm.html#vm-restore-cli">Metadata for virtual machines</a></p></li>
    /// </ul>
    pub fn get_metadata(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, ::std::string::String>> {
        self.inner.get_metadata()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role that Backup uses to create the target resource; for example: <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    pub fn iam_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.iam_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role that Backup uses to create the target resource; for example: <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    pub fn set_iam_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_iam_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role that Backup uses to create the target resource; for example: <code>arn:aws:iam::123456789012:role/S3Access</code>.</p>
    pub fn get_iam_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_iam_role_arn()
    }
    /// <p>A customer-chosen string that you can use to distinguish between otherwise identical calls to <code>StartRestoreJob</code>. Retrying a successful request with the same idempotency token results in a success message with no action taken.</p>
    pub fn idempotency_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.idempotency_token(input.into());
        self
    }
    /// <p>A customer-chosen string that you can use to distinguish between otherwise identical calls to <code>StartRestoreJob</code>. Retrying a successful request with the same idempotency token results in a success message with no action taken.</p>
    pub fn set_idempotency_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_idempotency_token(input);
        self
    }
    /// <p>A customer-chosen string that you can use to distinguish between otherwise identical calls to <code>StartRestoreJob</code>. Retrying a successful request with the same idempotency token results in a success message with no action taken.</p>
    pub fn get_idempotency_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_idempotency_token()
    }
    /// <p>Starts a job to restore a recovery point for one of the following resources:</p>
    /// <ul>
    /// <li>
    /// <p><code>Aurora</code> - Amazon Aurora</p></li>
    /// <li>
    /// <p><code>DocumentDB</code> - Amazon DocumentDB</p></li>
    /// <li>
    /// <p><code>CloudFormation</code> - CloudFormation</p></li>
    /// <li>
    /// <p><code>DynamoDB</code> - Amazon DynamoDB</p></li>
    /// <li>
    /// <p><code>EBS</code> - Amazon Elastic Block Store</p></li>
    /// <li>
    /// <p><code>EC2</code> - Amazon Elastic Compute Cloud</p></li>
    /// <li>
    /// <p><code>EFS</code> - Amazon Elastic File System</p></li>
    /// <li>
    /// <p><code>FSx</code> - Amazon FSx</p></li>
    /// <li>
    /// <p><code>Neptune</code> - Amazon Neptune</p></li>
    /// <li>
    /// <p><code>RDS</code> - Amazon Relational Database Service</p></li>
    /// <li>
    /// <p><code>Redshift</code> - Amazon Redshift</p></li>
    /// <li>
    /// <p><code>Storage Gateway</code> - Storage Gateway</p></li>
    /// <li>
    /// <p><code>S3</code> - Amazon Simple Storage Service</p></li>
    /// <li>
    /// <p><code>Timestream</code> - Amazon Timestream</p></li>
    /// <li>
    /// <p><code>VirtualMachine</code> - Virtual machines</p></li>
    /// </ul>
    pub fn resource_type(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.resource_type(input.into());
        self
    }
    /// <p>Starts a job to restore a recovery point for one of the following resources:</p>
    /// <ul>
    /// <li>
    /// <p><code>Aurora</code> - Amazon Aurora</p></li>
    /// <li>
    /// <p><code>DocumentDB</code> - Amazon DocumentDB</p></li>
    /// <li>
    /// <p><code>CloudFormation</code> - CloudFormation</p></li>
    /// <li>
    /// <p><code>DynamoDB</code> - Amazon DynamoDB</p></li>
    /// <li>
    /// <p><code>EBS</code> - Amazon Elastic Block Store</p></li>
    /// <li>
    /// <p><code>EC2</code> - Amazon Elastic Compute Cloud</p></li>
    /// <li>
    /// <p><code>EFS</code> - Amazon Elastic File System</p></li>
    /// <li>
    /// <p><code>FSx</code> - Amazon FSx</p></li>
    /// <li>
    /// <p><code>Neptune</code> - Amazon Neptune</p></li>
    /// <li>
    /// <p><code>RDS</code> - Amazon Relational Database Service</p></li>
    /// <li>
    /// <p><code>Redshift</code> - Amazon Redshift</p></li>
    /// <li>
    /// <p><code>Storage Gateway</code> - Storage Gateway</p></li>
    /// <li>
    /// <p><code>S3</code> - Amazon Simple Storage Service</p></li>
    /// <li>
    /// <p><code>Timestream</code> - Amazon Timestream</p></li>
    /// <li>
    /// <p><code>VirtualMachine</code> - Virtual machines</p></li>
    /// </ul>
    pub fn set_resource_type(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_resource_type(input);
        self
    }
    /// <p>Starts a job to restore a recovery point for one of the following resources:</p>
    /// <ul>
    /// <li>
    /// <p><code>Aurora</code> - Amazon Aurora</p></li>
    /// <li>
    /// <p><code>DocumentDB</code> - Amazon DocumentDB</p></li>
    /// <li>
    /// <p><code>CloudFormation</code> - CloudFormation</p></li>
    /// <li>
    /// <p><code>DynamoDB</code> - Amazon DynamoDB</p></li>
    /// <li>
    /// <p><code>EBS</code> - Amazon Elastic Block Store</p></li>
    /// <li>
    /// <p><code>EC2</code> - Amazon Elastic Compute Cloud</p></li>
    /// <li>
    /// <p><code>EFS</code> - Amazon Elastic File System</p></li>
    /// <li>
    /// <p><code>FSx</code> - Amazon FSx</p></li>
    /// <li>
    /// <p><code>Neptune</code> - Amazon Neptune</p></li>
    /// <li>
    /// <p><code>RDS</code> - Amazon Relational Database Service</p></li>
    /// <li>
    /// <p><code>Redshift</code> - Amazon Redshift</p></li>
    /// <li>
    /// <p><code>Storage Gateway</code> - Storage Gateway</p></li>
    /// <li>
    /// <p><code>S3</code> - Amazon Simple Storage Service</p></li>
    /// <li>
    /// <p><code>Timestream</code> - Amazon Timestream</p></li>
    /// <li>
    /// <p><code>VirtualMachine</code> - Virtual machines</p></li>
    /// </ul>
    pub fn get_resource_type(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_resource_type()
    }
    /// <p>This is an optional parameter. If this equals <code>True</code>, tags included in the backup will be copied to the restored resource.</p>
    /// <p>This can only be applied to backups created through Backup.</p>
    pub fn copy_source_tags_to_restored_resource(mut self, input: bool) -> Self {
        self.inner = self.inner.copy_source_tags_to_restored_resource(input);
        self
    }
    /// <p>This is an optional parameter. If this equals <code>True</code>, tags included in the backup will be copied to the restored resource.</p>
    /// <p>This can only be applied to backups created through Backup.</p>
    pub fn set_copy_source_tags_to_restored_resource(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_copy_source_tags_to_restored_resource(input);
        self
    }
    /// <p>This is an optional parameter. If this equals <code>True</code>, tags included in the backup will be copied to the restored resource.</p>
    /// <p>This can only be applied to backups created through Backup.</p>
    pub fn get_copy_source_tags_to_restored_resource(&self) -> &::std::option::Option<bool> {
        self.inner.get_copy_source_tags_to_restored_resource()
    }
}
