// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::activate_type::_activate_type_output::ActivateTypeOutputBuilder;

pub use crate::operation::activate_type::_activate_type_input::ActivateTypeInputBuilder;

impl crate::operation::activate_type::builders::ActivateTypeInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::activate_type::ActivateTypeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::activate_type::ActivateTypeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.activate_type();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `ActivateType`.
///
/// <p>Activates a public third-party extension, making it available for use in stack templates. Once you have activated a public third-party extension in your account and Region, use <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/APIReference/API_SetTypeConfiguration.html">SetTypeConfiguration</a> to specify configuration properties for the extension. For more information, see <a href="https://docs.aws.amazon.com/AWSCloudFormation/latest/UserGuide/registry-public.html">Using public extensions</a> in the <i>CloudFormation User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct ActivateTypeFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::activate_type::builders::ActivateTypeInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::activate_type::ActivateTypeOutput,
        crate::operation::activate_type::ActivateTypeError,
    > for ActivateTypeFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::activate_type::ActivateTypeOutput,
            crate::operation::activate_type::ActivateTypeError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl ActivateTypeFluentBuilder {
    /// Creates a new `ActivateTypeFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the ActivateType as a reference.
    pub fn as_input(&self) -> &crate::operation::activate_type::builders::ActivateTypeInputBuilder {
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
        crate::operation::activate_type::ActivateTypeOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::activate_type::ActivateTypeError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::activate_type::ActivateType::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::activate_type::ActivateType::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::activate_type::ActivateTypeOutput,
        crate::operation::activate_type::ActivateTypeError,
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
    /// <p>The extension type.</p>
    /// <p>Conditional: You must specify <code>PublicTypeArn</code>, or <code>TypeName</code>, <code>Type</code>, and <code>PublisherId</code>.</p>
    pub fn r#type(mut self, input: crate::types::ThirdPartyType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>The extension type.</p>
    /// <p>Conditional: You must specify <code>PublicTypeArn</code>, or <code>TypeName</code>, <code>Type</code>, and <code>PublisherId</code>.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::ThirdPartyType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>The extension type.</p>
    /// <p>Conditional: You must specify <code>PublicTypeArn</code>, or <code>TypeName</code>, <code>Type</code>, and <code>PublisherId</code>.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::ThirdPartyType> {
        self.inner.get_type()
    }
    /// <p>The Amazon Resource Name (ARN) of the public extension.</p>
    /// <p>Conditional: You must specify <code>PublicTypeArn</code>, or <code>TypeName</code>, <code>Type</code>, and <code>PublisherId</code>.</p>
    pub fn public_type_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.public_type_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the public extension.</p>
    /// <p>Conditional: You must specify <code>PublicTypeArn</code>, or <code>TypeName</code>, <code>Type</code>, and <code>PublisherId</code>.</p>
    pub fn set_public_type_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_public_type_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the public extension.</p>
    /// <p>Conditional: You must specify <code>PublicTypeArn</code>, or <code>TypeName</code>, <code>Type</code>, and <code>PublisherId</code>.</p>
    pub fn get_public_type_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_public_type_arn()
    }
    /// <p>The ID of the extension publisher.</p>
    /// <p>Conditional: You must specify <code>PublicTypeArn</code>, or <code>TypeName</code>, <code>Type</code>, and <code>PublisherId</code>.</p>
    pub fn publisher_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.publisher_id(input.into());
        self
    }
    /// <p>The ID of the extension publisher.</p>
    /// <p>Conditional: You must specify <code>PublicTypeArn</code>, or <code>TypeName</code>, <code>Type</code>, and <code>PublisherId</code>.</p>
    pub fn set_publisher_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_publisher_id(input);
        self
    }
    /// <p>The ID of the extension publisher.</p>
    /// <p>Conditional: You must specify <code>PublicTypeArn</code>, or <code>TypeName</code>, <code>Type</code>, and <code>PublisherId</code>.</p>
    pub fn get_publisher_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_publisher_id()
    }
    /// <p>The name of the extension.</p>
    /// <p>Conditional: You must specify <code>PublicTypeArn</code>, or <code>TypeName</code>, <code>Type</code>, and <code>PublisherId</code>.</p>
    pub fn type_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.type_name(input.into());
        self
    }
    /// <p>The name of the extension.</p>
    /// <p>Conditional: You must specify <code>PublicTypeArn</code>, or <code>TypeName</code>, <code>Type</code>, and <code>PublisherId</code>.</p>
    pub fn set_type_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_type_name(input);
        self
    }
    /// <p>The name of the extension.</p>
    /// <p>Conditional: You must specify <code>PublicTypeArn</code>, or <code>TypeName</code>, <code>Type</code>, and <code>PublisherId</code>.</p>
    pub fn get_type_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_type_name()
    }
    /// <p>An alias to assign to the public extension, in this account and Region. If you specify an alias for the extension, CloudFormation treats the alias as the extension type name within this account and Region. You must use the alias to refer to the extension in your templates, API calls, and CloudFormation console.</p>
    /// <p>An extension alias must be unique within a given account and Region. You can activate the same public resource multiple times in the same account and Region, using different type name aliases.</p>
    pub fn type_name_alias(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.type_name_alias(input.into());
        self
    }
    /// <p>An alias to assign to the public extension, in this account and Region. If you specify an alias for the extension, CloudFormation treats the alias as the extension type name within this account and Region. You must use the alias to refer to the extension in your templates, API calls, and CloudFormation console.</p>
    /// <p>An extension alias must be unique within a given account and Region. You can activate the same public resource multiple times in the same account and Region, using different type name aliases.</p>
    pub fn set_type_name_alias(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_type_name_alias(input);
        self
    }
    /// <p>An alias to assign to the public extension, in this account and Region. If you specify an alias for the extension, CloudFormation treats the alias as the extension type name within this account and Region. You must use the alias to refer to the extension in your templates, API calls, and CloudFormation console.</p>
    /// <p>An extension alias must be unique within a given account and Region. You can activate the same public resource multiple times in the same account and Region, using different type name aliases.</p>
    pub fn get_type_name_alias(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_type_name_alias()
    }
    /// <p>Whether to automatically update the extension in this account and Region when a new <i>minor</i> version is published by the extension publisher. Major versions released by the publisher must be manually updated.</p>
    /// <p>The default is <code>true</code>.</p>
    pub fn auto_update(mut self, input: bool) -> Self {
        self.inner = self.inner.auto_update(input);
        self
    }
    /// <p>Whether to automatically update the extension in this account and Region when a new <i>minor</i> version is published by the extension publisher. Major versions released by the publisher must be manually updated.</p>
    /// <p>The default is <code>true</code>.</p>
    pub fn set_auto_update(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_auto_update(input);
        self
    }
    /// <p>Whether to automatically update the extension in this account and Region when a new <i>minor</i> version is published by the extension publisher. Major versions released by the publisher must be manually updated.</p>
    /// <p>The default is <code>true</code>.</p>
    pub fn get_auto_update(&self) -> &::std::option::Option<bool> {
        self.inner.get_auto_update()
    }
    /// <p>Contains logging configuration information for an extension.</p>
    pub fn logging_config(mut self, input: crate::types::LoggingConfig) -> Self {
        self.inner = self.inner.logging_config(input);
        self
    }
    /// <p>Contains logging configuration information for an extension.</p>
    pub fn set_logging_config(mut self, input: ::std::option::Option<crate::types::LoggingConfig>) -> Self {
        self.inner = self.inner.set_logging_config(input);
        self
    }
    /// <p>Contains logging configuration information for an extension.</p>
    pub fn get_logging_config(&self) -> &::std::option::Option<crate::types::LoggingConfig> {
        self.inner.get_logging_config()
    }
    /// <p>The name of the IAM execution role to use to activate the extension.</p>
    pub fn execution_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.execution_role_arn(input.into());
        self
    }
    /// <p>The name of the IAM execution role to use to activate the extension.</p>
    pub fn set_execution_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_execution_role_arn(input);
        self
    }
    /// <p>The name of the IAM execution role to use to activate the extension.</p>
    pub fn get_execution_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_execution_role_arn()
    }
    /// <p>Manually updates a previously-activated type to a new major or minor version, if available. You can also use this parameter to update the value of <code>AutoUpdate</code>.</p>
    /// <ul>
    /// <li>
    /// <p><code>MAJOR</code>: CloudFormation updates the extension to the newest major version, if one is available.</p></li>
    /// <li>
    /// <p><code>MINOR</code>: CloudFormation updates the extension to the newest minor version, if one is available.</p></li>
    /// </ul>
    pub fn version_bump(mut self, input: crate::types::VersionBump) -> Self {
        self.inner = self.inner.version_bump(input);
        self
    }
    /// <p>Manually updates a previously-activated type to a new major or minor version, if available. You can also use this parameter to update the value of <code>AutoUpdate</code>.</p>
    /// <ul>
    /// <li>
    /// <p><code>MAJOR</code>: CloudFormation updates the extension to the newest major version, if one is available.</p></li>
    /// <li>
    /// <p><code>MINOR</code>: CloudFormation updates the extension to the newest minor version, if one is available.</p></li>
    /// </ul>
    pub fn set_version_bump(mut self, input: ::std::option::Option<crate::types::VersionBump>) -> Self {
        self.inner = self.inner.set_version_bump(input);
        self
    }
    /// <p>Manually updates a previously-activated type to a new major or minor version, if available. You can also use this parameter to update the value of <code>AutoUpdate</code>.</p>
    /// <ul>
    /// <li>
    /// <p><code>MAJOR</code>: CloudFormation updates the extension to the newest major version, if one is available.</p></li>
    /// <li>
    /// <p><code>MINOR</code>: CloudFormation updates the extension to the newest minor version, if one is available.</p></li>
    /// </ul>
    pub fn get_version_bump(&self) -> &::std::option::Option<crate::types::VersionBump> {
        self.inner.get_version_bump()
    }
    /// <p>The major version of this extension you want to activate, if multiple major versions are available. The default is the latest major version. CloudFormation uses the latest available <i>minor</i> version of the major version selected.</p>
    /// <p>You can specify <code>MajorVersion</code> or <code>VersionBump</code>, but not both.</p>
    pub fn major_version(mut self, input: i64) -> Self {
        self.inner = self.inner.major_version(input);
        self
    }
    /// <p>The major version of this extension you want to activate, if multiple major versions are available. The default is the latest major version. CloudFormation uses the latest available <i>minor</i> version of the major version selected.</p>
    /// <p>You can specify <code>MajorVersion</code> or <code>VersionBump</code>, but not both.</p>
    pub fn set_major_version(mut self, input: ::std::option::Option<i64>) -> Self {
        self.inner = self.inner.set_major_version(input);
        self
    }
    /// <p>The major version of this extension you want to activate, if multiple major versions are available. The default is the latest major version. CloudFormation uses the latest available <i>minor</i> version of the major version selected.</p>
    /// <p>You can specify <code>MajorVersion</code> or <code>VersionBump</code>, but not both.</p>
    pub fn get_major_version(&self) -> &::std::option::Option<i64> {
        self.inner.get_major_version()
    }
}
