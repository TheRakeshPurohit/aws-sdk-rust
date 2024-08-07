// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p><b>This data type is used with the Amazon GameLift containers feature, which is currently in public preview.</b></p>
/// <p>The properties of container groups that are running on a container fleet. Container group properties for a fleet can't be changed.</p>
/// <p><b>Returned by:</b> <code>DescribeFleetAttributes</code>, <code>CreateFleet</code></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ContainerGroupsAttributes {
    /// <p>A collection of properties that describe each container group in the fleet. A container fleet is deployed with one or more <code>ContainerGroupDefinition</code> resources, which is where these properties are set.</p>
    pub container_group_definition_properties: ::std::option::Option<::std::vec::Vec<crate::types::ContainerGroupDefinitionProperty>>,
    /// <p>A set of ports that allow inbound traffic to connect to processes running in the fleet's container groups. Amazon GameLift maps each connection port to a container port, which is assigned to a specific container process. A fleet's connection port range can't be changed, but you can control access to connection ports by updating a fleet's <code>EC2InboundPermissions</code> with <code>UpdateFleetPortSettings</code>.</p>
    pub connection_port_range: ::std::option::Option<crate::types::ConnectionPortRange>,
    /// <p>Details about the number of replica container groups that Amazon GameLift deploys to each instance in the container fleet.</p>
    pub container_groups_per_instance: ::std::option::Option<crate::types::ContainerGroupsPerInstance>,
}
impl ContainerGroupsAttributes {
    /// <p>A collection of properties that describe each container group in the fleet. A container fleet is deployed with one or more <code>ContainerGroupDefinition</code> resources, which is where these properties are set.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.container_group_definition_properties.is_none()`.
    pub fn container_group_definition_properties(&self) -> &[crate::types::ContainerGroupDefinitionProperty] {
        self.container_group_definition_properties.as_deref().unwrap_or_default()
    }
    /// <p>A set of ports that allow inbound traffic to connect to processes running in the fleet's container groups. Amazon GameLift maps each connection port to a container port, which is assigned to a specific container process. A fleet's connection port range can't be changed, but you can control access to connection ports by updating a fleet's <code>EC2InboundPermissions</code> with <code>UpdateFleetPortSettings</code>.</p>
    pub fn connection_port_range(&self) -> ::std::option::Option<&crate::types::ConnectionPortRange> {
        self.connection_port_range.as_ref()
    }
    /// <p>Details about the number of replica container groups that Amazon GameLift deploys to each instance in the container fleet.</p>
    pub fn container_groups_per_instance(&self) -> ::std::option::Option<&crate::types::ContainerGroupsPerInstance> {
        self.container_groups_per_instance.as_ref()
    }
}
impl ContainerGroupsAttributes {
    /// Creates a new builder-style object to manufacture [`ContainerGroupsAttributes`](crate::types::ContainerGroupsAttributes).
    pub fn builder() -> crate::types::builders::ContainerGroupsAttributesBuilder {
        crate::types::builders::ContainerGroupsAttributesBuilder::default()
    }
}

/// A builder for [`ContainerGroupsAttributes`](crate::types::ContainerGroupsAttributes).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ContainerGroupsAttributesBuilder {
    pub(crate) container_group_definition_properties: ::std::option::Option<::std::vec::Vec<crate::types::ContainerGroupDefinitionProperty>>,
    pub(crate) connection_port_range: ::std::option::Option<crate::types::ConnectionPortRange>,
    pub(crate) container_groups_per_instance: ::std::option::Option<crate::types::ContainerGroupsPerInstance>,
}
impl ContainerGroupsAttributesBuilder {
    /// Appends an item to `container_group_definition_properties`.
    ///
    /// To override the contents of this collection use [`set_container_group_definition_properties`](Self::set_container_group_definition_properties).
    ///
    /// <p>A collection of properties that describe each container group in the fleet. A container fleet is deployed with one or more <code>ContainerGroupDefinition</code> resources, which is where these properties are set.</p>
    pub fn container_group_definition_properties(mut self, input: crate::types::ContainerGroupDefinitionProperty) -> Self {
        let mut v = self.container_group_definition_properties.unwrap_or_default();
        v.push(input);
        self.container_group_definition_properties = ::std::option::Option::Some(v);
        self
    }
    /// <p>A collection of properties that describe each container group in the fleet. A container fleet is deployed with one or more <code>ContainerGroupDefinition</code> resources, which is where these properties are set.</p>
    pub fn set_container_group_definition_properties(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ContainerGroupDefinitionProperty>>,
    ) -> Self {
        self.container_group_definition_properties = input;
        self
    }
    /// <p>A collection of properties that describe each container group in the fleet. A container fleet is deployed with one or more <code>ContainerGroupDefinition</code> resources, which is where these properties are set.</p>
    pub fn get_container_group_definition_properties(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::ContainerGroupDefinitionProperty>> {
        &self.container_group_definition_properties
    }
    /// <p>A set of ports that allow inbound traffic to connect to processes running in the fleet's container groups. Amazon GameLift maps each connection port to a container port, which is assigned to a specific container process. A fleet's connection port range can't be changed, but you can control access to connection ports by updating a fleet's <code>EC2InboundPermissions</code> with <code>UpdateFleetPortSettings</code>.</p>
    pub fn connection_port_range(mut self, input: crate::types::ConnectionPortRange) -> Self {
        self.connection_port_range = ::std::option::Option::Some(input);
        self
    }
    /// <p>A set of ports that allow inbound traffic to connect to processes running in the fleet's container groups. Amazon GameLift maps each connection port to a container port, which is assigned to a specific container process. A fleet's connection port range can't be changed, but you can control access to connection ports by updating a fleet's <code>EC2InboundPermissions</code> with <code>UpdateFleetPortSettings</code>.</p>
    pub fn set_connection_port_range(mut self, input: ::std::option::Option<crate::types::ConnectionPortRange>) -> Self {
        self.connection_port_range = input;
        self
    }
    /// <p>A set of ports that allow inbound traffic to connect to processes running in the fleet's container groups. Amazon GameLift maps each connection port to a container port, which is assigned to a specific container process. A fleet's connection port range can't be changed, but you can control access to connection ports by updating a fleet's <code>EC2InboundPermissions</code> with <code>UpdateFleetPortSettings</code>.</p>
    pub fn get_connection_port_range(&self) -> &::std::option::Option<crate::types::ConnectionPortRange> {
        &self.connection_port_range
    }
    /// <p>Details about the number of replica container groups that Amazon GameLift deploys to each instance in the container fleet.</p>
    pub fn container_groups_per_instance(mut self, input: crate::types::ContainerGroupsPerInstance) -> Self {
        self.container_groups_per_instance = ::std::option::Option::Some(input);
        self
    }
    /// <p>Details about the number of replica container groups that Amazon GameLift deploys to each instance in the container fleet.</p>
    pub fn set_container_groups_per_instance(mut self, input: ::std::option::Option<crate::types::ContainerGroupsPerInstance>) -> Self {
        self.container_groups_per_instance = input;
        self
    }
    /// <p>Details about the number of replica container groups that Amazon GameLift deploys to each instance in the container fleet.</p>
    pub fn get_container_groups_per_instance(&self) -> &::std::option::Option<crate::types::ContainerGroupsPerInstance> {
        &self.container_groups_per_instance
    }
    /// Consumes the builder and constructs a [`ContainerGroupsAttributes`](crate::types::ContainerGroupsAttributes).
    pub fn build(self) -> crate::types::ContainerGroupsAttributes {
        crate::types::ContainerGroupsAttributes {
            container_group_definition_properties: self.container_group_definition_properties,
            connection_port_range: self.connection_port_range,
            container_groups_per_instance: self.container_groups_per_instance,
        }
    }
}
