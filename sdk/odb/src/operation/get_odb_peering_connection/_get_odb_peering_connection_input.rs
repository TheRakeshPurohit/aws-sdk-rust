// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct GetOdbPeeringConnectionInput {
    /// <p>The unique identifier of the ODB peering connection to retrieve information about.</p>
    pub odb_peering_connection_id: ::std::option::Option<::std::string::String>,
}
impl GetOdbPeeringConnectionInput {
    /// <p>The unique identifier of the ODB peering connection to retrieve information about.</p>
    pub fn odb_peering_connection_id(&self) -> ::std::option::Option<&str> {
        self.odb_peering_connection_id.as_deref()
    }
}
impl GetOdbPeeringConnectionInput {
    /// Creates a new builder-style object to manufacture [`GetOdbPeeringConnectionInput`](crate::operation::get_odb_peering_connection::GetOdbPeeringConnectionInput).
    pub fn builder() -> crate::operation::get_odb_peering_connection::builders::GetOdbPeeringConnectionInputBuilder {
        crate::operation::get_odb_peering_connection::builders::GetOdbPeeringConnectionInputBuilder::default()
    }
}

/// A builder for [`GetOdbPeeringConnectionInput`](crate::operation::get_odb_peering_connection::GetOdbPeeringConnectionInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct GetOdbPeeringConnectionInputBuilder {
    pub(crate) odb_peering_connection_id: ::std::option::Option<::std::string::String>,
}
impl GetOdbPeeringConnectionInputBuilder {
    /// <p>The unique identifier of the ODB peering connection to retrieve information about.</p>
    /// This field is required.
    pub fn odb_peering_connection_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.odb_peering_connection_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The unique identifier of the ODB peering connection to retrieve information about.</p>
    pub fn set_odb_peering_connection_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.odb_peering_connection_id = input;
        self
    }
    /// <p>The unique identifier of the ODB peering connection to retrieve information about.</p>
    pub fn get_odb_peering_connection_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.odb_peering_connection_id
    }
    /// Consumes the builder and constructs a [`GetOdbPeeringConnectionInput`](crate::operation::get_odb_peering_connection::GetOdbPeeringConnectionInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<
        crate::operation::get_odb_peering_connection::GetOdbPeeringConnectionInput,
        ::aws_smithy_types::error::operation::BuildError,
    > {
        ::std::result::Result::Ok(crate::operation::get_odb_peering_connection::GetOdbPeeringConnectionInput {
            odb_peering_connection_id: self.odb_peering_connection_id,
        })
    }
}
