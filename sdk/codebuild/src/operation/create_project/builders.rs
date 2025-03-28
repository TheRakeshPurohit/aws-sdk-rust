// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_project::_create_project_output::CreateProjectOutputBuilder;

pub use crate::operation::create_project::_create_project_input::CreateProjectInputBuilder;

impl crate::operation::create_project::builders::CreateProjectInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_project::CreateProjectOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_project::CreateProjectError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_project();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateProject`.
///
/// <p>Creates a build project.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateProjectFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_project::builders::CreateProjectInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_project::CreateProjectOutput,
        crate::operation::create_project::CreateProjectError,
    > for CreateProjectFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_project::CreateProjectOutput,
            crate::operation::create_project::CreateProjectError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateProjectFluentBuilder {
    /// Creates a new `CreateProjectFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateProject as a reference.
    pub fn as_input(&self) -> &crate::operation::create_project::builders::CreateProjectInputBuilder {
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
        crate::operation::create_project::CreateProjectOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_project::CreateProjectError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_project::CreateProject::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_project::CreateProject::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_project::CreateProjectOutput,
        crate::operation::create_project::CreateProjectError,
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
    /// <p>The name of the build project.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.name(input.into());
        self
    }
    /// <p>The name of the build project.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_name(input);
        self
    }
    /// <p>The name of the build project.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_name()
    }
    /// <p>A description that makes the build project easy to identify.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description that makes the build project easy to identify.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description that makes the build project easy to identify.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>Information about the build input source code for the build project.</p>
    pub fn source(mut self, input: crate::types::ProjectSource) -> Self {
        self.inner = self.inner.source(input);
        self
    }
    /// <p>Information about the build input source code for the build project.</p>
    pub fn set_source(mut self, input: ::std::option::Option<crate::types::ProjectSource>) -> Self {
        self.inner = self.inner.set_source(input);
        self
    }
    /// <p>Information about the build input source code for the build project.</p>
    pub fn get_source(&self) -> &::std::option::Option<crate::types::ProjectSource> {
        self.inner.get_source()
    }
    ///
    /// Appends an item to `secondarySources`.
    ///
    /// To override the contents of this collection use [`set_secondary_sources`](Self::set_secondary_sources).
    ///
    /// <p>An array of <code>ProjectSource</code> objects.</p>
    pub fn secondary_sources(mut self, input: crate::types::ProjectSource) -> Self {
        self.inner = self.inner.secondary_sources(input);
        self
    }
    /// <p>An array of <code>ProjectSource</code> objects.</p>
    pub fn set_secondary_sources(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ProjectSource>>) -> Self {
        self.inner = self.inner.set_secondary_sources(input);
        self
    }
    /// <p>An array of <code>ProjectSource</code> objects.</p>
    pub fn get_secondary_sources(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ProjectSource>> {
        self.inner.get_secondary_sources()
    }
    /// <p>A version of the build input to be built for this project. If not specified, the latest version is used. If specified, it must be one of:</p>
    /// <ul>
    /// <li>
    /// <p>For CodeCommit: the commit ID, branch, or Git tag to use.</p></li>
    /// <li>
    /// <p>For GitHub: the commit ID, pull request ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a pull request ID is specified, it must use the format <code>pr/pull-request-ID</code> (for example <code>pr/25</code>). If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p></li>
    /// <li>
    /// <p>For GitLab: the commit ID, branch, or Git tag to use.</p></li>
    /// <li>
    /// <p>For Bitbucket: the commit ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p></li>
    /// <li>
    /// <p>For Amazon S3: the version ID of the object that represents the build input ZIP file to use.</p></li>
    /// </ul>
    /// <p>If <code>sourceVersion</code> is specified at the build level, then that version takes precedence over this <code>sourceVersion</code> (at the project level).</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-source-version.html">Source Version Sample with CodeBuild</a> in the <i>CodeBuild User Guide</i>.</p>
    pub fn source_version(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.source_version(input.into());
        self
    }
    /// <p>A version of the build input to be built for this project. If not specified, the latest version is used. If specified, it must be one of:</p>
    /// <ul>
    /// <li>
    /// <p>For CodeCommit: the commit ID, branch, or Git tag to use.</p></li>
    /// <li>
    /// <p>For GitHub: the commit ID, pull request ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a pull request ID is specified, it must use the format <code>pr/pull-request-ID</code> (for example <code>pr/25</code>). If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p></li>
    /// <li>
    /// <p>For GitLab: the commit ID, branch, or Git tag to use.</p></li>
    /// <li>
    /// <p>For Bitbucket: the commit ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p></li>
    /// <li>
    /// <p>For Amazon S3: the version ID of the object that represents the build input ZIP file to use.</p></li>
    /// </ul>
    /// <p>If <code>sourceVersion</code> is specified at the build level, then that version takes precedence over this <code>sourceVersion</code> (at the project level).</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-source-version.html">Source Version Sample with CodeBuild</a> in the <i>CodeBuild User Guide</i>.</p>
    pub fn set_source_version(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_source_version(input);
        self
    }
    /// <p>A version of the build input to be built for this project. If not specified, the latest version is used. If specified, it must be one of:</p>
    /// <ul>
    /// <li>
    /// <p>For CodeCommit: the commit ID, branch, or Git tag to use.</p></li>
    /// <li>
    /// <p>For GitHub: the commit ID, pull request ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a pull request ID is specified, it must use the format <code>pr/pull-request-ID</code> (for example <code>pr/25</code>). If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p></li>
    /// <li>
    /// <p>For GitLab: the commit ID, branch, or Git tag to use.</p></li>
    /// <li>
    /// <p>For Bitbucket: the commit ID, branch name, or tag name that corresponds to the version of the source code you want to build. If a branch name is specified, the branch's HEAD commit ID is used. If not specified, the default branch's HEAD commit ID is used.</p></li>
    /// <li>
    /// <p>For Amazon S3: the version ID of the object that represents the build input ZIP file to use.</p></li>
    /// </ul>
    /// <p>If <code>sourceVersion</code> is specified at the build level, then that version takes precedence over this <code>sourceVersion</code> (at the project level).</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/codebuild/latest/userguide/sample-source-version.html">Source Version Sample with CodeBuild</a> in the <i>CodeBuild User Guide</i>.</p>
    pub fn get_source_version(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_source_version()
    }
    ///
    /// Appends an item to `secondarySourceVersions`.
    ///
    /// To override the contents of this collection use [`set_secondary_source_versions`](Self::set_secondary_source_versions).
    ///
    /// <p>An array of <code>ProjectSourceVersion</code> objects. If <code>secondarySourceVersions</code> is specified at the build level, then they take precedence over these <code>secondarySourceVersions</code> (at the project level).</p>
    pub fn secondary_source_versions(mut self, input: crate::types::ProjectSourceVersion) -> Self {
        self.inner = self.inner.secondary_source_versions(input);
        self
    }
    /// <p>An array of <code>ProjectSourceVersion</code> objects. If <code>secondarySourceVersions</code> is specified at the build level, then they take precedence over these <code>secondarySourceVersions</code> (at the project level).</p>
    pub fn set_secondary_source_versions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ProjectSourceVersion>>) -> Self {
        self.inner = self.inner.set_secondary_source_versions(input);
        self
    }
    /// <p>An array of <code>ProjectSourceVersion</code> objects. If <code>secondarySourceVersions</code> is specified at the build level, then they take precedence over these <code>secondarySourceVersions</code> (at the project level).</p>
    pub fn get_secondary_source_versions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ProjectSourceVersion>> {
        self.inner.get_secondary_source_versions()
    }
    /// <p>Information about the build output artifacts for the build project.</p>
    pub fn artifacts(mut self, input: crate::types::ProjectArtifacts) -> Self {
        self.inner = self.inner.artifacts(input);
        self
    }
    /// <p>Information about the build output artifacts for the build project.</p>
    pub fn set_artifacts(mut self, input: ::std::option::Option<crate::types::ProjectArtifacts>) -> Self {
        self.inner = self.inner.set_artifacts(input);
        self
    }
    /// <p>Information about the build output artifacts for the build project.</p>
    pub fn get_artifacts(&self) -> &::std::option::Option<crate::types::ProjectArtifacts> {
        self.inner.get_artifacts()
    }
    ///
    /// Appends an item to `secondaryArtifacts`.
    ///
    /// To override the contents of this collection use [`set_secondary_artifacts`](Self::set_secondary_artifacts).
    ///
    /// <p>An array of <code>ProjectArtifacts</code> objects.</p>
    pub fn secondary_artifacts(mut self, input: crate::types::ProjectArtifacts) -> Self {
        self.inner = self.inner.secondary_artifacts(input);
        self
    }
    /// <p>An array of <code>ProjectArtifacts</code> objects.</p>
    pub fn set_secondary_artifacts(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ProjectArtifacts>>) -> Self {
        self.inner = self.inner.set_secondary_artifacts(input);
        self
    }
    /// <p>An array of <code>ProjectArtifacts</code> objects.</p>
    pub fn get_secondary_artifacts(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ProjectArtifacts>> {
        self.inner.get_secondary_artifacts()
    }
    /// <p>Stores recently used information so that it can be quickly accessed at a later time.</p>
    pub fn cache(mut self, input: crate::types::ProjectCache) -> Self {
        self.inner = self.inner.cache(input);
        self
    }
    /// <p>Stores recently used information so that it can be quickly accessed at a later time.</p>
    pub fn set_cache(mut self, input: ::std::option::Option<crate::types::ProjectCache>) -> Self {
        self.inner = self.inner.set_cache(input);
        self
    }
    /// <p>Stores recently used information so that it can be quickly accessed at a later time.</p>
    pub fn get_cache(&self) -> &::std::option::Option<crate::types::ProjectCache> {
        self.inner.get_cache()
    }
    /// <p>Information about the build environment for the build project.</p>
    pub fn environment(mut self, input: crate::types::ProjectEnvironment) -> Self {
        self.inner = self.inner.environment(input);
        self
    }
    /// <p>Information about the build environment for the build project.</p>
    pub fn set_environment(mut self, input: ::std::option::Option<crate::types::ProjectEnvironment>) -> Self {
        self.inner = self.inner.set_environment(input);
        self
    }
    /// <p>Information about the build environment for the build project.</p>
    pub fn get_environment(&self) -> &::std::option::Option<crate::types::ProjectEnvironment> {
        self.inner.get_environment()
    }
    /// <p>The ARN of the IAM role that enables CodeBuild to interact with dependent Amazon Web Services services on behalf of the Amazon Web Services account.</p>
    pub fn service_role(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.service_role(input.into());
        self
    }
    /// <p>The ARN of the IAM role that enables CodeBuild to interact with dependent Amazon Web Services services on behalf of the Amazon Web Services account.</p>
    pub fn set_service_role(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_service_role(input);
        self
    }
    /// <p>The ARN of the IAM role that enables CodeBuild to interact with dependent Amazon Web Services services on behalf of the Amazon Web Services account.</p>
    pub fn get_service_role(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_service_role()
    }
    /// <p>How long, in minutes, from 5 to 2160 (36 hours), for CodeBuild to wait before it times out any build that has not been marked as completed. The default is 60 minutes.</p>
    pub fn timeout_in_minutes(mut self, input: i32) -> Self {
        self.inner = self.inner.timeout_in_minutes(input);
        self
    }
    /// <p>How long, in minutes, from 5 to 2160 (36 hours), for CodeBuild to wait before it times out any build that has not been marked as completed. The default is 60 minutes.</p>
    pub fn set_timeout_in_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_timeout_in_minutes(input);
        self
    }
    /// <p>How long, in minutes, from 5 to 2160 (36 hours), for CodeBuild to wait before it times out any build that has not been marked as completed. The default is 60 minutes.</p>
    pub fn get_timeout_in_minutes(&self) -> &::std::option::Option<i32> {
        self.inner.get_timeout_in_minutes()
    }
    /// <p>The number of minutes a build is allowed to be queued before it times out.</p>
    pub fn queued_timeout_in_minutes(mut self, input: i32) -> Self {
        self.inner = self.inner.queued_timeout_in_minutes(input);
        self
    }
    /// <p>The number of minutes a build is allowed to be queued before it times out.</p>
    pub fn set_queued_timeout_in_minutes(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_queued_timeout_in_minutes(input);
        self
    }
    /// <p>The number of minutes a build is allowed to be queued before it times out.</p>
    pub fn get_queued_timeout_in_minutes(&self) -> &::std::option::Option<i32> {
        self.inner.get_queued_timeout_in_minutes()
    }
    /// <p>The Key Management Service customer master key (CMK) to be used for encrypting the build output artifacts.</p><note>
    /// <p>You can use a cross-account KMS key to encrypt the build output artifacts if your service role has permission to that key.</p>
    /// </note>
    /// <p>You can specify either the Amazon Resource Name (ARN) of the CMK or, if available, the CMK's alias (using the format <code>alias/<alias-name></alias-name></code>).</p>
    pub fn encryption_key(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.encryption_key(input.into());
        self
    }
    /// <p>The Key Management Service customer master key (CMK) to be used for encrypting the build output artifacts.</p><note>
    /// <p>You can use a cross-account KMS key to encrypt the build output artifacts if your service role has permission to that key.</p>
    /// </note>
    /// <p>You can specify either the Amazon Resource Name (ARN) of the CMK or, if available, the CMK's alias (using the format <code>alias/<alias-name></alias-name></code>).</p>
    pub fn set_encryption_key(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_encryption_key(input);
        self
    }
    /// <p>The Key Management Service customer master key (CMK) to be used for encrypting the build output artifacts.</p><note>
    /// <p>You can use a cross-account KMS key to encrypt the build output artifacts if your service role has permission to that key.</p>
    /// </note>
    /// <p>You can specify either the Amazon Resource Name (ARN) of the CMK or, if available, the CMK's alias (using the format <code>alias/<alias-name></alias-name></code>).</p>
    pub fn get_encryption_key(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_encryption_key()
    }
    ///
    /// Appends an item to `tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>A list of tag key and value pairs associated with this build project.</p>
    /// <p>These tags are available for use by Amazon Web Services services that support CodeBuild build project tags.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>A list of tag key and value pairs associated with this build project.</p>
    /// <p>These tags are available for use by Amazon Web Services services that support CodeBuild build project tags.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>A list of tag key and value pairs associated with this build project.</p>
    /// <p>These tags are available for use by Amazon Web Services services that support CodeBuild build project tags.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    /// <p>VpcConfig enables CodeBuild to access resources in an Amazon VPC.</p><note>
    /// <p>If you're using compute fleets during project creation, do not provide vpcConfig.</p>
    /// </note>
    pub fn vpc_config(mut self, input: crate::types::VpcConfig) -> Self {
        self.inner = self.inner.vpc_config(input);
        self
    }
    /// <p>VpcConfig enables CodeBuild to access resources in an Amazon VPC.</p><note>
    /// <p>If you're using compute fleets during project creation, do not provide vpcConfig.</p>
    /// </note>
    pub fn set_vpc_config(mut self, input: ::std::option::Option<crate::types::VpcConfig>) -> Self {
        self.inner = self.inner.set_vpc_config(input);
        self
    }
    /// <p>VpcConfig enables CodeBuild to access resources in an Amazon VPC.</p><note>
    /// <p>If you're using compute fleets during project creation, do not provide vpcConfig.</p>
    /// </note>
    pub fn get_vpc_config(&self) -> &::std::option::Option<crate::types::VpcConfig> {
        self.inner.get_vpc_config()
    }
    /// <p>Set this to true to generate a publicly accessible URL for your project's build badge.</p>
    pub fn badge_enabled(mut self, input: bool) -> Self {
        self.inner = self.inner.badge_enabled(input);
        self
    }
    /// <p>Set this to true to generate a publicly accessible URL for your project's build badge.</p>
    pub fn set_badge_enabled(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_badge_enabled(input);
        self
    }
    /// <p>Set this to true to generate a publicly accessible URL for your project's build badge.</p>
    pub fn get_badge_enabled(&self) -> &::std::option::Option<bool> {
        self.inner.get_badge_enabled()
    }
    /// <p>Information about logs for the build project. These can be logs in CloudWatch Logs, logs uploaded to a specified S3 bucket, or both.</p>
    pub fn logs_config(mut self, input: crate::types::LogsConfig) -> Self {
        self.inner = self.inner.logs_config(input);
        self
    }
    /// <p>Information about logs for the build project. These can be logs in CloudWatch Logs, logs uploaded to a specified S3 bucket, or both.</p>
    pub fn set_logs_config(mut self, input: ::std::option::Option<crate::types::LogsConfig>) -> Self {
        self.inner = self.inner.set_logs_config(input);
        self
    }
    /// <p>Information about logs for the build project. These can be logs in CloudWatch Logs, logs uploaded to a specified S3 bucket, or both.</p>
    pub fn get_logs_config(&self) -> &::std::option::Option<crate::types::LogsConfig> {
        self.inner.get_logs_config()
    }
    ///
    /// Appends an item to `fileSystemLocations`.
    ///
    /// To override the contents of this collection use [`set_file_system_locations`](Self::set_file_system_locations).
    ///
    /// <p>An array of <code>ProjectFileSystemLocation</code> objects for a CodeBuild build project. A <code>ProjectFileSystemLocation</code> object specifies the <code>identifier</code>, <code>location</code>, <code>mountOptions</code>, <code>mountPoint</code>, and <code>type</code> of a file system created using Amazon Elastic File System.</p>
    pub fn file_system_locations(mut self, input: crate::types::ProjectFileSystemLocation) -> Self {
        self.inner = self.inner.file_system_locations(input);
        self
    }
    /// <p>An array of <code>ProjectFileSystemLocation</code> objects for a CodeBuild build project. A <code>ProjectFileSystemLocation</code> object specifies the <code>identifier</code>, <code>location</code>, <code>mountOptions</code>, <code>mountPoint</code>, and <code>type</code> of a file system created using Amazon Elastic File System.</p>
    pub fn set_file_system_locations(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ProjectFileSystemLocation>>) -> Self {
        self.inner = self.inner.set_file_system_locations(input);
        self
    }
    /// <p>An array of <code>ProjectFileSystemLocation</code> objects for a CodeBuild build project. A <code>ProjectFileSystemLocation</code> object specifies the <code>identifier</code>, <code>location</code>, <code>mountOptions</code>, <code>mountPoint</code>, and <code>type</code> of a file system created using Amazon Elastic File System.</p>
    pub fn get_file_system_locations(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ProjectFileSystemLocation>> {
        self.inner.get_file_system_locations()
    }
    /// <p>A <code>ProjectBuildBatchConfig</code> object that defines the batch build options for the project.</p>
    pub fn build_batch_config(mut self, input: crate::types::ProjectBuildBatchConfig) -> Self {
        self.inner = self.inner.build_batch_config(input);
        self
    }
    /// <p>A <code>ProjectBuildBatchConfig</code> object that defines the batch build options for the project.</p>
    pub fn set_build_batch_config(mut self, input: ::std::option::Option<crate::types::ProjectBuildBatchConfig>) -> Self {
        self.inner = self.inner.set_build_batch_config(input);
        self
    }
    /// <p>A <code>ProjectBuildBatchConfig</code> object that defines the batch build options for the project.</p>
    pub fn get_build_batch_config(&self) -> &::std::option::Option<crate::types::ProjectBuildBatchConfig> {
        self.inner.get_build_batch_config()
    }
    /// <p>The maximum number of concurrent builds that are allowed for this project.</p>
    /// <p>New builds are only started if the current number of builds is less than or equal to this limit. If the current build count meets this limit, new builds are throttled and are not run.</p>
    pub fn concurrent_build_limit(mut self, input: i32) -> Self {
        self.inner = self.inner.concurrent_build_limit(input);
        self
    }
    /// <p>The maximum number of concurrent builds that are allowed for this project.</p>
    /// <p>New builds are only started if the current number of builds is less than or equal to this limit. If the current build count meets this limit, new builds are throttled and are not run.</p>
    pub fn set_concurrent_build_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_concurrent_build_limit(input);
        self
    }
    /// <p>The maximum number of concurrent builds that are allowed for this project.</p>
    /// <p>New builds are only started if the current number of builds is less than or equal to this limit. If the current build count meets this limit, new builds are throttled and are not run.</p>
    pub fn get_concurrent_build_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_concurrent_build_limit()
    }
    /// <p>The maximum number of additional automatic retries after a failed build. For example, if the auto-retry limit is set to 2, CodeBuild will call the <code>RetryBuild</code> API to automatically retry your build for up to 2 additional times.</p>
    pub fn auto_retry_limit(mut self, input: i32) -> Self {
        self.inner = self.inner.auto_retry_limit(input);
        self
    }
    /// <p>The maximum number of additional automatic retries after a failed build. For example, if the auto-retry limit is set to 2, CodeBuild will call the <code>RetryBuild</code> API to automatically retry your build for up to 2 additional times.</p>
    pub fn set_auto_retry_limit(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_auto_retry_limit(input);
        self
    }
    /// <p>The maximum number of additional automatic retries after a failed build. For example, if the auto-retry limit is set to 2, CodeBuild will call the <code>RetryBuild</code> API to automatically retry your build for up to 2 additional times.</p>
    pub fn get_auto_retry_limit(&self) -> &::std::option::Option<i32> {
        self.inner.get_auto_retry_limit()
    }
}
