// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration details for a service managed Amazon EC2 fleet.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ServiceManagedEc2FleetConfiguration {
    /// <p>The Amazon EC2 instance capabilities.</p>
    pub instance_capabilities: ::std::option::Option<crate::types::ServiceManagedEc2InstanceCapabilities>,
    /// <p>The Amazon EC2 market type.</p>
    pub instance_market_options: ::std::option::Option<crate::types::ServiceManagedEc2InstanceMarketOptions>,
}
impl ServiceManagedEc2FleetConfiguration {
    /// <p>The Amazon EC2 instance capabilities.</p>
    pub fn instance_capabilities(&self) -> ::std::option::Option<&crate::types::ServiceManagedEc2InstanceCapabilities> {
        self.instance_capabilities.as_ref()
    }
    /// <p>The Amazon EC2 market type.</p>
    pub fn instance_market_options(&self) -> ::std::option::Option<&crate::types::ServiceManagedEc2InstanceMarketOptions> {
        self.instance_market_options.as_ref()
    }
}
impl ServiceManagedEc2FleetConfiguration {
    /// Creates a new builder-style object to manufacture [`ServiceManagedEc2FleetConfiguration`](crate::types::ServiceManagedEc2FleetConfiguration).
    pub fn builder() -> crate::types::builders::ServiceManagedEc2FleetConfigurationBuilder {
        crate::types::builders::ServiceManagedEc2FleetConfigurationBuilder::default()
    }
}

/// A builder for [`ServiceManagedEc2FleetConfiguration`](crate::types::ServiceManagedEc2FleetConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ServiceManagedEc2FleetConfigurationBuilder {
    pub(crate) instance_capabilities: ::std::option::Option<crate::types::ServiceManagedEc2InstanceCapabilities>,
    pub(crate) instance_market_options: ::std::option::Option<crate::types::ServiceManagedEc2InstanceMarketOptions>,
}
impl ServiceManagedEc2FleetConfigurationBuilder {
    /// <p>The Amazon EC2 instance capabilities.</p>
    /// This field is required.
    pub fn instance_capabilities(mut self, input: crate::types::ServiceManagedEc2InstanceCapabilities) -> Self {
        self.instance_capabilities = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Amazon EC2 instance capabilities.</p>
    pub fn set_instance_capabilities(mut self, input: ::std::option::Option<crate::types::ServiceManagedEc2InstanceCapabilities>) -> Self {
        self.instance_capabilities = input;
        self
    }
    /// <p>The Amazon EC2 instance capabilities.</p>
    pub fn get_instance_capabilities(&self) -> &::std::option::Option<crate::types::ServiceManagedEc2InstanceCapabilities> {
        &self.instance_capabilities
    }
    /// <p>The Amazon EC2 market type.</p>
    /// This field is required.
    pub fn instance_market_options(mut self, input: crate::types::ServiceManagedEc2InstanceMarketOptions) -> Self {
        self.instance_market_options = ::std::option::Option::Some(input);
        self
    }
    /// <p>The Amazon EC2 market type.</p>
    pub fn set_instance_market_options(mut self, input: ::std::option::Option<crate::types::ServiceManagedEc2InstanceMarketOptions>) -> Self {
        self.instance_market_options = input;
        self
    }
    /// <p>The Amazon EC2 market type.</p>
    pub fn get_instance_market_options(&self) -> &::std::option::Option<crate::types::ServiceManagedEc2InstanceMarketOptions> {
        &self.instance_market_options
    }
    /// Consumes the builder and constructs a [`ServiceManagedEc2FleetConfiguration`](crate::types::ServiceManagedEc2FleetConfiguration).
    pub fn build(self) -> crate::types::ServiceManagedEc2FleetConfiguration {
        crate::types::ServiceManagedEc2FleetConfiguration {
            instance_capabilities: self.instance_capabilities,
            instance_market_options: self.instance_market_options,
        }
    }
}
