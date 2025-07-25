// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetCloudExadataInfrastructureUnallocatedResourcesOutput {
    /// <p>Details about the unallocated resources in the specified Cloud Exadata infrastructure.</p>
    pub cloud_exadata_infrastructure_unallocated_resources: ::std::option::Option<crate::types::CloudExadataInfrastructureUnallocatedResources>,
    _request_id: Option<String>,
}
impl GetCloudExadataInfrastructureUnallocatedResourcesOutput {
    /// <p>Details about the unallocated resources in the specified Cloud Exadata infrastructure.</p>
    pub fn cloud_exadata_infrastructure_unallocated_resources(
        &self,
    ) -> ::std::option::Option<&crate::types::CloudExadataInfrastructureUnallocatedResources> {
        self.cloud_exadata_infrastructure_unallocated_resources.as_ref()
    }
}
impl ::aws_types::request_id::RequestId for GetCloudExadataInfrastructureUnallocatedResourcesOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl GetCloudExadataInfrastructureUnallocatedResourcesOutput {
    /// Creates a new builder-style object to manufacture [`GetCloudExadataInfrastructureUnallocatedResourcesOutput`](crate::operation::get_cloud_exadata_infrastructure_unallocated_resources::GetCloudExadataInfrastructureUnallocatedResourcesOutput).
    pub fn builder() -> crate::operation::get_cloud_exadata_infrastructure_unallocated_resources::builders::GetCloudExadataInfrastructureUnallocatedResourcesOutputBuilder{
        crate::operation::get_cloud_exadata_infrastructure_unallocated_resources::builders::GetCloudExadataInfrastructureUnallocatedResourcesOutputBuilder::default()
    }
}

/// A builder for [`GetCloudExadataInfrastructureUnallocatedResourcesOutput`](crate::operation::get_cloud_exadata_infrastructure_unallocated_resources::GetCloudExadataInfrastructureUnallocatedResourcesOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetCloudExadataInfrastructureUnallocatedResourcesOutputBuilder {
    pub(crate) cloud_exadata_infrastructure_unallocated_resources:
        ::std::option::Option<crate::types::CloudExadataInfrastructureUnallocatedResources>,
    _request_id: Option<String>,
}
impl GetCloudExadataInfrastructureUnallocatedResourcesOutputBuilder {
    /// <p>Details about the unallocated resources in the specified Cloud Exadata infrastructure.</p>
    pub fn cloud_exadata_infrastructure_unallocated_resources(mut self, input: crate::types::CloudExadataInfrastructureUnallocatedResources) -> Self {
        self.cloud_exadata_infrastructure_unallocated_resources = ::std::option::Option::Some(input);
        self
    }
    /// <p>Details about the unallocated resources in the specified Cloud Exadata infrastructure.</p>
    pub fn set_cloud_exadata_infrastructure_unallocated_resources(
        mut self,
        input: ::std::option::Option<crate::types::CloudExadataInfrastructureUnallocatedResources>,
    ) -> Self {
        self.cloud_exadata_infrastructure_unallocated_resources = input;
        self
    }
    /// <p>Details about the unallocated resources in the specified Cloud Exadata infrastructure.</p>
    pub fn get_cloud_exadata_infrastructure_unallocated_resources(
        &self,
    ) -> &::std::option::Option<crate::types::CloudExadataInfrastructureUnallocatedResources> {
        &self.cloud_exadata_infrastructure_unallocated_resources
    }
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`GetCloudExadataInfrastructureUnallocatedResourcesOutput`](crate::operation::get_cloud_exadata_infrastructure_unallocated_resources::GetCloudExadataInfrastructureUnallocatedResourcesOutput).
    pub fn build(
        self,
    ) -> crate::operation::get_cloud_exadata_infrastructure_unallocated_resources::GetCloudExadataInfrastructureUnallocatedResourcesOutput {
        crate::operation::get_cloud_exadata_infrastructure_unallocated_resources::GetCloudExadataInfrastructureUnallocatedResourcesOutput {
            cloud_exadata_infrastructure_unallocated_resources: self.cloud_exadata_infrastructure_unallocated_resources,
            _request_id: self._request_id,
        }
    }
}
