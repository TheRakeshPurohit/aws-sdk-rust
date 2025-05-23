// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::enable_allowed_images_settings::_enable_allowed_images_settings_output::EnableAllowedImagesSettingsOutputBuilder;

pub use crate::operation::enable_allowed_images_settings::_enable_allowed_images_settings_input::EnableAllowedImagesSettingsInputBuilder;

impl crate::operation::enable_allowed_images_settings::builders::EnableAllowedImagesSettingsInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::enable_allowed_images_settings::EnableAllowedImagesSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::enable_allowed_images_settings::EnableAllowedImagesSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.enable_allowed_images_settings();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `EnableAllowedImagesSettings`.
///
/// <p>Enables Allowed AMIs for your account in the specified Amazon Web Services Region. Two values are accepted:</p>
/// <ul>
/// <li>
/// <p><code>enabled</code>: The image criteria in your Allowed AMIs settings are applied. As a result, only AMIs matching these criteria are discoverable and can be used by your account to launch instances.</p></li>
/// <li>
/// <p><code>audit-mode</code>: The image criteria in your Allowed AMIs settings are not applied. No restrictions are placed on AMI discoverability or usage. Users in your account can launch instances using any public AMI or AMI shared with your account.</p>
/// <p>The purpose of <code>audit-mode</code> is to indicate which AMIs will be affected when Allowed AMIs is <code>enabled</code>. In <code>audit-mode</code>, each AMI displays either <code>"ImageAllowed": true</code> or <code>"ImageAllowed": false</code> to indicate whether the AMI will be discoverable and available to users in the account when Allowed AMIs is enabled.</p></li>
/// </ul><note>
/// <p>The Allowed AMIs feature does not restrict the AMIs owned by your account. Regardless of the criteria you set, the AMIs created by your account will always be discoverable and usable by users in your account.</p>
/// </note>
/// <p>For more information, see <a href="https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-allowed-amis.html">Control the discovery and use of AMIs in Amazon EC2 with Allowed AMIs</a> in <i>Amazon EC2 User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct EnableAllowedImagesSettingsFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::enable_allowed_images_settings::builders::EnableAllowedImagesSettingsInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::enable_allowed_images_settings::EnableAllowedImagesSettingsOutput,
        crate::operation::enable_allowed_images_settings::EnableAllowedImagesSettingsError,
    > for EnableAllowedImagesSettingsFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::enable_allowed_images_settings::EnableAllowedImagesSettingsOutput,
            crate::operation::enable_allowed_images_settings::EnableAllowedImagesSettingsError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl EnableAllowedImagesSettingsFluentBuilder {
    /// Creates a new `EnableAllowedImagesSettingsFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the EnableAllowedImagesSettings as a reference.
    pub fn as_input(&self) -> &crate::operation::enable_allowed_images_settings::builders::EnableAllowedImagesSettingsInputBuilder {
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
        crate::operation::enable_allowed_images_settings::EnableAllowedImagesSettingsOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::enable_allowed_images_settings::EnableAllowedImagesSettingsError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::enable_allowed_images_settings::EnableAllowedImagesSettings::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::enable_allowed_images_settings::EnableAllowedImagesSettings::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::enable_allowed_images_settings::EnableAllowedImagesSettingsOutput,
        crate::operation::enable_allowed_images_settings::EnableAllowedImagesSettingsError,
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
    /// <p>Specify <code>enabled</code> to apply the image criteria specified by the Allowed AMIs settings. Specify <code>audit-mode</code> so that you can check which AMIs will be allowed or not allowed by the image criteria.</p>
    pub fn allowed_images_settings_state(mut self, input: crate::types::AllowedImagesSettingsEnabledState) -> Self {
        self.inner = self.inner.allowed_images_settings_state(input);
        self
    }
    /// <p>Specify <code>enabled</code> to apply the image criteria specified by the Allowed AMIs settings. Specify <code>audit-mode</code> so that you can check which AMIs will be allowed or not allowed by the image criteria.</p>
    pub fn set_allowed_images_settings_state(mut self, input: ::std::option::Option<crate::types::AllowedImagesSettingsEnabledState>) -> Self {
        self.inner = self.inner.set_allowed_images_settings_state(input);
        self
    }
    /// <p>Specify <code>enabled</code> to apply the image criteria specified by the Allowed AMIs settings. Specify <code>audit-mode</code> so that you can check which AMIs will be allowed or not allowed by the image criteria.</p>
    pub fn get_allowed_images_settings_state(&self) -> &::std::option::Option<crate::types::AllowedImagesSettingsEnabledState> {
        self.inner.get_allowed_images_settings_state()
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Checks whether you have the required permissions for the action, without actually making the request, and provides an error response. If you have the required permissions, the error response is <code>DryRunOperation</code>. Otherwise, it is <code>UnauthorizedOperation</code>.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
}
