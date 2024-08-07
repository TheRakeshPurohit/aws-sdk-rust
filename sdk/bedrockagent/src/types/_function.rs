// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Defines parameters that the agent needs to invoke from the user to complete the function. Corresponds to an action in an action group.</p>
/// <p>This data type is used in the following API operations:</p>
/// <ul>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent_CreateAgentActionGroup.html#API_agent_CreateAgentActionGroup_RequestSyntax">CreateAgentActionGroup request</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent_CreateAgentActionGroup.html#API_agent_CreateAgentActionGroup_ResponseSyntax">CreateAgentActionGroup response</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent_UpdateAgentActionGroup.html#API_agent_UpdateAgentActionGroup_RequestSyntax">UpdateAgentActionGroup request</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent_UpdateAgentActionGroup.html#API_agent_UpdateAgentActionGroup_ResponseSyntax">UpdateAgentActionGroup response</a></p></li>
/// <li>
/// <p><a href="https://docs.aws.amazon.com/bedrock/latest/APIReference/API_agent_GetAgentActionGroup.html#API_agent_GetAgentActionGroup_ResponseSyntax">GetAgentActionGroup response</a></p></li>
/// </ul>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Function {
    /// <p>A name for the function.</p>
    pub name: ::std::string::String,
    /// <p>A description of the function and its purpose.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The parameters that the agent elicits from the user to fulfill the function.</p>
    pub parameters: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::ParameterDetail>>,
}
impl Function {
    /// <p>A name for the function.</p>
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    /// <p>A description of the function and its purpose.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The parameters that the agent elicits from the user to fulfill the function.</p>
    pub fn parameters(&self) -> ::std::option::Option<&::std::collections::HashMap<::std::string::String, crate::types::ParameterDetail>> {
        self.parameters.as_ref()
    }
}
impl Function {
    /// Creates a new builder-style object to manufacture [`Function`](crate::types::Function).
    pub fn builder() -> crate::types::builders::FunctionBuilder {
        crate::types::builders::FunctionBuilder::default()
    }
}

/// A builder for [`Function`](crate::types::Function).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct FunctionBuilder {
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) parameters: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::ParameterDetail>>,
}
impl FunctionBuilder {
    /// <p>A name for the function.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A name for the function.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>A name for the function.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p>A description of the function and its purpose.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A description of the function and its purpose.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>A description of the function and its purpose.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// Adds a key-value pair to `parameters`.
    ///
    /// To override the contents of this collection use [`set_parameters`](Self::set_parameters).
    ///
    /// <p>The parameters that the agent elicits from the user to fulfill the function.</p>
    pub fn parameters(mut self, k: impl ::std::convert::Into<::std::string::String>, v: crate::types::ParameterDetail) -> Self {
        let mut hash_map = self.parameters.unwrap_or_default();
        hash_map.insert(k.into(), v);
        self.parameters = ::std::option::Option::Some(hash_map);
        self
    }
    /// <p>The parameters that the agent elicits from the user to fulfill the function.</p>
    pub fn set_parameters(
        mut self,
        input: ::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::ParameterDetail>>,
    ) -> Self {
        self.parameters = input;
        self
    }
    /// <p>The parameters that the agent elicits from the user to fulfill the function.</p>
    pub fn get_parameters(&self) -> &::std::option::Option<::std::collections::HashMap<::std::string::String, crate::types::ParameterDetail>> {
        &self.parameters
    }
    /// Consumes the builder and constructs a [`Function`](crate::types::Function).
    /// This method will fail if any of the following fields are not set:
    /// - [`name`](crate::types::builders::FunctionBuilder::name)
    pub fn build(self) -> ::std::result::Result<crate::types::Function, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Function {
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building Function",
                )
            })?,
            description: self.description,
            parameters: self.parameters,
        })
    }
}
