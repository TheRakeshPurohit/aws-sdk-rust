// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateOauth2CredentialProvider`](crate::operation::update_oauth2_credential_provider::builders::UpdateOauth2CredentialProviderFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`name(impl Into<String>)`](crate::operation::update_oauth2_credential_provider::builders::UpdateOauth2CredentialProviderFluentBuilder::name) / [`set_name(Option<String>)`](crate::operation::update_oauth2_credential_provider::builders::UpdateOauth2CredentialProviderFluentBuilder::set_name):<br>required: **true**<br><p>The name of the OAuth2 credential provider to update.</p><br>
    ///   - [`credential_provider_vendor(CredentialProviderVendorType)`](crate::operation::update_oauth2_credential_provider::builders::UpdateOauth2CredentialProviderFluentBuilder::credential_provider_vendor) / [`set_credential_provider_vendor(Option<CredentialProviderVendorType>)`](crate::operation::update_oauth2_credential_provider::builders::UpdateOauth2CredentialProviderFluentBuilder::set_credential_provider_vendor):<br>required: **true**<br><p>The vendor of the OAuth2 credential provider.</p><br>
    ///   - [`oauth2_provider_config_input(Oauth2ProviderConfigInput)`](crate::operation::update_oauth2_credential_provider::builders::UpdateOauth2CredentialProviderFluentBuilder::oauth2_provider_config_input) / [`set_oauth2_provider_config_input(Option<Oauth2ProviderConfigInput>)`](crate::operation::update_oauth2_credential_provider::builders::UpdateOauth2CredentialProviderFluentBuilder::set_oauth2_provider_config_input):<br>required: **true**<br><p>The configuration input for the OAuth2 provider.</p><br>
    /// - On success, responds with [`UpdateOauth2CredentialProviderOutput`](crate::operation::update_oauth2_credential_provider::UpdateOauth2CredentialProviderOutput) with field(s):
    ///   - [`client_secret_arn(Option<Secret>)`](crate::operation::update_oauth2_credential_provider::UpdateOauth2CredentialProviderOutput::client_secret_arn): <p>The Amazon Resource Name (ARN) of the client secret in AWS Secrets Manager.</p>
    ///   - [`name(String)`](crate::operation::update_oauth2_credential_provider::UpdateOauth2CredentialProviderOutput::name): <p>The name of the OAuth2 credential provider.</p>
    ///   - [`credential_provider_vendor(CredentialProviderVendorType)`](crate::operation::update_oauth2_credential_provider::UpdateOauth2CredentialProviderOutput::credential_provider_vendor): <p>The vendor of the OAuth2 credential provider.</p>
    ///   - [`credential_provider_arn(String)`](crate::operation::update_oauth2_credential_provider::UpdateOauth2CredentialProviderOutput::credential_provider_arn): <p>The Amazon Resource Name (ARN) of the OAuth2 credential provider.</p>
    ///   - [`oauth2_provider_config_output(Option<Oauth2ProviderConfigOutput>)`](crate::operation::update_oauth2_credential_provider::UpdateOauth2CredentialProviderOutput::oauth2_provider_config_output): <p>The configuration output for the OAuth2 provider.</p>
    ///   - [`created_time(DateTime)`](crate::operation::update_oauth2_credential_provider::UpdateOauth2CredentialProviderOutput::created_time): <p>The timestamp when the OAuth2 credential provider was created.</p>
    ///   - [`last_updated_time(DateTime)`](crate::operation::update_oauth2_credential_provider::UpdateOauth2CredentialProviderOutput::last_updated_time): <p>The timestamp when the OAuth2 credential provider was last updated.</p>
    /// - On failure, responds with [`SdkError<UpdateOauth2CredentialProviderError>`](crate::operation::update_oauth2_credential_provider::UpdateOauth2CredentialProviderError)
    pub fn update_oauth2_credential_provider(
        &self,
    ) -> crate::operation::update_oauth2_credential_provider::builders::UpdateOauth2CredentialProviderFluentBuilder {
        crate::operation::update_oauth2_credential_provider::builders::UpdateOauth2CredentialProviderFluentBuilder::new(self.handle.clone())
    }
}
