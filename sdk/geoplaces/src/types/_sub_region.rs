// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The sub-region.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct SubRegion {
    /// <p>Abbreviated code for the county or sub-region.</p>
    pub code: ::std::option::Option<::std::string::String>,
    /// <p>Name for the county or sub-region.</p>
    pub name: ::std::option::Option<::std::string::String>,
}
impl SubRegion {
    /// <p>Abbreviated code for the county or sub-region.</p>
    pub fn code(&self) -> ::std::option::Option<&str> {
        self.code.as_deref()
    }
    /// <p>Name for the county or sub-region.</p>
    pub fn name(&self) -> ::std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl SubRegion {
    /// Creates a new builder-style object to manufacture [`SubRegion`](crate::types::SubRegion).
    pub fn builder() -> crate::types::builders::SubRegionBuilder {
        crate::types::builders::SubRegionBuilder::default()
    }
}

/// A builder for [`SubRegion`](crate::types::SubRegion).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct SubRegionBuilder {
    pub(crate) code: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
}
impl SubRegionBuilder {
    /// <p>Abbreviated code for the county or sub-region.</p>
    pub fn code(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.code = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Abbreviated code for the county or sub-region.</p>
    pub fn set_code(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.code = input;
        self
    }
    /// <p>Abbreviated code for the county or sub-region.</p>
    pub fn get_code(&self) -> &::std::option::Option<::std::string::String> {
        &self.code
    }
    /// <p>Name for the county or sub-region.</p>
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Name for the county or sub-region.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>Name for the county or sub-region.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// Consumes the builder and constructs a [`SubRegion`](crate::types::SubRegion).
    pub fn build(self) -> crate::types::SubRegion {
        crate::types::SubRegion {
            code: self.code,
            name: self.name,
        }
    }
}
