// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetNotificationConfiguration`](crate::operation::get_notification_configuration::builders::GetNotificationConfigurationFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`event_type(EventType)`](crate::operation::get_notification_configuration::builders::GetNotificationConfigurationFluentBuilder::event_type) / [`set_event_type(Option<EventType>)`](crate::operation::get_notification_configuration::builders::GetNotificationConfigurationFluentBuilder::set_event_type):<br>required: **true**<br><p>The type of event triggering a device notification to the customer-managed destination.</p><br>
    /// - On success, responds with [`GetNotificationConfigurationOutput`](crate::operation::get_notification_configuration::GetNotificationConfigurationOutput) with field(s):
    ///   - [`event_type(Option<EventType>)`](crate::operation::get_notification_configuration::GetNotificationConfigurationOutput::event_type): <p>The type of event triggering a device notification to the customer-managed destination.</p>
    ///   - [`destination_name(Option<String>)`](crate::operation::get_notification_configuration::GetNotificationConfigurationOutput::destination_name): <p>The name of the destination for the notification configuration.</p>
    ///   - [`created_at(Option<DateTime>)`](crate::operation::get_notification_configuration::GetNotificationConfigurationOutput::created_at): <p>The timestamp value of when the notification configuration was created.</p>
    ///   - [`updated_at(Option<DateTime>)`](crate::operation::get_notification_configuration::GetNotificationConfigurationOutput::updated_at): <p>The timestamp value of when the notification configuration was last updated.</p>
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::get_notification_configuration::GetNotificationConfigurationOutput::tags): <p>A set of key/value pairs that are used to manage the notification configuration.</p>
    /// - On failure, responds with [`SdkError<GetNotificationConfigurationError>`](crate::operation::get_notification_configuration::GetNotificationConfigurationError)
    pub fn get_notification_configuration(
        &self,
    ) -> crate::operation::get_notification_configuration::builders::GetNotificationConfigurationFluentBuilder {
        crate::operation::get_notification_configuration::builders::GetNotificationConfigurationFluentBuilder::new(self.handle.clone())
    }
}
