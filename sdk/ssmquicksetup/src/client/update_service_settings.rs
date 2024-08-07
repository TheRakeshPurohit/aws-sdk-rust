// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateServiceSettings`](crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`explorer_enabling_role_arn(impl Into<String>)`](crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder::explorer_enabling_role_arn) / [`set_explorer_enabling_role_arn(Option<String>)`](crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder::set_explorer_enabling_role_arn):<br>required: **false**<br><p>The IAM role used to enable Explorer.</p><br>
    /// - On success, responds with [`UpdateServiceSettingsOutput`](crate::operation::update_service_settings::UpdateServiceSettingsOutput)
    /// - On failure, responds with [`SdkError<UpdateServiceSettingsError>`](crate::operation::update_service_settings::UpdateServiceSettingsError)
    pub fn update_service_settings(&self) -> crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder {
        crate::operation::update_service_settings::builders::UpdateServiceSettingsFluentBuilder::new(self.handle.clone())
    }
}
