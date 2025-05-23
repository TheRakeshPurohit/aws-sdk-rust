// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::batch_update_bill_scenario_commitment_modification::_batch_update_bill_scenario_commitment_modification_output::BatchUpdateBillScenarioCommitmentModificationOutputBuilder;

pub use crate::operation::batch_update_bill_scenario_commitment_modification::_batch_update_bill_scenario_commitment_modification_input::BatchUpdateBillScenarioCommitmentModificationInputBuilder;

impl crate::operation::batch_update_bill_scenario_commitment_modification::builders::BatchUpdateBillScenarioCommitmentModificationInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::batch_update_bill_scenario_commitment_modification::BatchUpdateBillScenarioCommitmentModificationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_update_bill_scenario_commitment_modification::BatchUpdateBillScenarioCommitmentModificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.batch_update_bill_scenario_commitment_modification();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `BatchUpdateBillScenarioCommitmentModification`.
///
/// <p>Update a newly added or existing commitment. You can update the commitment group based on a commitment ID and a Bill scenario ID.</p><note>
/// <p>The <code>BatchUpdateBillScenarioCommitmentModification</code> operation doesn't have its own IAM permission. To authorize this operation for Amazon Web Services principals, include the permission <code>bcm-pricing-calculator:UpdateBillScenarioCommitmentModification</code> in your policies.</p>
/// </note>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct BatchUpdateBillScenarioCommitmentModificationFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::batch_update_bill_scenario_commitment_modification::builders::BatchUpdateBillScenarioCommitmentModificationInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::batch_update_bill_scenario_commitment_modification::BatchUpdateBillScenarioCommitmentModificationOutput,
        crate::operation::batch_update_bill_scenario_commitment_modification::BatchUpdateBillScenarioCommitmentModificationError,
    > for BatchUpdateBillScenarioCommitmentModificationFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::batch_update_bill_scenario_commitment_modification::BatchUpdateBillScenarioCommitmentModificationOutput,
            crate::operation::batch_update_bill_scenario_commitment_modification::BatchUpdateBillScenarioCommitmentModificationError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl BatchUpdateBillScenarioCommitmentModificationFluentBuilder {
    /// Creates a new `BatchUpdateBillScenarioCommitmentModificationFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the BatchUpdateBillScenarioCommitmentModification as a reference.
    pub fn as_input(
        &self,
    ) -> &crate::operation::batch_update_bill_scenario_commitment_modification::builders::BatchUpdateBillScenarioCommitmentModificationInputBuilder
    {
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
        crate::operation::batch_update_bill_scenario_commitment_modification::BatchUpdateBillScenarioCommitmentModificationOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::batch_update_bill_scenario_commitment_modification::BatchUpdateBillScenarioCommitmentModificationError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::batch_update_bill_scenario_commitment_modification::BatchUpdateBillScenarioCommitmentModification::operation_runtime_plugins(
                            self.handle.runtime_plugins.clone(),
                            &self.handle.conf,
                            self.config_override,
                        );
        crate::operation::batch_update_bill_scenario_commitment_modification::BatchUpdateBillScenarioCommitmentModification::orchestrate(
            &runtime_plugins,
            input,
        )
        .await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::batch_update_bill_scenario_commitment_modification::BatchUpdateBillScenarioCommitmentModificationOutput,
        crate::operation::batch_update_bill_scenario_commitment_modification::BatchUpdateBillScenarioCommitmentModificationError,
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
    /// <p>The ID of the Bill Scenario for which you want to modify the commitment group of a modeled commitment.</p>
    pub fn bill_scenario_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.bill_scenario_id(input.into());
        self
    }
    /// <p>The ID of the Bill Scenario for which you want to modify the commitment group of a modeled commitment.</p>
    pub fn set_bill_scenario_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_bill_scenario_id(input);
        self
    }
    /// <p>The ID of the Bill Scenario for which you want to modify the commitment group of a modeled commitment.</p>
    pub fn get_bill_scenario_id(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_bill_scenario_id()
    }
    ///
    /// Appends an item to `commitmentModifications`.
    ///
    /// To override the contents of this collection use [`set_commitment_modifications`](Self::set_commitment_modifications).
    ///
    /// <p>List of commitments that you want to update in a Bill Scenario.</p>
    pub fn commitment_modifications(mut self, input: crate::types::BatchUpdateBillScenarioCommitmentModificationEntry) -> Self {
        self.inner = self.inner.commitment_modifications(input);
        self
    }
    /// <p>List of commitments that you want to update in a Bill Scenario.</p>
    pub fn set_commitment_modifications(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::BatchUpdateBillScenarioCommitmentModificationEntry>>,
    ) -> Self {
        self.inner = self.inner.set_commitment_modifications(input);
        self
    }
    /// <p>List of commitments that you want to update in a Bill Scenario.</p>
    pub fn get_commitment_modifications(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::BatchUpdateBillScenarioCommitmentModificationEntry>> {
        self.inner.get_commitment_modifications()
    }
}
