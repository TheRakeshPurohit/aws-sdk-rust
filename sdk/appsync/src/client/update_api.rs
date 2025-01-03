// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateApi`](crate::operation::update_api::builders::UpdateApiFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`api_id(impl Into<String>)`](crate::operation::update_api::builders::UpdateApiFluentBuilder::api_id) / [`set_api_id(Option<String>)`](crate::operation::update_api::builders::UpdateApiFluentBuilder::set_api_id):<br>required: **true**<br><p>The <code>Api</code> ID.</p><br>
    ///   - [`name(impl Into<String>)`](crate::operation::update_api::builders::UpdateApiFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_api::builders::UpdateApiFluentBuilder::set_name):<br>required: **true**<br><p>The name of the Api.</p><br>
    ///   - [`owner_contact(impl Into<String>)`](crate::operation::update_api::builders::UpdateApiFluentBuilder::owner_contact) / [`set_owner_contact(Option<String>)`](crate::operation::update_api::builders::UpdateApiFluentBuilder::set_owner_contact):<br>required: **false**<br><p>The owner contact information for the <code>Api</code>.</p><br>
    ///   - [`event_config(EventConfig)`](crate::operation::update_api::builders::UpdateApiFluentBuilder::event_config) / [`set_event_config(Option<EventConfig>)`](crate::operation::update_api::builders::UpdateApiFluentBuilder::set_event_config):<br>required: **false**<br><p>The new event configuration. This includes the default authorization configuration for connecting, publishing, and subscribing to an Event API.</p><br>
    /// - On success, responds with [`UpdateApiOutput`](crate::operation::update_api::UpdateApiOutput) with field(s):
    ///   - [`api(Option<Api>)`](crate::operation::update_api::UpdateApiOutput::api): <p>The <code>Api</code> object.</p>
    /// - On failure, responds with [`SdkError<UpdateApiError>`](crate::operation::update_api::UpdateApiError)
    pub fn update_api(&self) -> crate::operation::update_api::builders::UpdateApiFluentBuilder {
        crate::operation::update_api::builders::UpdateApiFluentBuilder::new(self.handle.clone())
    }
}
