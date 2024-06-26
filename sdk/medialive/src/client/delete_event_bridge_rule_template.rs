// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteEventBridgeRuleTemplate`](crate::operation::delete_event_bridge_rule_template::builders::DeleteEventBridgeRuleTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`identifier(impl Into<String>)`](crate::operation::delete_event_bridge_rule_template::builders::DeleteEventBridgeRuleTemplateFluentBuilder::identifier) / [`set_identifier(Option<String>)`](crate::operation::delete_event_bridge_rule_template::builders::DeleteEventBridgeRuleTemplateFluentBuilder::set_identifier):<br>required: **true**<br>An eventbridge rule template's identifier. Can be either be its id or current name.<br>
    /// - On success, responds with [`DeleteEventBridgeRuleTemplateOutput`](crate::operation::delete_event_bridge_rule_template::DeleteEventBridgeRuleTemplateOutput)
    /// - On failure, responds with [`SdkError<DeleteEventBridgeRuleTemplateError>`](crate::operation::delete_event_bridge_rule_template::DeleteEventBridgeRuleTemplateError)
    pub fn delete_event_bridge_rule_template(
        &self,
    ) -> crate::operation::delete_event_bridge_rule_template::builders::DeleteEventBridgeRuleTemplateFluentBuilder {
        crate::operation::delete_event_bridge_rule_template::builders::DeleteEventBridgeRuleTemplateFluentBuilder::new(self.handle.clone())
    }
}
