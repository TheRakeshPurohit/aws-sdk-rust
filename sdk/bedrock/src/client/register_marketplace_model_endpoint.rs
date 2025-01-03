// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`RegisterMarketplaceModelEndpoint`](crate::operation::register_marketplace_model_endpoint::builders::RegisterMarketplaceModelEndpointFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`endpoint_identifier(impl Into<String>)`](crate::operation::register_marketplace_model_endpoint::builders::RegisterMarketplaceModelEndpointFluentBuilder::endpoint_identifier) / [`set_endpoint_identifier(Option<String>)`](crate::operation::register_marketplace_model_endpoint::builders::RegisterMarketplaceModelEndpointFluentBuilder::set_endpoint_identifier):<br>required: **true**<br><p>The ARN of the Amazon SageMaker endpoint you want to register with Amazon Bedrock Marketplace.</p><br>
    ///   - [`model_source_identifier(impl Into<String>)`](crate::operation::register_marketplace_model_endpoint::builders::RegisterMarketplaceModelEndpointFluentBuilder::model_source_identifier) / [`set_model_source_identifier(Option<String>)`](crate::operation::register_marketplace_model_endpoint::builders::RegisterMarketplaceModelEndpointFluentBuilder::set_model_source_identifier):<br>required: **true**<br><p>The ARN of the model from Amazon Bedrock Marketplace that is deployed on the endpoint.</p><br>
    /// - On success, responds with [`RegisterMarketplaceModelEndpointOutput`](crate::operation::register_marketplace_model_endpoint::RegisterMarketplaceModelEndpointOutput) with field(s):
    ///   - [`marketplace_model_endpoint(Option<MarketplaceModelEndpoint>)`](crate::operation::register_marketplace_model_endpoint::RegisterMarketplaceModelEndpointOutput::marketplace_model_endpoint): <p>Details about the registered endpoint.</p>
    /// - On failure, responds with [`SdkError<RegisterMarketplaceModelEndpointError>`](crate::operation::register_marketplace_model_endpoint::RegisterMarketplaceModelEndpointError)
    pub fn register_marketplace_model_endpoint(
        &self,
    ) -> crate::operation::register_marketplace_model_endpoint::builders::RegisterMarketplaceModelEndpointFluentBuilder {
        crate::operation::register_marketplace_model_endpoint::builders::RegisterMarketplaceModelEndpointFluentBuilder::new(self.handle.clone())
    }
}
