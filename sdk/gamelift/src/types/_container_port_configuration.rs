// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Defines ranges of ports that server processes can connect to.</p>
/// <p><b>Part of:</b> <code>ContainerDefinition$PortConfiguration</code></p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ContainerPortConfiguration {
    /// <p>Specifies one or more ranges of ports on a container. These ranges must not overlap.</p>
    pub container_port_ranges: ::std::option::Option<::std::vec::Vec<crate::types::ContainerPortRange>>,
}
impl ContainerPortConfiguration {
    /// <p>Specifies one or more ranges of ports on a container. These ranges must not overlap.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.container_port_ranges.is_none()`.
    pub fn container_port_ranges(&self) -> &[crate::types::ContainerPortRange] {
        self.container_port_ranges.as_deref().unwrap_or_default()
    }
}
impl ContainerPortConfiguration {
    /// Creates a new builder-style object to manufacture [`ContainerPortConfiguration`](crate::types::ContainerPortConfiguration).
    pub fn builder() -> crate::types::builders::ContainerPortConfigurationBuilder {
        crate::types::builders::ContainerPortConfigurationBuilder::default()
    }
}

/// A builder for [`ContainerPortConfiguration`](crate::types::ContainerPortConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ContainerPortConfigurationBuilder {
    pub(crate) container_port_ranges: ::std::option::Option<::std::vec::Vec<crate::types::ContainerPortRange>>,
}
impl ContainerPortConfigurationBuilder {
    /// Appends an item to `container_port_ranges`.
    ///
    /// To override the contents of this collection use [`set_container_port_ranges`](Self::set_container_port_ranges).
    ///
    /// <p>Specifies one or more ranges of ports on a container. These ranges must not overlap.</p>
    pub fn container_port_ranges(mut self, input: crate::types::ContainerPortRange) -> Self {
        let mut v = self.container_port_ranges.unwrap_or_default();
        v.push(input);
        self.container_port_ranges = ::std::option::Option::Some(v);
        self
    }
    /// <p>Specifies one or more ranges of ports on a container. These ranges must not overlap.</p>
    pub fn set_container_port_ranges(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ContainerPortRange>>) -> Self {
        self.container_port_ranges = input;
        self
    }
    /// <p>Specifies one or more ranges of ports on a container. These ranges must not overlap.</p>
    pub fn get_container_port_ranges(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ContainerPortRange>> {
        &self.container_port_ranges
    }
    /// Consumes the builder and constructs a [`ContainerPortConfiguration`](crate::types::ContainerPortConfiguration).
    pub fn build(self) -> crate::types::ContainerPortConfiguration {
        crate::types::ContainerPortConfiguration {
            container_port_ranges: self.container_port_ranges,
        }
    }
}
