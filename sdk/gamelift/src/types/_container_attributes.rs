// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p><b>This data type is used with the Amazon GameLift containers feature, which is currently in public preview.</b></p>
/// <p>Describes attributes of containers that are deployed to a fleet with compute type <code>CONTAINER</code>.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ContainerAttributes {
    /// <p>Describes how container ports map to connection ports on the fleet instance. Incoming traffic connects to a game via a connection port. A <code>ContainerPortMapping</code> directs the traffic from a connection port to a port on the container that hosts the game session.</p>
    pub container_port_mappings: ::std::option::Option<::std::vec::Vec<crate::types::ContainerPortMapping>>,
}
impl ContainerAttributes {
    /// <p>Describes how container ports map to connection ports on the fleet instance. Incoming traffic connects to a game via a connection port. A <code>ContainerPortMapping</code> directs the traffic from a connection port to a port on the container that hosts the game session.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.container_port_mappings.is_none()`.
    pub fn container_port_mappings(&self) -> &[crate::types::ContainerPortMapping] {
        self.container_port_mappings.as_deref().unwrap_or_default()
    }
}
impl ContainerAttributes {
    /// Creates a new builder-style object to manufacture [`ContainerAttributes`](crate::types::ContainerAttributes).
    pub fn builder() -> crate::types::builders::ContainerAttributesBuilder {
        crate::types::builders::ContainerAttributesBuilder::default()
    }
}

/// A builder for [`ContainerAttributes`](crate::types::ContainerAttributes).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ContainerAttributesBuilder {
    pub(crate) container_port_mappings: ::std::option::Option<::std::vec::Vec<crate::types::ContainerPortMapping>>,
}
impl ContainerAttributesBuilder {
    /// Appends an item to `container_port_mappings`.
    ///
    /// To override the contents of this collection use [`set_container_port_mappings`](Self::set_container_port_mappings).
    ///
    /// <p>Describes how container ports map to connection ports on the fleet instance. Incoming traffic connects to a game via a connection port. A <code>ContainerPortMapping</code> directs the traffic from a connection port to a port on the container that hosts the game session.</p>
    pub fn container_port_mappings(mut self, input: crate::types::ContainerPortMapping) -> Self {
        let mut v = self.container_port_mappings.unwrap_or_default();
        v.push(input);
        self.container_port_mappings = ::std::option::Option::Some(v);
        self
    }
    /// <p>Describes how container ports map to connection ports on the fleet instance. Incoming traffic connects to a game via a connection port. A <code>ContainerPortMapping</code> directs the traffic from a connection port to a port on the container that hosts the game session.</p>
    pub fn set_container_port_mappings(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ContainerPortMapping>>) -> Self {
        self.container_port_mappings = input;
        self
    }
    /// <p>Describes how container ports map to connection ports on the fleet instance. Incoming traffic connects to a game via a connection port. A <code>ContainerPortMapping</code> directs the traffic from a connection port to a port on the container that hosts the game session.</p>
    pub fn get_container_port_mappings(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ContainerPortMapping>> {
        &self.container_port_mappings
    }
    /// Consumes the builder and constructs a [`ContainerAttributes`](crate::types::ContainerAttributes).
    pub fn build(self) -> crate::types::ContainerAttributes {
        crate::types::ContainerAttributes {
            container_port_mappings: self.container_port_mappings,
        }
    }
}
