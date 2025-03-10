// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::create_opportunity::_create_opportunity_output::CreateOpportunityOutputBuilder;

pub use crate::operation::create_opportunity::_create_opportunity_input::CreateOpportunityInputBuilder;

impl crate::operation::create_opportunity::builders::CreateOpportunityInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::create_opportunity::CreateOpportunityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_opportunity::CreateOpportunityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.create_opportunity();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `CreateOpportunity`.
///
/// <p>Creates an <code>Opportunity</code> record in Partner Central. Use this operation to create a potential business opportunity for submission to Amazon Web Services. Creating an opportunity sets <code>Lifecycle.ReviewStatus</code> to <code>Pending Submission</code>.</p>
/// <p>To submit an opportunity, follow these steps:</p>
/// <ol>
/// <li>
/// <p>To create the opportunity, use <code>CreateOpportunity</code>.</p></li>
/// <li>
/// <p>To associate a solution with the opportunity, use <code>AssociateOpportunity</code>.</p></li>
/// <li>
/// <p>To start the engagement with AWS, use <code>StartEngagementFromOpportunity</code>.</p></li>
/// </ol>
/// <p>After submission, you can't edit the opportunity until the review is complete. But opportunities in the <code>Pending Submission</code> state must have complete details. You can update the opportunity while it's in the <code>Pending Submission</code> state.</p>
/// <p>There's a set of mandatory fields to create opportunities, but consider providing optional fields to enrich the opportunity record.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct CreateOpportunityFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::create_opportunity::builders::CreateOpportunityInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::create_opportunity::CreateOpportunityOutput,
        crate::operation::create_opportunity::CreateOpportunityError,
    > for CreateOpportunityFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::create_opportunity::CreateOpportunityOutput,
            crate::operation::create_opportunity::CreateOpportunityError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl CreateOpportunityFluentBuilder {
    /// Creates a new `CreateOpportunityFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the CreateOpportunity as a reference.
    pub fn as_input(&self) -> &crate::operation::create_opportunity::builders::CreateOpportunityInputBuilder {
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
        crate::operation::create_opportunity::CreateOpportunityOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::create_opportunity::CreateOpportunityError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::create_opportunity::CreateOpportunity::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::create_opportunity::CreateOpportunity::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::create_opportunity::CreateOpportunityOutput,
        crate::operation::create_opportunity::CreateOpportunityError,
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
    /// <p>Specifies the catalog associated with the request. This field takes a string value from a predefined list: <code>AWS</code> or <code>Sandbox</code>. The catalog determines which environment the opportunity is created in. Use <code>AWS</code> to create opportunities in the Amazon Web Services catalog, and <code>Sandbox</code> for testing in secure, isolated environments.</p>
    pub fn catalog(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.catalog(input.into());
        self
    }
    /// <p>Specifies the catalog associated with the request. This field takes a string value from a predefined list: <code>AWS</code> or <code>Sandbox</code>. The catalog determines which environment the opportunity is created in. Use <code>AWS</code> to create opportunities in the Amazon Web Services catalog, and <code>Sandbox</code> for testing in secure, isolated environments.</p>
    pub fn set_catalog(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_catalog(input);
        self
    }
    /// <p>Specifies the catalog associated with the request. This field takes a string value from a predefined list: <code>AWS</code> or <code>Sandbox</code>. The catalog determines which environment the opportunity is created in. Use <code>AWS</code> to create opportunities in the Amazon Web Services catalog, and <code>Sandbox</code> for testing in secure, isolated environments.</p>
    pub fn get_catalog(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_catalog()
    }
    ///
    /// Appends an item to `PrimaryNeedsFromAws`.
    ///
    /// To override the contents of this collection use [`set_primary_needs_from_aws`](Self::set_primary_needs_from_aws).
    ///
    /// <p>Identifies the type of support the partner needs from Amazon Web Services.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li>
    /// <p>Cosell—Architectural Validation: Confirmation from Amazon Web Services that the partner's proposed solution architecture is aligned with Amazon Web Services best practices and poses minimal architectural risks.</p></li>
    /// <li>
    /// <p>Cosell—Business Presentation: Request Amazon Web Services seller's participation in a joint customer presentation.</p></li>
    /// <li>
    /// <p>Cosell—Competitive Information: Access to Amazon Web Services competitive resources and support for the partner's proposed solution.</p></li>
    /// <li>
    /// <p>Cosell—Pricing Assistance: Connect with an Amazon Web Services seller for support situations where a partner may be receiving an upfront discount on a service (for example: EDP deals).</p></li>
    /// <li>
    /// <p>Cosell—Technical Consultation: Connect with an Amazon Web Services Solutions Architect to address the partner's questions about the proposed solution.</p></li>
    /// <li>
    /// <p>Cosell—Total Cost of Ownership Evaluation: Assistance with quoting different cost savings of proposed solutions on Amazon Web Services versus on-premises or a traditional hosting environment.</p></li>
    /// <li>
    /// <p>Cosell—Deal Support: Request Amazon Web Services seller's support to progress the opportunity (for example: joint customer call, strategic positioning).</p></li>
    /// <li>
    /// <p>Cosell—Support for Public Tender/RFx: Opportunity related to the public sector where the partner needs Amazon Web Services RFx support.</p></li>
    /// </ul>
    pub fn primary_needs_from_aws(mut self, input: crate::types::PrimaryNeedFromAws) -> Self {
        self.inner = self.inner.primary_needs_from_aws(input);
        self
    }
    /// <p>Identifies the type of support the partner needs from Amazon Web Services.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li>
    /// <p>Cosell—Architectural Validation: Confirmation from Amazon Web Services that the partner's proposed solution architecture is aligned with Amazon Web Services best practices and poses minimal architectural risks.</p></li>
    /// <li>
    /// <p>Cosell—Business Presentation: Request Amazon Web Services seller's participation in a joint customer presentation.</p></li>
    /// <li>
    /// <p>Cosell—Competitive Information: Access to Amazon Web Services competitive resources and support for the partner's proposed solution.</p></li>
    /// <li>
    /// <p>Cosell—Pricing Assistance: Connect with an Amazon Web Services seller for support situations where a partner may be receiving an upfront discount on a service (for example: EDP deals).</p></li>
    /// <li>
    /// <p>Cosell—Technical Consultation: Connect with an Amazon Web Services Solutions Architect to address the partner's questions about the proposed solution.</p></li>
    /// <li>
    /// <p>Cosell—Total Cost of Ownership Evaluation: Assistance with quoting different cost savings of proposed solutions on Amazon Web Services versus on-premises or a traditional hosting environment.</p></li>
    /// <li>
    /// <p>Cosell—Deal Support: Request Amazon Web Services seller's support to progress the opportunity (for example: joint customer call, strategic positioning).</p></li>
    /// <li>
    /// <p>Cosell—Support for Public Tender/RFx: Opportunity related to the public sector where the partner needs Amazon Web Services RFx support.</p></li>
    /// </ul>
    pub fn set_primary_needs_from_aws(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::PrimaryNeedFromAws>>) -> Self {
        self.inner = self.inner.set_primary_needs_from_aws(input);
        self
    }
    /// <p>Identifies the type of support the partner needs from Amazon Web Services.</p>
    /// <p>Valid values:</p>
    /// <ul>
    /// <li>
    /// <p>Cosell—Architectural Validation: Confirmation from Amazon Web Services that the partner's proposed solution architecture is aligned with Amazon Web Services best practices and poses minimal architectural risks.</p></li>
    /// <li>
    /// <p>Cosell—Business Presentation: Request Amazon Web Services seller's participation in a joint customer presentation.</p></li>
    /// <li>
    /// <p>Cosell—Competitive Information: Access to Amazon Web Services competitive resources and support for the partner's proposed solution.</p></li>
    /// <li>
    /// <p>Cosell—Pricing Assistance: Connect with an Amazon Web Services seller for support situations where a partner may be receiving an upfront discount on a service (for example: EDP deals).</p></li>
    /// <li>
    /// <p>Cosell—Technical Consultation: Connect with an Amazon Web Services Solutions Architect to address the partner's questions about the proposed solution.</p></li>
    /// <li>
    /// <p>Cosell—Total Cost of Ownership Evaluation: Assistance with quoting different cost savings of proposed solutions on Amazon Web Services versus on-premises or a traditional hosting environment.</p></li>
    /// <li>
    /// <p>Cosell—Deal Support: Request Amazon Web Services seller's support to progress the opportunity (for example: joint customer call, strategic positioning).</p></li>
    /// <li>
    /// <p>Cosell—Support for Public Tender/RFx: Opportunity related to the public sector where the partner needs Amazon Web Services RFx support.</p></li>
    /// </ul>
    pub fn get_primary_needs_from_aws(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::PrimaryNeedFromAws>> {
        self.inner.get_primary_needs_from_aws()
    }
    /// <p>Indicates whether the <code>Opportunity</code> pertains to a national security project. This field must be set to <code>true</code> only when the customer's industry is <i>Government</i>. Additional privacy and security measures apply during the review and management process for opportunities marked as <code>NationalSecurity</code>.</p>
    pub fn national_security(mut self, input: crate::types::NationalSecurity) -> Self {
        self.inner = self.inner.national_security(input);
        self
    }
    /// <p>Indicates whether the <code>Opportunity</code> pertains to a national security project. This field must be set to <code>true</code> only when the customer's industry is <i>Government</i>. Additional privacy and security measures apply during the review and management process for opportunities marked as <code>NationalSecurity</code>.</p>
    pub fn set_national_security(mut self, input: ::std::option::Option<crate::types::NationalSecurity>) -> Self {
        self.inner = self.inner.set_national_security(input);
        self
    }
    /// <p>Indicates whether the <code>Opportunity</code> pertains to a national security project. This field must be set to <code>true</code> only when the customer's industry is <i>Government</i>. Additional privacy and security measures apply during the review and management process for opportunities marked as <code>NationalSecurity</code>.</p>
    pub fn get_national_security(&self) -> &::std::option::Option<crate::types::NationalSecurity> {
        self.inner.get_national_security()
    }
    /// <p>Specifies the opportunity's unique identifier in the partner's CRM system. This value is essential to track and reconcile because it's included in the outbound payload to the partner.</p>
    /// <p>This field allows partners to link an opportunity to their CRM, which helps to ensure seamless integration and accurate synchronization between the Partner Central API and the partner's internal systems.</p>
    pub fn partner_opportunity_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.partner_opportunity_identifier(input.into());
        self
    }
    /// <p>Specifies the opportunity's unique identifier in the partner's CRM system. This value is essential to track and reconcile because it's included in the outbound payload to the partner.</p>
    /// <p>This field allows partners to link an opportunity to their CRM, which helps to ensure seamless integration and accurate synchronization between the Partner Central API and the partner's internal systems.</p>
    pub fn set_partner_opportunity_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_partner_opportunity_identifier(input);
        self
    }
    /// <p>Specifies the opportunity's unique identifier in the partner's CRM system. This value is essential to track and reconcile because it's included in the outbound payload to the partner.</p>
    /// <p>This field allows partners to link an opportunity to their CRM, which helps to ensure seamless integration and accurate synchronization between the Partner Central API and the partner's internal systems.</p>
    pub fn get_partner_opportunity_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_partner_opportunity_identifier()
    }
    /// <p>Specifies customer details associated with the <code>Opportunity</code>.</p>
    pub fn customer(mut self, input: crate::types::Customer) -> Self {
        self.inner = self.inner.customer(input);
        self
    }
    /// <p>Specifies customer details associated with the <code>Opportunity</code>.</p>
    pub fn set_customer(mut self, input: ::std::option::Option<crate::types::Customer>) -> Self {
        self.inner = self.inner.set_customer(input);
        self
    }
    /// <p>Specifies customer details associated with the <code>Opportunity</code>.</p>
    pub fn get_customer(&self) -> &::std::option::Option<crate::types::Customer> {
        self.inner.get_customer()
    }
    /// <p>An object that contains project details for the <code>Opportunity</code>.</p>
    pub fn project(mut self, input: crate::types::Project) -> Self {
        self.inner = self.inner.project(input);
        self
    }
    /// <p>An object that contains project details for the <code>Opportunity</code>.</p>
    pub fn set_project(mut self, input: ::std::option::Option<crate::types::Project>) -> Self {
        self.inner = self.inner.set_project(input);
        self
    }
    /// <p>An object that contains project details for the <code>Opportunity</code>.</p>
    pub fn get_project(&self) -> &::std::option::Option<crate::types::Project> {
        self.inner.get_project()
    }
    /// <p>Specifies the opportunity type as a renewal, new, or expansion.</p>
    /// <p>Opportunity types:</p>
    /// <ul>
    /// <li>
    /// <p>New opportunity: Represents a new business opportunity with a potential customer that's not previously engaged with your solutions or services.</p></li>
    /// <li>
    /// <p>Renewal opportunity: Represents an opportunity to renew an existing contract or subscription with a current customer, ensuring continuity of service.</p></li>
    /// <li>
    /// <p>Expansion opportunity: Represents an opportunity to expand the scope of an existing contract or subscription, either by adding new services or increasing the volume of existing services for a current customer.</p></li>
    /// </ul>
    pub fn opportunity_type(mut self, input: crate::types::OpportunityType) -> Self {
        self.inner = self.inner.opportunity_type(input);
        self
    }
    /// <p>Specifies the opportunity type as a renewal, new, or expansion.</p>
    /// <p>Opportunity types:</p>
    /// <ul>
    /// <li>
    /// <p>New opportunity: Represents a new business opportunity with a potential customer that's not previously engaged with your solutions or services.</p></li>
    /// <li>
    /// <p>Renewal opportunity: Represents an opportunity to renew an existing contract or subscription with a current customer, ensuring continuity of service.</p></li>
    /// <li>
    /// <p>Expansion opportunity: Represents an opportunity to expand the scope of an existing contract or subscription, either by adding new services or increasing the volume of existing services for a current customer.</p></li>
    /// </ul>
    pub fn set_opportunity_type(mut self, input: ::std::option::Option<crate::types::OpportunityType>) -> Self {
        self.inner = self.inner.set_opportunity_type(input);
        self
    }
    /// <p>Specifies the opportunity type as a renewal, new, or expansion.</p>
    /// <p>Opportunity types:</p>
    /// <ul>
    /// <li>
    /// <p>New opportunity: Represents a new business opportunity with a potential customer that's not previously engaged with your solutions or services.</p></li>
    /// <li>
    /// <p>Renewal opportunity: Represents an opportunity to renew an existing contract or subscription with a current customer, ensuring continuity of service.</p></li>
    /// <li>
    /// <p>Expansion opportunity: Represents an opportunity to expand the scope of an existing contract or subscription, either by adding new services or increasing the volume of existing services for a current customer.</p></li>
    /// </ul>
    pub fn get_opportunity_type(&self) -> &::std::option::Option<crate::types::OpportunityType> {
        self.inner.get_opportunity_type()
    }
    /// <p>This object contains marketing details and is optional for an opportunity.</p>
    pub fn marketing(mut self, input: crate::types::Marketing) -> Self {
        self.inner = self.inner.marketing(input);
        self
    }
    /// <p>This object contains marketing details and is optional for an opportunity.</p>
    pub fn set_marketing(mut self, input: ::std::option::Option<crate::types::Marketing>) -> Self {
        self.inner = self.inner.set_marketing(input);
        self
    }
    /// <p>This object contains marketing details and is optional for an opportunity.</p>
    pub fn get_marketing(&self) -> &::std::option::Option<crate::types::Marketing> {
        self.inner.get_marketing()
    }
    /// <p>Specifies details of a customer's procurement terms. This is required only for partners in eligible programs.</p>
    pub fn software_revenue(mut self, input: crate::types::SoftwareRevenue) -> Self {
        self.inner = self.inner.software_revenue(input);
        self
    }
    /// <p>Specifies details of a customer's procurement terms. This is required only for partners in eligible programs.</p>
    pub fn set_software_revenue(mut self, input: ::std::option::Option<crate::types::SoftwareRevenue>) -> Self {
        self.inner = self.inner.set_software_revenue(input);
        self
    }
    /// <p>Specifies details of a customer's procurement terms. This is required only for partners in eligible programs.</p>
    pub fn get_software_revenue(&self) -> &::std::option::Option<crate::types::SoftwareRevenue> {
        self.inner.get_software_revenue()
    }
    /// <p>Required to be unique, and should be unchanging, it can be randomly generated or a meaningful string.</p>
    /// <p>Default: None</p>
    /// <p>Best practice: To help ensure uniqueness and avoid conflicts, use a Universally Unique Identifier (UUID) as the <code>ClientToken</code>. You can use standard libraries from most programming languages to generate this. If you use the same client token, the API returns the following error: "Conflicting client token submitted for a new request body."</p>
    pub fn client_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.client_token(input.into());
        self
    }
    /// <p>Required to be unique, and should be unchanging, it can be randomly generated or a meaningful string.</p>
    /// <p>Default: None</p>
    /// <p>Best practice: To help ensure uniqueness and avoid conflicts, use a Universally Unique Identifier (UUID) as the <code>ClientToken</code>. You can use standard libraries from most programming languages to generate this. If you use the same client token, the API returns the following error: "Conflicting client token submitted for a new request body."</p>
    pub fn set_client_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_client_token(input);
        self
    }
    /// <p>Required to be unique, and should be unchanging, it can be randomly generated or a meaningful string.</p>
    /// <p>Default: None</p>
    /// <p>Best practice: To help ensure uniqueness and avoid conflicts, use a Universally Unique Identifier (UUID) as the <code>ClientToken</code>. You can use standard libraries from most programming languages to generate this. If you use the same client token, the API returns the following error: "Conflicting client token submitted for a new request body."</p>
    pub fn get_client_token(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_client_token()
    }
    /// <p>An object that contains lifecycle details for the <code>Opportunity</code>.</p>
    pub fn life_cycle(mut self, input: crate::types::LifeCycle) -> Self {
        self.inner = self.inner.life_cycle(input);
        self
    }
    /// <p>An object that contains lifecycle details for the <code>Opportunity</code>.</p>
    pub fn set_life_cycle(mut self, input: ::std::option::Option<crate::types::LifeCycle>) -> Self {
        self.inner = self.inner.set_life_cycle(input);
        self
    }
    /// <p>An object that contains lifecycle details for the <code>Opportunity</code>.</p>
    pub fn get_life_cycle(&self) -> &::std::option::Option<crate::types::LifeCycle> {
        self.inner.get_life_cycle()
    }
    /// <p>Specifies the origin of the opportunity, indicating if it was sourced from Amazon Web Services or the partner. For all opportunities created with <code>Catalog: AWS</code>, this field must only be <code>Partner Referral</code>. However, when using <code>Catalog: Sandbox</code>, you can set this field to <code>AWS Referral</code> to simulate Amazon Web Services referral creation. This allows Amazon Web Services-originated flows testing in the sandbox catalog.</p>
    pub fn origin(mut self, input: crate::types::OpportunityOrigin) -> Self {
        self.inner = self.inner.origin(input);
        self
    }
    /// <p>Specifies the origin of the opportunity, indicating if it was sourced from Amazon Web Services or the partner. For all opportunities created with <code>Catalog: AWS</code>, this field must only be <code>Partner Referral</code>. However, when using <code>Catalog: Sandbox</code>, you can set this field to <code>AWS Referral</code> to simulate Amazon Web Services referral creation. This allows Amazon Web Services-originated flows testing in the sandbox catalog.</p>
    pub fn set_origin(mut self, input: ::std::option::Option<crate::types::OpportunityOrigin>) -> Self {
        self.inner = self.inner.set_origin(input);
        self
    }
    /// <p>Specifies the origin of the opportunity, indicating if it was sourced from Amazon Web Services or the partner. For all opportunities created with <code>Catalog: AWS</code>, this field must only be <code>Partner Referral</code>. However, when using <code>Catalog: Sandbox</code>, you can set this field to <code>AWS Referral</code> to simulate Amazon Web Services referral creation. This allows Amazon Web Services-originated flows testing in the sandbox catalog.</p>
    pub fn get_origin(&self) -> &::std::option::Option<crate::types::OpportunityOrigin> {
        self.inner.get_origin()
    }
    ///
    /// Appends an item to `OpportunityTeam`.
    ///
    /// To override the contents of this collection use [`set_opportunity_team`](Self::set_opportunity_team).
    ///
    /// <p>Represents the internal team handling the opportunity. Specify collaborating members of this opportunity who are within the partner's organization.</p>
    pub fn opportunity_team(mut self, input: crate::types::Contact) -> Self {
        self.inner = self.inner.opportunity_team(input);
        self
    }
    /// <p>Represents the internal team handling the opportunity. Specify collaborating members of this opportunity who are within the partner's organization.</p>
    pub fn set_opportunity_team(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::Contact>>) -> Self {
        self.inner = self.inner.set_opportunity_team(input);
        self
    }
    /// <p>Represents the internal team handling the opportunity. Specify collaborating members of this opportunity who are within the partner's organization.</p>
    pub fn get_opportunity_team(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::Contact>> {
        self.inner.get_opportunity_team()
    }
}
