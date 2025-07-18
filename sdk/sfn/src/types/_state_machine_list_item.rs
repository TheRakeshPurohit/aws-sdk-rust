// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Contains details about the state machine.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StateMachineListItem {
    /// <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
    pub state_machine_arn: ::std::string::String,
    /// <p>The name of the state machine.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li>
    /// <p>white space</p></li>
    /// <li>
    /// <p>brackets <code>&lt; &gt; { } \[ \]</code></p></li>
    /// <li>
    /// <p>wildcard characters <code>? *</code></p></li>
    /// <li>
    /// <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code></p></li>
    /// <li>
    /// <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>, <code>U+FFFE-FFFF</code>)</p></li>
    /// <li>
    /// <p>surrogates (<code>U+D800-DFFF</code>)</p></li>
    /// <li>
    /// <p>invalid characters (<code> U+10FFFF</code>)</p></li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    pub name: ::std::string::String,
    /// <p></p>
    pub r#type: crate::types::StateMachineType,
    /// <p>The date the state machine is created.</p>
    pub creation_date: ::aws_smithy_types::DateTime,
}
impl StateMachineListItem {
    /// <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
    pub fn state_machine_arn(&self) -> &str {
        use std::ops::Deref;
        self.state_machine_arn.deref()
    }
    /// <p>The name of the state machine.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li>
    /// <p>white space</p></li>
    /// <li>
    /// <p>brackets <code>&lt; &gt; { } \[ \]</code></p></li>
    /// <li>
    /// <p>wildcard characters <code>? *</code></p></li>
    /// <li>
    /// <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code></p></li>
    /// <li>
    /// <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>, <code>U+FFFE-FFFF</code>)</p></li>
    /// <li>
    /// <p>surrogates (<code>U+D800-DFFF</code>)</p></li>
    /// <li>
    /// <p>invalid characters (<code> U+10FFFF</code>)</p></li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    pub fn name(&self) -> &str {
        use std::ops::Deref;
        self.name.deref()
    }
    /// <p></p>
    pub fn r#type(&self) -> &crate::types::StateMachineType {
        &self.r#type
    }
    /// <p>The date the state machine is created.</p>
    pub fn creation_date(&self) -> &::aws_smithy_types::DateTime {
        &self.creation_date
    }
}
impl StateMachineListItem {
    /// Creates a new builder-style object to manufacture [`StateMachineListItem`](crate::types::StateMachineListItem).
    pub fn builder() -> crate::types::builders::StateMachineListItemBuilder {
        crate::types::builders::StateMachineListItemBuilder::default()
    }
}

/// A builder for [`StateMachineListItem`](crate::types::StateMachineListItem).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StateMachineListItemBuilder {
    pub(crate) state_machine_arn: ::std::option::Option<::std::string::String>,
    pub(crate) name: ::std::option::Option<::std::string::String>,
    pub(crate) r#type: ::std::option::Option<crate::types::StateMachineType>,
    pub(crate) creation_date: ::std::option::Option<::aws_smithy_types::DateTime>,
}
impl StateMachineListItemBuilder {
    /// <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
    /// This field is required.
    pub fn state_machine_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.state_machine_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
    pub fn set_state_machine_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.state_machine_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) that identifies the state machine.</p>
    pub fn get_state_machine_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.state_machine_arn
    }
    /// <p>The name of the state machine.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li>
    /// <p>white space</p></li>
    /// <li>
    /// <p>brackets <code>&lt; &gt; { } \[ \]</code></p></li>
    /// <li>
    /// <p>wildcard characters <code>? *</code></p></li>
    /// <li>
    /// <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code></p></li>
    /// <li>
    /// <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>, <code>U+FFFE-FFFF</code>)</p></li>
    /// <li>
    /// <p>surrogates (<code>U+D800-DFFF</code>)</p></li>
    /// <li>
    /// <p>invalid characters (<code> U+10FFFF</code>)</p></li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    /// This field is required.
    pub fn name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the state machine.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li>
    /// <p>white space</p></li>
    /// <li>
    /// <p>brackets <code>&lt; &gt; { } \[ \]</code></p></li>
    /// <li>
    /// <p>wildcard characters <code>? *</code></p></li>
    /// <li>
    /// <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code></p></li>
    /// <li>
    /// <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>, <code>U+FFFE-FFFF</code>)</p></li>
    /// <li>
    /// <p>surrogates (<code>U+D800-DFFF</code>)</p></li>
    /// <li>
    /// <p>invalid characters (<code> U+10FFFF</code>)</p></li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    pub fn set_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.name = input;
        self
    }
    /// <p>The name of the state machine.</p>
    /// <p>A name must <i>not</i> contain:</p>
    /// <ul>
    /// <li>
    /// <p>white space</p></li>
    /// <li>
    /// <p>brackets <code>&lt; &gt; { } \[ \]</code></p></li>
    /// <li>
    /// <p>wildcard characters <code>? *</code></p></li>
    /// <li>
    /// <p>special characters <code>" # % \ ^ | ~ ` $ &amp; , ; : /</code></p></li>
    /// <li>
    /// <p>control characters (<code>U+0000-001F</code>, <code>U+007F-009F</code>, <code>U+FFFE-FFFF</code>)</p></li>
    /// <li>
    /// <p>surrogates (<code>U+D800-DFFF</code>)</p></li>
    /// <li>
    /// <p>invalid characters (<code> U+10FFFF</code>)</p></li>
    /// </ul>
    /// <p>To enable logging with CloudWatch Logs, the name should only contain 0-9, A-Z, a-z, - and _.</p>
    pub fn get_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.name
    }
    /// <p></p>
    /// This field is required.
    pub fn r#type(mut self, input: crate::types::StateMachineType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p></p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::StateMachineType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p></p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::StateMachineType> {
        &self.r#type
    }
    /// <p>The date the state machine is created.</p>
    /// This field is required.
    pub fn creation_date(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_date = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date the state machine is created.</p>
    pub fn set_creation_date(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_date = input;
        self
    }
    /// <p>The date the state machine is created.</p>
    pub fn get_creation_date(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_date
    }
    /// Consumes the builder and constructs a [`StateMachineListItem`](crate::types::StateMachineListItem).
    /// This method will fail if any of the following fields are not set:
    /// - [`state_machine_arn`](crate::types::builders::StateMachineListItemBuilder::state_machine_arn)
    /// - [`name`](crate::types::builders::StateMachineListItemBuilder::name)
    /// - [`r#type`](crate::types::builders::StateMachineListItemBuilder::type)
    /// - [`creation_date`](crate::types::builders::StateMachineListItemBuilder::creation_date)
    pub fn build(self) -> ::std::result::Result<crate::types::StateMachineListItem, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::StateMachineListItem {
            state_machine_arn: self.state_machine_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "state_machine_arn",
                    "state_machine_arn was not specified but it is required when building StateMachineListItem",
                )
            })?,
            name: self.name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "name",
                    "name was not specified but it is required when building StateMachineListItem",
                )
            })?,
            r#type: self.r#type.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "r#type",
                    "r#type was not specified but it is required when building StateMachineListItem",
                )
            })?,
            creation_date: self.creation_date.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "creation_date",
                    "creation_date was not specified but it is required when building StateMachineListItem",
                )
            })?,
        })
    }
}
