// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct UpdateDbInstanceInput {
    /// <p>The id of the DB instance.</p>
    pub identifier: ::std::option::Option<::std::string::String>,
    /// <p>Configuration for sending InfluxDB engine logs to send to specified S3 bucket.</p>
    pub log_delivery_configuration: ::std::option::Option<crate::types::LogDeliveryConfiguration>,
    /// <p>The id of the DB parameter group to assign to your DB instance. DB parameter groups specify how the database is configured. For example, DB parameter groups can specify the limit for query concurrency.</p>
    pub db_parameter_group_identifier: ::std::option::Option<::std::string::String>,
}
impl UpdateDbInstanceInput {
    /// <p>The id of the DB instance.</p>
    pub fn identifier(&self) -> ::std::option::Option<&str> {
        self.identifier.as_deref()
    }
    /// <p>Configuration for sending InfluxDB engine logs to send to specified S3 bucket.</p>
    pub fn log_delivery_configuration(&self) -> ::std::option::Option<&crate::types::LogDeliveryConfiguration> {
        self.log_delivery_configuration.as_ref()
    }
    /// <p>The id of the DB parameter group to assign to your DB instance. DB parameter groups specify how the database is configured. For example, DB parameter groups can specify the limit for query concurrency.</p>
    pub fn db_parameter_group_identifier(&self) -> ::std::option::Option<&str> {
        self.db_parameter_group_identifier.as_deref()
    }
}
impl UpdateDbInstanceInput {
    /// Creates a new builder-style object to manufacture [`UpdateDbInstanceInput`](crate::operation::update_db_instance::UpdateDbInstanceInput).
    pub fn builder() -> crate::operation::update_db_instance::builders::UpdateDbInstanceInputBuilder {
        crate::operation::update_db_instance::builders::UpdateDbInstanceInputBuilder::default()
    }
}

/// A builder for [`UpdateDbInstanceInput`](crate::operation::update_db_instance::UpdateDbInstanceInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct UpdateDbInstanceInputBuilder {
    pub(crate) identifier: ::std::option::Option<::std::string::String>,
    pub(crate) log_delivery_configuration: ::std::option::Option<crate::types::LogDeliveryConfiguration>,
    pub(crate) db_parameter_group_identifier: ::std::option::Option<::std::string::String>,
}
impl UpdateDbInstanceInputBuilder {
    /// <p>The id of the DB instance.</p>
    /// This field is required.
    pub fn identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The id of the DB instance.</p>
    pub fn set_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.identifier = input;
        self
    }
    /// <p>The id of the DB instance.</p>
    pub fn get_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.identifier
    }
    /// <p>Configuration for sending InfluxDB engine logs to send to specified S3 bucket.</p>
    pub fn log_delivery_configuration(mut self, input: crate::types::LogDeliveryConfiguration) -> Self {
        self.log_delivery_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>Configuration for sending InfluxDB engine logs to send to specified S3 bucket.</p>
    pub fn set_log_delivery_configuration(mut self, input: ::std::option::Option<crate::types::LogDeliveryConfiguration>) -> Self {
        self.log_delivery_configuration = input;
        self
    }
    /// <p>Configuration for sending InfluxDB engine logs to send to specified S3 bucket.</p>
    pub fn get_log_delivery_configuration(&self) -> &::std::option::Option<crate::types::LogDeliveryConfiguration> {
        &self.log_delivery_configuration
    }
    /// <p>The id of the DB parameter group to assign to your DB instance. DB parameter groups specify how the database is configured. For example, DB parameter groups can specify the limit for query concurrency.</p>
    pub fn db_parameter_group_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.db_parameter_group_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The id of the DB parameter group to assign to your DB instance. DB parameter groups specify how the database is configured. For example, DB parameter groups can specify the limit for query concurrency.</p>
    pub fn set_db_parameter_group_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.db_parameter_group_identifier = input;
        self
    }
    /// <p>The id of the DB parameter group to assign to your DB instance. DB parameter groups specify how the database is configured. For example, DB parameter groups can specify the limit for query concurrency.</p>
    pub fn get_db_parameter_group_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.db_parameter_group_identifier
    }
    /// Consumes the builder and constructs a [`UpdateDbInstanceInput`](crate::operation::update_db_instance::UpdateDbInstanceInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::update_db_instance::UpdateDbInstanceInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::update_db_instance::UpdateDbInstanceInput {
            identifier: self.identifier,
            log_delivery_configuration: self.log_delivery_configuration,
            db_parameter_group_identifier: self.db_parameter_group_identifier,
        })
    }
}
