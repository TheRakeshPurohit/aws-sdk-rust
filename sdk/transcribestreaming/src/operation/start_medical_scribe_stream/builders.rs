// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::start_medical_scribe_stream::_start_medical_scribe_stream_output::StartMedicalScribeStreamOutputBuilder;

pub use crate::operation::start_medical_scribe_stream::_start_medical_scribe_stream_input::StartMedicalScribeStreamInputBuilder;

impl crate::operation::start_medical_scribe_stream::builders::StartMedicalScribeStreamInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::start_medical_scribe_stream::StartMedicalScribeStreamOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_medical_scribe_stream::StartMedicalScribeStreamError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.start_medical_scribe_stream();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `StartMedicalScribeStream`.
///
/// <p>Starts a bidirectional HTTP/2 stream, where audio is streamed to Amazon Web Services HealthScribe and the transcription results are streamed to your application.</p>
/// <p>When you start a stream, you first specify the stream configuration in a <code>MedicalScribeConfigurationEvent</code>. This event includes channel definitions, encryption settings, and post-stream analytics settings, such as the output configuration for aggregated transcript and clinical note generation. These are additional streaming session configurations beyond those provided in your initial start request headers. Whether you are starting a new session or resuming an existing session, your first event must be a <code>MedicalScribeConfigurationEvent</code>.</p>
/// <p>After you send a <code>MedicalScribeConfigurationEvent</code>, you start <code>AudioEvents</code> and Amazon Web Services HealthScribe responds with real-time transcription results. When you are finished, to start processing the results with the post-stream analytics, send a <code>MedicalScribeSessionControlEvent</code> with a <code>Type</code> of <code>END_OF_SESSION</code> and Amazon Web Services HealthScribe starts the analytics.</p>
/// <p>You can pause or resume streaming. To pause streaming, complete the input stream without sending the <code>MedicalScribeSessionControlEvent</code>. To resume streaming, call the <code>StartMedicalScribeStream</code> and specify the same SessionId you used to start the stream.</p>
/// <p>The following parameters are required:</p>
/// <ul>
/// <li>
/// <p><code>language-code</code></p></li>
/// <li>
/// <p><code>media-encoding</code></p></li>
/// <li>
/// <p><code>media-sample-rate-hertz</code></p></li>
/// </ul>
/// <p></p>
/// <p>For more information on streaming with Amazon Web Services HealthScribe, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/health-scribe-streaming.html">Amazon Web Services HealthScribe</a>.</p>
#[derive(::std::fmt::Debug)]
pub struct StartMedicalScribeStreamFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::start_medical_scribe_stream::builders::StartMedicalScribeStreamInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::start_medical_scribe_stream::StartMedicalScribeStreamOutput,
        crate::operation::start_medical_scribe_stream::StartMedicalScribeStreamError,
    > for StartMedicalScribeStreamFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::start_medical_scribe_stream::StartMedicalScribeStreamOutput,
            crate::operation::start_medical_scribe_stream::StartMedicalScribeStreamError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl StartMedicalScribeStreamFluentBuilder {
    /// Creates a new `StartMedicalScribeStreamFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the StartMedicalScribeStream as a reference.
    pub fn as_input(&self) -> &crate::operation::start_medical_scribe_stream::builders::StartMedicalScribeStreamInputBuilder {
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
        crate::operation::start_medical_scribe_stream::StartMedicalScribeStreamOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::start_medical_scribe_stream::StartMedicalScribeStreamError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::start_medical_scribe_stream::StartMedicalScribeStream::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::start_medical_scribe_stream::StartMedicalScribeStream::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::start_medical_scribe_stream::StartMedicalScribeStreamOutput,
        crate::operation::start_medical_scribe_stream::StartMedicalScribeStreamError,
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
    /// <p>Specify an identifier for your streaming session (in UUID format). If you don't include a SessionId in your request, Amazon Web Services HealthScribe generates an ID and returns it in the response.</p>
    pub fn session_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.session_id(input.into());
        self
    }
    /// <p>Specify an identifier for your streaming session (in UUID format). If you don't include a SessionId in your request, Amazon Web Services HealthScribe generates an ID and returns it in the response.</p>
    pub fn set_session_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_session_id(input);
        self
    }
    /// <p>Specify an identifier for your streaming session (in UUID format). If you don't include a SessionId in your request, Amazon Web Services HealthScribe generates an ID and returns it in the response.</p>
    pub fn get_session_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_session_id()
    }
    /// <p>Specify the language code for your HealthScribe streaming session.</p>
    pub fn language_code(mut self, input: crate::types::MedicalScribeLanguageCode) -> Self {
        self.inner = self.inner.language_code(input);
        self
    }
    /// <p>Specify the language code for your HealthScribe streaming session.</p>
    pub fn set_language_code(mut self, input: ::std::option::Option<crate::types::MedicalScribeLanguageCode>) -> Self {
        self.inner = self.inner.set_language_code(input);
        self
    }
    /// <p>Specify the language code for your HealthScribe streaming session.</p>
    pub fn get_language_code(&self) -> &::std::option::Option<crate::types::MedicalScribeLanguageCode> {
        self.inner.get_language_code()
    }
    /// <p>Specify the sample rate of the input audio (in hertz). Amazon Web Services HealthScribe supports a range from 16,000 Hz to 48,000 Hz. The sample rate you specify must match that of your audio.</p>
    pub fn media_sample_rate_hertz(mut self, input: i32) -> Self {
        self.inner = self.inner.media_sample_rate_hertz(input);
        self
    }
    /// <p>Specify the sample rate of the input audio (in hertz). Amazon Web Services HealthScribe supports a range from 16,000 Hz to 48,000 Hz. The sample rate you specify must match that of your audio.</p>
    pub fn set_media_sample_rate_hertz(mut self, input: ::std::option::Option<i32>) -> Self {
        self.inner = self.inner.set_media_sample_rate_hertz(input);
        self
    }
    /// <p>Specify the sample rate of the input audio (in hertz). Amazon Web Services HealthScribe supports a range from 16,000 Hz to 48,000 Hz. The sample rate you specify must match that of your audio.</p>
    pub fn get_media_sample_rate_hertz(&self) -> &::std::option::Option<i32> {
        self.inner.get_media_sample_rate_hertz()
    }
    /// <p>Specify the encoding used for the input audio.</p>
    /// <p>Supported formats are:</p>
    /// <ul>
    /// <li>
    /// <p>FLAC</p></li>
    /// <li>
    /// <p>OPUS-encoded audio in an Ogg container</p></li>
    /// <li>
    /// <p>PCM (only signed 16-bit little-endian audio formats, which does not include WAV)</p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/how-input.html#how-input-audio">Media formats</a>.</p>
    pub fn media_encoding(mut self, input: crate::types::MedicalScribeMediaEncoding) -> Self {
        self.inner = self.inner.media_encoding(input);
        self
    }
    /// <p>Specify the encoding used for the input audio.</p>
    /// <p>Supported formats are:</p>
    /// <ul>
    /// <li>
    /// <p>FLAC</p></li>
    /// <li>
    /// <p>OPUS-encoded audio in an Ogg container</p></li>
    /// <li>
    /// <p>PCM (only signed 16-bit little-endian audio formats, which does not include WAV)</p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/how-input.html#how-input-audio">Media formats</a>.</p>
    pub fn set_media_encoding(mut self, input: ::std::option::Option<crate::types::MedicalScribeMediaEncoding>) -> Self {
        self.inner = self.inner.set_media_encoding(input);
        self
    }
    /// <p>Specify the encoding used for the input audio.</p>
    /// <p>Supported formats are:</p>
    /// <ul>
    /// <li>
    /// <p>FLAC</p></li>
    /// <li>
    /// <p>OPUS-encoded audio in an Ogg container</p></li>
    /// <li>
    /// <p>PCM (only signed 16-bit little-endian audio formats, which does not include WAV)</p></li>
    /// </ul>
    /// <p>For more information, see <a href="https://docs.aws.amazon.com/transcribe/latest/dg/how-input.html#how-input-audio">Media formats</a>.</p>
    pub fn get_media_encoding(&self) -> &::std::option::Option<crate::types::MedicalScribeMediaEncoding> {
        self.inner.get_media_encoding()
    }
    /// <p>Specify the input stream where you will send events in real time.</p>
    /// <p>The first element of the input stream must be a <code>MedicalScribeConfigurationEvent</code>.</p>
    pub fn input_stream(
        mut self,
        input: ::aws_smithy_http::event_stream::EventStreamSender<
            crate::types::MedicalScribeInputStream,
            crate::types::error::MedicalScribeInputStreamError,
        >,
    ) -> Self {
        self.inner = self.inner.input_stream(input);
        self
    }
    /// <p>Specify the input stream where you will send events in real time.</p>
    /// <p>The first element of the input stream must be a <code>MedicalScribeConfigurationEvent</code>.</p>
    pub fn set_input_stream(
        mut self,
        input: ::std::option::Option<
            ::aws_smithy_http::event_stream::EventStreamSender<
                crate::types::MedicalScribeInputStream,
                crate::types::error::MedicalScribeInputStreamError,
            >,
        >,
    ) -> Self {
        self.inner = self.inner.set_input_stream(input);
        self
    }
    /// <p>Specify the input stream where you will send events in real time.</p>
    /// <p>The first element of the input stream must be a <code>MedicalScribeConfigurationEvent</code>.</p>
    pub fn get_input_stream(
        &self,
    ) -> &::std::option::Option<
        ::aws_smithy_http::event_stream::EventStreamSender<
            crate::types::MedicalScribeInputStream,
            crate::types::error::MedicalScribeInputStreamError,
        >,
    > {
        self.inner.get_input_stream()
    }
}
