// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_call_analytics_job::_start_call_analytics_job_output::StartCallAnalyticsJobOutputBuilder;

pub use crate::operation::start_call_analytics_job::_start_call_analytics_job_input::StartCallAnalyticsJobInputBuilder;

impl crate::operation::start_call_analytics_job::builders::StartCallAnalyticsJobInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_call_analytics_job::StartCallAnalyticsJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_call_analytics_job::StartCallAnalyticsJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_call_analytics_job();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartCallAnalyticsJob`.
///
/// <p>Transcribes the audio from a customer service call and applies any additional Request Parameters you choose to include in your request.</p>
/// <p>In addition to many standard transcription features, Call Analytics provides you with call characteristics, call summarization, speaker sentiment, and optional redaction of your text transcript and your audio file. You can also apply custom categories to flag specified conditions. To learn more about these features and insights, refer to <a href="https://docs.aws.amazon.com/transcribe/latest/dg/call-analytics.html">Analyzing call center audio with Call Analytics</a>.</p>
/// <p>If you want to apply categories to your Call Analytics job, you must create them before submitting your job request. Categories cannot be retroactively applied to a job. To create a new category, use the operation. To learn more about Call Analytics categories, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/tca-categories-batch.html">Creating categories for post-call transcriptions</a> and <a href="https://docs.aws.amazon.com/transcribe/latest/dg/tca-categories-stream.html">Creating categories for real-time transcriptions</a>.</p>
/// <p>To make a <code>StartCallAnalyticsJob</code> request, you must first upload your media file into an Amazon S3 bucket; you can then specify the Amazon S3 location of the file using the <code>Media</code> parameter.</p>
/// <p>Job queuing is available for Call Analytics jobs. If you pass a <code>DataAccessRoleArn</code> in your request and you exceed your Concurrent Job Limit, your job will automatically be added to a queue to be processed once your concurrent job count is below the limit.</p>
/// <p>You must include the following parameters in your <code>StartCallAnalyticsJob</code> request:</p>
/// <ul>
/// <li>
/// <p><code>region</code>: The Amazon Web Services Region where you are making your request. For a list of Amazon Web Services Regions supported with Amazon Transcribe, refer to <a href="https://docs.aws.amazon.com/general/latest/gr/transcribe.html">Amazon Transcribe endpoints and quotas</a>.</p></li>
/// <li>
/// <p><code>CallAnalyticsJobName</code>: A custom name that you create for your transcription job that's unique within your Amazon Web Services account.</p></li>
/// <li>
/// <p><code>Media</code> (<code>MediaFileUri</code> or <code>RedactedMediaFileUri</code>): The Amazon S3 location of your media file.</p></li>
/// </ul><note>
/// <p>With Call Analytics, you can redact the audio contained in your media file by including <code>RedactedMediaFileUri</code>, instead of <code>MediaFileUri</code>, to specify the location of your input audio. If you choose to redact your audio, you can find your redacted media at the location specified in the <code>RedactedMediaFileUri</code> field of your response.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct StartCallAnalyticsJobFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_call_analytics_job::builders::StartCallAnalyticsJobInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_call_analytics_job::StartCallAnalyticsJobOutput,
        crate::operation::start_call_analytics_job::StartCallAnalyticsJobError,
    > for StartCallAnalyticsJobFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_call_analytics_job::StartCallAnalyticsJobOutput,
            crate::operation::start_call_analytics_job::StartCallAnalyticsJobError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartCallAnalyticsJobFluentBuilder {
    /// Creates a new `StartCallAnalyticsJobFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartCallAnalyticsJob as a reference.
    pub fn as_input(&self) -> &crate::operation::start_call_analytics_job::builders::StartCallAnalyticsJobInputBuilder {
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
        crate::operation::start_call_analytics_job::StartCallAnalyticsJobOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_call_analytics_job::StartCallAnalyticsJobError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_call_analytics_job::StartCallAnalyticsJob::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_call_analytics_job::StartCallAnalyticsJob::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_call_analytics_job::StartCallAnalyticsJobOutput,
        crate::operation::start_call_analytics_job::StartCallAnalyticsJobError,
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
    /// <p>A unique name, chosen by you, for your Call Analytics job.</p>
    /// <p>This name is case sensitive, cannot contain spaces, and must be unique within an Amazon Web Services account. If you try to create a new job with the same name as an existing job, you get a <code>ConflictException</code> error.</p>
    pub fn call_analytics_job_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.call_analytics_job_name(input.into());
        self
    }
    /// <p>A unique name, chosen by you, for your Call Analytics job.</p>
    /// <p>This name is case sensitive, cannot contain spaces, and must be unique within an Amazon Web Services account. If you try to create a new job with the same name as an existing job, you get a <code>ConflictException</code> error.</p>
    pub fn set_call_analytics_job_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_call_analytics_job_name(input);
        self
    }
    /// <p>A unique name, chosen by you, for your Call Analytics job.</p>
    /// <p>This name is case sensitive, cannot contain spaces, and must be unique within an Amazon Web Services account. If you try to create a new job with the same name as an existing job, you get a <code>ConflictException</code> error.</p>
    pub fn get_call_analytics_job_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_call_analytics_job_name()
    }
    /// <p>Describes the Amazon S3 location of the media file you want to use in your Call Analytics request.</p>
    pub fn media(mut self, input: crate::types::Media) -> Self {
        self.inner = self.inner.media(input);
        self
    }
    /// <p>Describes the Amazon S3 location of the media file you want to use in your Call Analytics request.</p>
    pub fn set_media(mut self, input: ::std::option::Option<crate::types::Media>) -> Self {
        self.inner = self.inner.set_media(input);
        self
    }
    /// <p>Describes the Amazon S3 location of the media file you want to use in your Call Analytics request.</p>
    pub fn get_media(&self) -> &::std::option::Option<crate::types::Media> {
        self.inner.get_media()
    }
    /// <p>The Amazon S3 location where you want your Call Analytics transcription output stored. You can use any of the following formats to specify the output location:</p>
    /// <ol>
    /// <li>
    /// <p>s3://DOC-EXAMPLE-BUCKET</p></li>
    /// <li>
    /// <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/</p></li>
    /// <li>
    /// <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/my-call-analytics-job.json</p></li>
    /// </ol>
    /// <p>Unless you specify a file name (option 3), the name of your output file has a default value that matches the name you specified for your transcription job using the <code>CallAnalyticsJobName</code> parameter.</p>
    /// <p>You can specify a KMS key to encrypt your output using the <code>OutputEncryptionKMSKeyId</code> parameter. If you do not specify a KMS key, Amazon Transcribe uses the default Amazon S3 key for server-side encryption.</p>
    /// <p>If you do not specify <code>OutputLocation</code>, your transcript is placed in a service-managed Amazon S3 bucket and you are provided with a URI to access your transcript.</p>
    pub fn output_location(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.output_location(input.into());
        self
    }
    /// <p>The Amazon S3 location where you want your Call Analytics transcription output stored. You can use any of the following formats to specify the output location:</p>
    /// <ol>
    /// <li>
    /// <p>s3://DOC-EXAMPLE-BUCKET</p></li>
    /// <li>
    /// <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/</p></li>
    /// <li>
    /// <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/my-call-analytics-job.json</p></li>
    /// </ol>
    /// <p>Unless you specify a file name (option 3), the name of your output file has a default value that matches the name you specified for your transcription job using the <code>CallAnalyticsJobName</code> parameter.</p>
    /// <p>You can specify a KMS key to encrypt your output using the <code>OutputEncryptionKMSKeyId</code> parameter. If you do not specify a KMS key, Amazon Transcribe uses the default Amazon S3 key for server-side encryption.</p>
    /// <p>If you do not specify <code>OutputLocation</code>, your transcript is placed in a service-managed Amazon S3 bucket and you are provided with a URI to access your transcript.</p>
    pub fn set_output_location(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_output_location(input);
        self
    }
    /// <p>The Amazon S3 location where you want your Call Analytics transcription output stored. You can use any of the following formats to specify the output location:</p>
    /// <ol>
    /// <li>
    /// <p>s3://DOC-EXAMPLE-BUCKET</p></li>
    /// <li>
    /// <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/</p></li>
    /// <li>
    /// <p>s3://DOC-EXAMPLE-BUCKET/my-output-folder/my-call-analytics-job.json</p></li>
    /// </ol>
    /// <p>Unless you specify a file name (option 3), the name of your output file has a default value that matches the name you specified for your transcription job using the <code>CallAnalyticsJobName</code> parameter.</p>
    /// <p>You can specify a KMS key to encrypt your output using the <code>OutputEncryptionKMSKeyId</code> parameter. If you do not specify a KMS key, Amazon Transcribe uses the default Amazon S3 key for server-side encryption.</p>
    /// <p>If you do not specify <code>OutputLocation</code>, your transcript is placed in a service-managed Amazon S3 bucket and you are provided with a URI to access your transcript.</p>
    pub fn get_output_location(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_output_location()
    }
    /// <p>The KMS key you want to use to encrypt your Call Analytics output.</p>
    /// <p>If using a key located in the <b>current</b> Amazon Web Services account, you can specify your KMS key in one of four ways:</p>
    /// <ol>
    /// <li>
    /// <p>Use the KMS key ID itself. For example, <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Use an alias for the KMS key ID. For example, <code>alias/ExampleAlias</code>.</p></li>
    /// <li>
    /// <p>Use the Amazon Resource Name (ARN) for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p></li>
    /// </ol>
    /// <p>If using a key located in a <b>different</b> Amazon Web Services account than the current Amazon Web Services account, you can specify your KMS key in one of two ways:</p>
    /// <ol>
    /// <li>
    /// <p>Use the ARN for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p></li>
    /// </ol>
    /// <p>If you do not specify an encryption key, your output is encrypted with the default Amazon S3 key (SSE-S3).</p>
    /// <p>If you specify a KMS key to encrypt your output, you must also specify an output location using the <code>OutputLocation</code> parameter.</p>
    /// <p>Note that the role making the request must have permission to use the specified KMS key.</p>
    pub fn output_encryption_kms_key_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.output_encryption_kms_key_id(input.into());
        self
    }
    /// <p>The KMS key you want to use to encrypt your Call Analytics output.</p>
    /// <p>If using a key located in the <b>current</b> Amazon Web Services account, you can specify your KMS key in one of four ways:</p>
    /// <ol>
    /// <li>
    /// <p>Use the KMS key ID itself. For example, <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Use an alias for the KMS key ID. For example, <code>alias/ExampleAlias</code>.</p></li>
    /// <li>
    /// <p>Use the Amazon Resource Name (ARN) for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p></li>
    /// </ol>
    /// <p>If using a key located in a <b>different</b> Amazon Web Services account than the current Amazon Web Services account, you can specify your KMS key in one of two ways:</p>
    /// <ol>
    /// <li>
    /// <p>Use the ARN for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p></li>
    /// </ol>
    /// <p>If you do not specify an encryption key, your output is encrypted with the default Amazon S3 key (SSE-S3).</p>
    /// <p>If you specify a KMS key to encrypt your output, you must also specify an output location using the <code>OutputLocation</code> parameter.</p>
    /// <p>Note that the role making the request must have permission to use the specified KMS key.</p>
    pub fn set_output_encryption_kms_key_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_output_encryption_kms_key_id(input);
        self
    }
    /// <p>The KMS key you want to use to encrypt your Call Analytics output.</p>
    /// <p>If using a key located in the <b>current</b> Amazon Web Services account, you can specify your KMS key in one of four ways:</p>
    /// <ol>
    /// <li>
    /// <p>Use the KMS key ID itself. For example, <code>1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Use an alias for the KMS key ID. For example, <code>alias/ExampleAlias</code>.</p></li>
    /// <li>
    /// <p>Use the Amazon Resource Name (ARN) for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p></li>
    /// </ol>
    /// <p>If using a key located in a <b>different</b> Amazon Web Services account than the current Amazon Web Services account, you can specify your KMS key in one of two ways:</p>
    /// <ol>
    /// <li>
    /// <p>Use the ARN for the KMS key ID. For example, <code>arn:aws:kms:region:account-ID:key/1234abcd-12ab-34cd-56ef-1234567890ab</code>.</p></li>
    /// <li>
    /// <p>Use the ARN for the KMS key alias. For example, <code>arn:aws:kms:region:account-ID:alias/ExampleAlias</code>.</p></li>
    /// </ol>
    /// <p>If you do not specify an encryption key, your output is encrypted with the default Amazon S3 key (SSE-S3).</p>
    /// <p>If you specify a KMS key to encrypt your output, you must also specify an output location using the <code>OutputLocation</code> parameter.</p>
    /// <p>Note that the role making the request must have permission to use the specified KMS key.</p>
    pub fn get_output_encryption_kms_key_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_output_encryption_kms_key_id()
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role that has permissions to access the Amazon S3 bucket that contains your input files. If the role that you specify doesn’t have the appropriate permissions to access the specified Amazon S3 location, your request fails.</p>
    /// <p>IAM role ARNs have the format <code>arn:partition:iam::account:role/role-name-with-path</code>. For example: <code>arn:aws:iam::111122223333:role/Admin</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a>.</p>
    pub fn data_access_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.data_access_role_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role that has permissions to access the Amazon S3 bucket that contains your input files. If the role that you specify doesn’t have the appropriate permissions to access the specified Amazon S3 location, your request fails.</p>
    /// <p>IAM role ARNs have the format <code>arn:partition:iam::account:role/role-name-with-path</code>. For example: <code>arn:aws:iam::111122223333:role/Admin</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a>.</p>
    pub fn set_data_access_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_data_access_role_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of an IAM role that has permissions to access the Amazon S3 bucket that contains your input files. If the role that you specify doesn’t have the appropriate permissions to access the specified Amazon S3 location, your request fails.</p>
    /// <p>IAM role ARNs have the format <code>arn:partition:iam::account:role/role-name-with-path</code>. For example: <code>arn:aws:iam::111122223333:role/Admin</code>.</p>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/reference_identifiers.html#identifiers-arns">IAM ARNs</a>.</p>
    pub fn get_data_access_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_data_access_role_arn()
    }
    /// <p>Specify additional optional settings in your request, including content redaction; allows you to apply custom language models, vocabulary filters, and custom vocabularies to your Call Analytics job.</p>
    pub fn settings(mut self, input: crate::types::CallAnalyticsJobSettings) -> Self {
        self.inner = self.inner.settings(input);
        self
    }
    /// <p>Specify additional optional settings in your request, including content redaction; allows you to apply custom language models, vocabulary filters, and custom vocabularies to your Call Analytics job.</p>
    pub fn set_settings(mut self, input: ::std::option::Option<crate::types::CallAnalyticsJobSettings>) -> Self {
        self.inner = self.inner.set_settings(input);
        self
    }
    /// <p>Specify additional optional settings in your request, including content redaction; allows you to apply custom language models, vocabulary filters, and custom vocabularies to your Call Analytics job.</p>
    pub fn get_settings(&self) -> &::std::option::Option<crate::types::CallAnalyticsJobSettings> {
        self.inner.get_settings()
    }
    ///
    /// Appends an item to `Tags`.
    ///
    /// To override the contents of this collection use [`set_tags`](Self::set_tags).
    ///
    /// <p>Adds one or more custom tags, each in the form of a key:value pair, to a new call analytics job at the time you start this new job.</p>
    /// <p>To learn more about using tags with Amazon Transcribe, refer to <a href="https://docs.aws.amazon.com/transcribe/latest/dg/tagging.html">Tagging resources</a>.</p>
    pub fn tags(mut self, input: crate::types::Tag) -> Self {
        self.inner = self.inner.tags(input);
        self
    }
    /// <p>Adds one or more custom tags, each in the form of a key:value pair, to a new call analytics job at the time you start this new job.</p>
    /// <p>To learn more about using tags with Amazon Transcribe, refer to <a href="https://docs.aws.amazon.com/transcribe/latest/dg/tagging.html">Tagging resources</a>.</p>
    pub fn set_tags(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Tag>>) -> Self {
        self.inner = self.inner.set_tags(input);
        self
    }
    /// <p>Adds one or more custom tags, each in the form of a key:value pair, to a new call analytics job at the time you start this new job.</p>
    /// <p>To learn more about using tags with Amazon Transcribe, refer to <a href="https://docs.aws.amazon.com/transcribe/latest/dg/tagging.html">Tagging resources</a>.</p>
    pub fn get_tags(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Tag>> {
        self.inner.get_tags()
    }
    ///
    /// Appends an item to `ChannelDefinitions`.
    ///
    /// To override the contents of this collection use [`set_channel_definitions`](Self::set_channel_definitions).
    ///
    /// <p>Makes it possible to specify which speaker is on which channel. For example, if your agent is the first participant to speak, you would set <code>ChannelId</code> to <code>0</code> (to indicate the first channel) and <code>ParticipantRole</code> to <code>AGENT</code> (to indicate that it's the agent speaking).</p>
    pub fn channel_definitions(mut self, input: crate::types::ChannelDefinition) -> Self {
        self.inner = self.inner.channel_definitions(input);
        self
    }
    /// <p>Makes it possible to specify which speaker is on which channel. For example, if your agent is the first participant to speak, you would set <code>ChannelId</code> to <code>0</code> (to indicate the first channel) and <code>ParticipantRole</code> to <code>AGENT</code> (to indicate that it's the agent speaking).</p>
    pub fn set_channel_definitions(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ChannelDefinition>>) -> Self {
        self.inner = self.inner.set_channel_definitions(input);
        self
    }
    /// <p>Makes it possible to specify which speaker is on which channel. For example, if your agent is the first participant to speak, you would set <code>ChannelId</code> to <code>0</code> (to indicate the first channel) and <code>ParticipantRole</code> to <code>AGENT</code> (to indicate that it's the agent speaking).</p>
    pub fn get_channel_definitions(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ChannelDefinition>> {
        self.inner.get_channel_definitions()
    }
}
