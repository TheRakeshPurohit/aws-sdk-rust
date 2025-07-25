// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::update_rule_group::_update_rule_group_output::UpdateRuleGroupOutputBuilder;

pub use crate::operation::update_rule_group::_update_rule_group_input::UpdateRuleGroupInputBuilder;

impl crate::operation::update_rule_group::builders::UpdateRuleGroupInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::update_rule_group::UpdateRuleGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_rule_group::UpdateRuleGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.update_rule_group();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `UpdateRuleGroup`.
///
/// <p>Updates the rule settings for the specified rule group. You use a rule group by reference in one or more firewall policies. When you modify a rule group, you modify all firewall policies that use the rule group.</p>
/// <p>To update a rule group, first call <code>DescribeRuleGroup</code> to retrieve the current <code>RuleGroup</code> object, update the object as needed, and then provide the updated object to this call.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct UpdateRuleGroupFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::update_rule_group::builders::UpdateRuleGroupInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::update_rule_group::UpdateRuleGroupOutput,
        crate::operation::update_rule_group::UpdateRuleGroupError,
    > for UpdateRuleGroupFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::update_rule_group::UpdateRuleGroupOutput,
            crate::operation::update_rule_group::UpdateRuleGroupError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl UpdateRuleGroupFluentBuilder {
    /// Creates a new `UpdateRuleGroupFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the UpdateRuleGroup as a reference.
    pub fn as_input(&self) -> &crate::operation::update_rule_group::builders::UpdateRuleGroupInputBuilder {
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
        crate::operation::update_rule_group::UpdateRuleGroupOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::update_rule_group::UpdateRuleGroupError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::update_rule_group::UpdateRuleGroup::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::update_rule_group::UpdateRuleGroup::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::update_rule_group::UpdateRuleGroupOutput,
        crate::operation::update_rule_group::UpdateRuleGroupError,
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
    /// <p>A token used for optimistic locking. Network Firewall returns a token to your requests that access the rule group. The token marks the state of the rule group resource at the time of the request.</p>
    /// <p>To make changes to the rule group, you provide the token in your request. Network Firewall uses the token to ensure that the rule group hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the rule group again to get a current copy of it with a current token. Reapply your changes as needed, then try the operation again using the new token.</p>
    pub fn update_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.update_token(input.into());
        self
    }
    /// <p>A token used for optimistic locking. Network Firewall returns a token to your requests that access the rule group. The token marks the state of the rule group resource at the time of the request.</p>
    /// <p>To make changes to the rule group, you provide the token in your request. Network Firewall uses the token to ensure that the rule group hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the rule group again to get a current copy of it with a current token. Reapply your changes as needed, then try the operation again using the new token.</p>
    pub fn set_update_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_update_token(input);
        self
    }
    /// <p>A token used for optimistic locking. Network Firewall returns a token to your requests that access the rule group. The token marks the state of the rule group resource at the time of the request.</p>
    /// <p>To make changes to the rule group, you provide the token in your request. Network Firewall uses the token to ensure that the rule group hasn't changed since you last retrieved it. If it has changed, the operation fails with an <code>InvalidTokenException</code>. If this happens, retrieve the rule group again to get a current copy of it with a current token. Reapply your changes as needed, then try the operation again using the new token.</p>
    pub fn get_update_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_update_token()
    }
    /// <p>The Amazon Resource Name (ARN) of the rule group.</p>
    /// <p>You must specify the ARN or the name, and you can specify both.</p>
    pub fn rule_group_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rule_group_arn(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the rule group.</p>
    /// <p>You must specify the ARN or the name, and you can specify both.</p>
    pub fn set_rule_group_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rule_group_arn(input);
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the rule group.</p>
    /// <p>You must specify the ARN or the name, and you can specify both.</p>
    pub fn get_rule_group_arn(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rule_group_arn()
    }
    /// <p>The descriptive name of the rule group. You can't change the name of a rule group after you create it.</p>
    /// <p>You must specify the ARN or the name, and you can specify both.</p>
    pub fn rule_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rule_group_name(input.into());
        self
    }
    /// <p>The descriptive name of the rule group. You can't change the name of a rule group after you create it.</p>
    /// <p>You must specify the ARN or the name, and you can specify both.</p>
    pub fn set_rule_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rule_group_name(input);
        self
    }
    /// <p>The descriptive name of the rule group. You can't change the name of a rule group after you create it.</p>
    /// <p>You must specify the ARN or the name, and you can specify both.</p>
    pub fn get_rule_group_name(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rule_group_name()
    }
    /// <p>An object that defines the rule group rules.</p><note>
    /// <p>You must provide either this rule group setting or a <code>Rules</code> setting, but not both.</p>
    /// </note>
    pub fn rule_group(mut self, input: crate::types::RuleGroup) -> Self {
        self.inner = self.inner.rule_group(input);
        self
    }
    /// <p>An object that defines the rule group rules.</p><note>
    /// <p>You must provide either this rule group setting or a <code>Rules</code> setting, but not both.</p>
    /// </note>
    pub fn set_rule_group(mut self, input: ::std::option::Option<crate::types::RuleGroup>) -> Self {
        self.inner = self.inner.set_rule_group(input);
        self
    }
    /// <p>An object that defines the rule group rules.</p><note>
    /// <p>You must provide either this rule group setting or a <code>Rules</code> setting, but not both.</p>
    /// </note>
    pub fn get_rule_group(&self) -> &::std::option::Option<crate::types::RuleGroup> {
        self.inner.get_rule_group()
    }
    /// <p>A string containing stateful rule group rules specifications in Suricata flat format, with one rule per line. Use this to import your existing Suricata compatible rule groups.</p><note>
    /// <p>You must provide either this rules setting or a populated <code>RuleGroup</code> setting, but not both.</p>
    /// </note>
    /// <p>You can provide your rule group specification in Suricata flat format through this setting when you create or update your rule group. The call response returns a <code>RuleGroup</code> object that Network Firewall has populated from your string.</p>
    pub fn rules(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.rules(input.into());
        self
    }
    /// <p>A string containing stateful rule group rules specifications in Suricata flat format, with one rule per line. Use this to import your existing Suricata compatible rule groups.</p><note>
    /// <p>You must provide either this rules setting or a populated <code>RuleGroup</code> setting, but not both.</p>
    /// </note>
    /// <p>You can provide your rule group specification in Suricata flat format through this setting when you create or update your rule group. The call response returns a <code>RuleGroup</code> object that Network Firewall has populated from your string.</p>
    pub fn set_rules(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_rules(input);
        self
    }
    /// <p>A string containing stateful rule group rules specifications in Suricata flat format, with one rule per line. Use this to import your existing Suricata compatible rule groups.</p><note>
    /// <p>You must provide either this rules setting or a populated <code>RuleGroup</code> setting, but not both.</p>
    /// </note>
    /// <p>You can provide your rule group specification in Suricata flat format through this setting when you create or update your rule group. The call response returns a <code>RuleGroup</code> object that Network Firewall has populated from your string.</p>
    pub fn get_rules(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_rules()
    }
    /// <p>Indicates whether the rule group is stateless or stateful. If the rule group is stateless, it contains stateless rules. If it is stateful, it contains stateful rules.</p><note>
    /// <p>This setting is required for requests that do not include the <code>RuleGroupARN</code>.</p>
    /// </note>
    pub fn r#type(mut self, input: crate::types::RuleGroupType) -> Self {
        self.inner = self.inner.r#type(input);
        self
    }
    /// <p>Indicates whether the rule group is stateless or stateful. If the rule group is stateless, it contains stateless rules. If it is stateful, it contains stateful rules.</p><note>
    /// <p>This setting is required for requests that do not include the <code>RuleGroupARN</code>.</p>
    /// </note>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::RuleGroupType>) -> Self {
        self.inner = self.inner.set_type(input);
        self
    }
    /// <p>Indicates whether the rule group is stateless or stateful. If the rule group is stateless, it contains stateless rules. If it is stateful, it contains stateful rules.</p><note>
    /// <p>This setting is required for requests that do not include the <code>RuleGroupARN</code>.</p>
    /// </note>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::RuleGroupType> {
        self.inner.get_type()
    }
    /// <p>A description of the rule group.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.description(input.into());
        self
    }
    /// <p>A description of the rule group.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_description(input);
        self
    }
    /// <p>A description of the rule group.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_description()
    }
    /// <p>Indicates whether you want Network Firewall to just check the validity of the request, rather than run the request.</p>
    /// <p>If set to <code>TRUE</code>, Network Firewall checks whether the request can run successfully, but doesn't actually make the requested changes. The call returns the value that the request would return if you ran it with dry run set to <code>FALSE</code>, but doesn't make additions or changes to your resources. This option allows you to make sure that you have the required permissions to run the request and that your request parameters are valid.</p>
    /// <p>If set to <code>FALSE</code>, Network Firewall makes the requested changes to your resources.</p>
    pub fn dry_run(mut self, input: bool) -> Self {
        self.inner = self.inner.dry_run(input);
        self
    }
    /// <p>Indicates whether you want Network Firewall to just check the validity of the request, rather than run the request.</p>
    /// <p>If set to <code>TRUE</code>, Network Firewall checks whether the request can run successfully, but doesn't actually make the requested changes. The call returns the value that the request would return if you ran it with dry run set to <code>FALSE</code>, but doesn't make additions or changes to your resources. This option allows you to make sure that you have the required permissions to run the request and that your request parameters are valid.</p>
    /// <p>If set to <code>FALSE</code>, Network Firewall makes the requested changes to your resources.</p>
    pub fn set_dry_run(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_dry_run(input);
        self
    }
    /// <p>Indicates whether you want Network Firewall to just check the validity of the request, rather than run the request.</p>
    /// <p>If set to <code>TRUE</code>, Network Firewall checks whether the request can run successfully, but doesn't actually make the requested changes. The call returns the value that the request would return if you ran it with dry run set to <code>FALSE</code>, but doesn't make additions or changes to your resources. This option allows you to make sure that you have the required permissions to run the request and that your request parameters are valid.</p>
    /// <p>If set to <code>FALSE</code>, Network Firewall makes the requested changes to your resources.</p>
    pub fn get_dry_run(&self) -> &::std::option::Option<bool> {
        self.inner.get_dry_run()
    }
    /// <p>A complex type that contains settings for encryption of your rule group resources.</p>
    pub fn encryption_configuration(mut self, input: crate::types::EncryptionConfiguration) -> Self {
        self.inner = self.inner.encryption_configuration(input);
        self
    }
    /// <p>A complex type that contains settings for encryption of your rule group resources.</p>
    pub fn set_encryption_configuration(mut self, input: ::std::option::Option<crate::types::EncryptionConfiguration>) -> Self {
        self.inner = self.inner.set_encryption_configuration(input);
        self
    }
    /// <p>A complex type that contains settings for encryption of your rule group resources.</p>
    pub fn get_encryption_configuration(&self) -> &::std::option::Option<crate::types::EncryptionConfiguration> {
        self.inner.get_encryption_configuration()
    }
    /// <p>A complex type that contains metadata about the rule group that your own rule group is copied from. You can use the metadata to keep track of updates made to the originating rule group.</p>
    pub fn source_metadata(mut self, input: crate::types::SourceMetadata) -> Self {
        self.inner = self.inner.source_metadata(input);
        self
    }
    /// <p>A complex type that contains metadata about the rule group that your own rule group is copied from. You can use the metadata to keep track of updates made to the originating rule group.</p>
    pub fn set_source_metadata(mut self, input: ::std::option::Option<crate::types::SourceMetadata>) -> Self {
        self.inner = self.inner.set_source_metadata(input);
        self
    }
    /// <p>A complex type that contains metadata about the rule group that your own rule group is copied from. You can use the metadata to keep track of updates made to the originating rule group.</p>
    pub fn get_source_metadata(&self) -> &::std::option::Option<crate::types::SourceMetadata> {
        self.inner.get_source_metadata()
    }
    /// <p>Indicates whether you want Network Firewall to analyze the stateless rules in the rule group for rule behavior such as asymmetric routing. If set to <code>TRUE</code>, Network Firewall runs the analysis and then updates the rule group for you. To run the stateless rule group analyzer without updating the rule group, set <code>DryRun</code> to <code>TRUE</code>.</p>
    pub fn analyze_rule_group(mut self, input: bool) -> Self {
        self.inner = self.inner.analyze_rule_group(input);
        self
    }
    /// <p>Indicates whether you want Network Firewall to analyze the stateless rules in the rule group for rule behavior such as asymmetric routing. If set to <code>TRUE</code>, Network Firewall runs the analysis and then updates the rule group for you. To run the stateless rule group analyzer without updating the rule group, set <code>DryRun</code> to <code>TRUE</code>.</p>
    pub fn set_analyze_rule_group(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_analyze_rule_group(input);
        self
    }
    /// <p>Indicates whether you want Network Firewall to analyze the stateless rules in the rule group for rule behavior such as asymmetric routing. If set to <code>TRUE</code>, Network Firewall runs the analysis and then updates the rule group for you. To run the stateless rule group analyzer without updating the rule group, set <code>DryRun</code> to <code>TRUE</code>.</p>
    pub fn get_analyze_rule_group(&self) -> &::std::option::Option<bool> {
        self.inner.get_analyze_rule_group()
    }
    /// <p>Updates the selected summary configuration for a rule group.</p>
    /// <p>Changes affect subsequent responses from <code>DescribeRuleGroupSummary</code>.</p>
    pub fn summary_configuration(mut self, input: crate::types::SummaryConfiguration) -> Self {
        self.inner = self.inner.summary_configuration(input);
        self
    }
    /// <p>Updates the selected summary configuration for a rule group.</p>
    /// <p>Changes affect subsequent responses from <code>DescribeRuleGroupSummary</code>.</p>
    pub fn set_summary_configuration(mut self, input: ::std::option::Option<crate::types::SummaryConfiguration>) -> Self {
        self.inner = self.inner.set_summary_configuration(input);
        self
    }
    /// <p>Updates the selected summary configuration for a rule group.</p>
    /// <p>Changes affect subsequent responses from <code>DescribeRuleGroupSummary</code>.</p>
    pub fn get_summary_configuration(&self) -> &::std::option::Option<crate::types::SummaryConfiguration> {
        self.inner.get_summary_configuration()
    }
}
