// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`AssociateWhatsAppBusinessAccount`](crate::operation::associate_whats_app_business_account::builders::AssociateWhatsAppBusinessAccountFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`signup_callback(WhatsAppSignupCallback)`](crate::operation::associate_whats_app_business_account::builders::AssociateWhatsAppBusinessAccountFluentBuilder::signup_callback) / [`set_signup_callback(Option<WhatsAppSignupCallback>)`](crate::operation::associate_whats_app_business_account::builders::AssociateWhatsAppBusinessAccountFluentBuilder::set_signup_callback):<br>required: **false**<br><p>Contains the callback access token.</p><br>
    ///   - [`setup_finalization(WhatsAppSetupFinalization)`](crate::operation::associate_whats_app_business_account::builders::AssociateWhatsAppBusinessAccountFluentBuilder::setup_finalization) / [`set_setup_finalization(Option<WhatsAppSetupFinalization>)`](crate::operation::associate_whats_app_business_account::builders::AssociateWhatsAppBusinessAccountFluentBuilder::set_setup_finalization):<br>required: **false**<br><p>A JSON object that contains the phone numbers and WhatsApp Business Account to link to your account.</p><br>
    /// - On success, responds with [`AssociateWhatsAppBusinessAccountOutput`](crate::operation::associate_whats_app_business_account::AssociateWhatsAppBusinessAccountOutput) with field(s):
    ///   - [`signup_callback_result(Option<WhatsAppSignupCallbackResult>)`](crate::operation::associate_whats_app_business_account::AssociateWhatsAppBusinessAccountOutput::signup_callback_result): <p>Contains your WhatsApp registration status.</p>
    ///   - [`status_code(Option<i32>)`](crate::operation::associate_whats_app_business_account::AssociateWhatsAppBusinessAccountOutput::status_code): <p>The status code for the response.</p>
    /// - On failure, responds with [`SdkError<AssociateWhatsAppBusinessAccountError>`](crate::operation::associate_whats_app_business_account::AssociateWhatsAppBusinessAccountError)
    pub fn associate_whats_app_business_account(
        &self,
    ) -> crate::operation::associate_whats_app_business_account::builders::AssociateWhatsAppBusinessAccountFluentBuilder {
        crate::operation::associate_whats_app_business_account::builders::AssociateWhatsAppBusinessAccountFluentBuilder::new(self.handle.clone())
    }
}
