// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct RegisterTypeInput {
    /// <p>The kind of extension.</p>
    pub r#type: ::std::option::Option<crate::types::RegistryType>,
    /// <p>The name of the extension being registered.</p>
    /// <p>We suggest that extension names adhere to the following patterns:</p>
    /// <ul>
    /// <li>
    /// <p>For resource types, <code>company_or_organization::service::type</code>.</p></li>
    /// <li>
    /// <p>For modules, <code>company_or_organization::service::type::MODULE</code>.</p></li>
    /// <li>
    /// <p>For Hooks, <code>MyCompany::Testing::MyTestHook</code>.</p></li>
    /// </ul><note>
    /// <p>The following organization namespaces are reserved and can't be used in your extension names:</p>
    /// <ul>
    /// <li>
    /// <p><code>Alexa</code></p></li>
    /// <li>
    /// <p><code>AMZN</code></p></li>
    /// <li>
    /// <p><code>Amazon</code></p></li>
    /// <li>
    /// <p><code>AWS</code></p></li>
    /// <li>
    /// <p><code>Custom</code></p></li>
    /// <li>
    /// <p><code>Dev</code></p></li>
    /// </ul>
    /// </note>
    pub type_name: ::std::option::Option<::std::string::String>,
    /// <p>A URL to the S3 bucket that contains the extension project package that contains the necessary files for the extension you want to register.</p>
    /// <p>For information about generating a schema handler package for the extension you want to register, see <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-cli-submit.html">submit</a> in the <i>CloudFormation Command Line Interface (CLI) User Guide</i>.</p><note>
    /// <p>The user registering the extension must be able to access the package in the S3 bucket. That's, the user needs to have <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a> permissions for the schema handler package. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html">Actions, Resources, and Condition Keys for Amazon S3</a> in the <i>Identity and Access Management User Guide</i>.</p>
    /// </note>
    pub schema_handler_package: ::std::option::Option<::std::string::String>,
    /// <p>Specifies logging configuration information for an extension.</p>
    pub logging_config: ::std::option::Option<crate::types::LoggingConfig>,
    /// <p>The Amazon Resource Name (ARN) of the IAM role for CloudFormation to assume when invoking the extension.</p>
    /// <p>For CloudFormation to assume the specified execution role, the role must contain a trust relationship with the CloudFormation service principal (<code>resources.cloudformation.amazonaws.com</code>). For more information about adding trust relationships, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/roles-managingrole-editing-console.html#roles-managingrole_edit-trust-policy">Modifying a role trust policy</a> in the <i>Identity and Access Management User Guide</i>.</p>
    /// <p>If your extension calls Amazon Web Services APIs in any of its handlers, you must create an <i> <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html">IAM execution role</a> </i> that includes the necessary permissions to call those Amazon Web Services APIs, and provision that execution role in your account. When CloudFormation needs to invoke the resource type handler, CloudFormation assumes this execution role to create a temporary session token, which it then passes to the resource type handler, thereby supplying your resource type with the appropriate credentials.</p>
    pub execution_role_arn: ::std::option::Option<::std::string::String>,
    /// <p>A unique identifier that acts as an idempotency key for this registration request. Specifying a client request token prevents CloudFormation from generating more than one version of an extension from the same registration request, even if the request is submitted multiple times.</p>
    pub client_request_token: ::std::option::Option<::std::string::String>,
}
impl RegisterTypeInput {
    /// <p>The kind of extension.</p>
    pub fn r#type(&self) -> ::std::option::Option<&crate::types::RegistryType> {
        self.r#type.as_ref()
    }
    /// <p>The name of the extension being registered.</p>
    /// <p>We suggest that extension names adhere to the following patterns:</p>
    /// <ul>
    /// <li>
    /// <p>For resource types, <code>company_or_organization::service::type</code>.</p></li>
    /// <li>
    /// <p>For modules, <code>company_or_organization::service::type::MODULE</code>.</p></li>
    /// <li>
    /// <p>For Hooks, <code>MyCompany::Testing::MyTestHook</code>.</p></li>
    /// </ul><note>
    /// <p>The following organization namespaces are reserved and can't be used in your extension names:</p>
    /// <ul>
    /// <li>
    /// <p><code>Alexa</code></p></li>
    /// <li>
    /// <p><code>AMZN</code></p></li>
    /// <li>
    /// <p><code>Amazon</code></p></li>
    /// <li>
    /// <p><code>AWS</code></p></li>
    /// <li>
    /// <p><code>Custom</code></p></li>
    /// <li>
    /// <p><code>Dev</code></p></li>
    /// </ul>
    /// </note>
    pub fn type_name(&self) -> ::std::option::Option<&str> {
        self.type_name.as_deref()
    }
    /// <p>A URL to the S3 bucket that contains the extension project package that contains the necessary files for the extension you want to register.</p>
    /// <p>For information about generating a schema handler package for the extension you want to register, see <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-cli-submit.html">submit</a> in the <i>CloudFormation Command Line Interface (CLI) User Guide</i>.</p><note>
    /// <p>The user registering the extension must be able to access the package in the S3 bucket. That's, the user needs to have <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a> permissions for the schema handler package. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html">Actions, Resources, and Condition Keys for Amazon S3</a> in the <i>Identity and Access Management User Guide</i>.</p>
    /// </note>
    pub fn schema_handler_package(&self) -> ::std::option::Option<&str> {
        self.schema_handler_package.as_deref()
    }
    /// <p>Specifies logging configuration information for an extension.</p>
    pub fn logging_config(&self) -> ::std::option::Option<&crate::types::LoggingConfig> {
        self.logging_config.as_ref()
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role for CloudFormation to assume when invoking the extension.</p>
    /// <p>For CloudFormation to assume the specified execution role, the role must contain a trust relationship with the CloudFormation service principal (<code>resources.cloudformation.amazonaws.com</code>). For more information about adding trust relationships, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/roles-managingrole-editing-console.html#roles-managingrole_edit-trust-policy">Modifying a role trust policy</a> in the <i>Identity and Access Management User Guide</i>.</p>
    /// <p>If your extension calls Amazon Web Services APIs in any of its handlers, you must create an <i> <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html">IAM execution role</a> </i> that includes the necessary permissions to call those Amazon Web Services APIs, and provision that execution role in your account. When CloudFormation needs to invoke the resource type handler, CloudFormation assumes this execution role to create a temporary session token, which it then passes to the resource type handler, thereby supplying your resource type with the appropriate credentials.</p>
    pub fn execution_role_arn(&self) -> ::std::option::Option<&str> {
        self.execution_role_arn.as_deref()
    }
    /// <p>A unique identifier that acts as an idempotency key for this registration request. Specifying a client request token prevents CloudFormation from generating more than one version of an extension from the same registration request, even if the request is submitted multiple times.</p>
    pub fn client_request_token(&self) -> ::std::option::Option<&str> {
        self.client_request_token.as_deref()
    }
}
impl RegisterTypeInput {
    /// Creates a new builder-style object to manufacture [`RegisterTypeInput`](crate::operation::register_type::RegisterTypeInput).
    pub fn builder() -> crate::operation::register_type::builders::RegisterTypeInputBuilder {
        crate::operation::register_type::builders::RegisterTypeInputBuilder::default()
    }
}

/// A builder for [`RegisterTypeInput`](crate::operation::register_type::RegisterTypeInput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct RegisterTypeInputBuilder {
    pub(crate) r#type: ::std::option::Option<crate::types::RegistryType>,
    pub(crate) type_name: ::std::option::Option<::std::string::String>,
    pub(crate) schema_handler_package: ::std::option::Option<::std::string::String>,
    pub(crate) logging_config: ::std::option::Option<crate::types::LoggingConfig>,
    pub(crate) execution_role_arn: ::std::option::Option<::std::string::String>,
    pub(crate) client_request_token: ::std::option::Option<::std::string::String>,
}
impl RegisterTypeInputBuilder {
    /// <p>The kind of extension.</p>
    pub fn r#type(mut self, input: crate::types::RegistryType) -> Self {
        self.r#type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The kind of extension.</p>
    pub fn set_type(mut self, input: ::std::option::Option<crate::types::RegistryType>) -> Self {
        self.r#type = input;
        self
    }
    /// <p>The kind of extension.</p>
    pub fn get_type(&self) -> &::std::option::Option<crate::types::RegistryType> {
        &self.r#type
    }
    /// <p>The name of the extension being registered.</p>
    /// <p>We suggest that extension names adhere to the following patterns:</p>
    /// <ul>
    /// <li>
    /// <p>For resource types, <code>company_or_organization::service::type</code>.</p></li>
    /// <li>
    /// <p>For modules, <code>company_or_organization::service::type::MODULE</code>.</p></li>
    /// <li>
    /// <p>For Hooks, <code>MyCompany::Testing::MyTestHook</code>.</p></li>
    /// </ul><note>
    /// <p>The following organization namespaces are reserved and can't be used in your extension names:</p>
    /// <ul>
    /// <li>
    /// <p><code>Alexa</code></p></li>
    /// <li>
    /// <p><code>AMZN</code></p></li>
    /// <li>
    /// <p><code>Amazon</code></p></li>
    /// <li>
    /// <p><code>AWS</code></p></li>
    /// <li>
    /// <p><code>Custom</code></p></li>
    /// <li>
    /// <p><code>Dev</code></p></li>
    /// </ul>
    /// </note>
    /// This field is required.
    pub fn type_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.type_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the extension being registered.</p>
    /// <p>We suggest that extension names adhere to the following patterns:</p>
    /// <ul>
    /// <li>
    /// <p>For resource types, <code>company_or_organization::service::type</code>.</p></li>
    /// <li>
    /// <p>For modules, <code>company_or_organization::service::type::MODULE</code>.</p></li>
    /// <li>
    /// <p>For Hooks, <code>MyCompany::Testing::MyTestHook</code>.</p></li>
    /// </ul><note>
    /// <p>The following organization namespaces are reserved and can't be used in your extension names:</p>
    /// <ul>
    /// <li>
    /// <p><code>Alexa</code></p></li>
    /// <li>
    /// <p><code>AMZN</code></p></li>
    /// <li>
    /// <p><code>Amazon</code></p></li>
    /// <li>
    /// <p><code>AWS</code></p></li>
    /// <li>
    /// <p><code>Custom</code></p></li>
    /// <li>
    /// <p><code>Dev</code></p></li>
    /// </ul>
    /// </note>
    pub fn set_type_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.type_name = input;
        self
    }
    /// <p>The name of the extension being registered.</p>
    /// <p>We suggest that extension names adhere to the following patterns:</p>
    /// <ul>
    /// <li>
    /// <p>For resource types, <code>company_or_organization::service::type</code>.</p></li>
    /// <li>
    /// <p>For modules, <code>company_or_organization::service::type::MODULE</code>.</p></li>
    /// <li>
    /// <p>For Hooks, <code>MyCompany::Testing::MyTestHook</code>.</p></li>
    /// </ul><note>
    /// <p>The following organization namespaces are reserved and can't be used in your extension names:</p>
    /// <ul>
    /// <li>
    /// <p><code>Alexa</code></p></li>
    /// <li>
    /// <p><code>AMZN</code></p></li>
    /// <li>
    /// <p><code>Amazon</code></p></li>
    /// <li>
    /// <p><code>AWS</code></p></li>
    /// <li>
    /// <p><code>Custom</code></p></li>
    /// <li>
    /// <p><code>Dev</code></p></li>
    /// </ul>
    /// </note>
    pub fn get_type_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.type_name
    }
    /// <p>A URL to the S3 bucket that contains the extension project package that contains the necessary files for the extension you want to register.</p>
    /// <p>For information about generating a schema handler package for the extension you want to register, see <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-cli-submit.html">submit</a> in the <i>CloudFormation Command Line Interface (CLI) User Guide</i>.</p><note>
    /// <p>The user registering the extension must be able to access the package in the S3 bucket. That's, the user needs to have <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a> permissions for the schema handler package. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html">Actions, Resources, and Condition Keys for Amazon S3</a> in the <i>Identity and Access Management User Guide</i>.</p>
    /// </note>
    /// This field is required.
    pub fn schema_handler_package(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.schema_handler_package = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A URL to the S3 bucket that contains the extension project package that contains the necessary files for the extension you want to register.</p>
    /// <p>For information about generating a schema handler package for the extension you want to register, see <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-cli-submit.html">submit</a> in the <i>CloudFormation Command Line Interface (CLI) User Guide</i>.</p><note>
    /// <p>The user registering the extension must be able to access the package in the S3 bucket. That's, the user needs to have <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a> permissions for the schema handler package. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html">Actions, Resources, and Condition Keys for Amazon S3</a> in the <i>Identity and Access Management User Guide</i>.</p>
    /// </note>
    pub fn set_schema_handler_package(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.schema_handler_package = input;
        self
    }
    /// <p>A URL to the S3 bucket that contains the extension project package that contains the necessary files for the extension you want to register.</p>
    /// <p>For information about generating a schema handler package for the extension you want to register, see <a href="https://docs.aws.amazon.com/cloudformation-cli/latest/userguide/resource-type-cli-submit.html">submit</a> in the <i>CloudFormation Command Line Interface (CLI) User Guide</i>.</p><note>
    /// <p>The user registering the extension must be able to access the package in the S3 bucket. That's, the user needs to have <a href="https://docs.aws.amazon.com/AmazonS3/latest/API/API_GetObject.html">GetObject</a> permissions for the schema handler package. For more information, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/list_amazons3.html">Actions, Resources, and Condition Keys for Amazon S3</a> in the <i>Identity and Access Management User Guide</i>.</p>
    /// </note>
    pub fn get_schema_handler_package(&self) -> &::std::option::Option<::std::string::String> {
        &self.schema_handler_package
    }
    /// <p>Specifies logging configuration information for an extension.</p>
    pub fn logging_config(mut self, input: crate::types::LoggingConfig) -> Self {
        self.logging_config = ::std::option::Option::Some(input);
        self
    }
    /// <p>Specifies logging configuration information for an extension.</p>
    pub fn set_logging_config(mut self, input: ::std::option::Option<crate::types::LoggingConfig>) -> Self {
        self.logging_config = input;
        self
    }
    /// <p>Specifies logging configuration information for an extension.</p>
    pub fn get_logging_config(&self) -> &::std::option::Option<crate::types::LoggingConfig> {
        &self.logging_config
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role for CloudFormation to assume when invoking the extension.</p>
    /// <p>For CloudFormation to assume the specified execution role, the role must contain a trust relationship with the CloudFormation service principal (<code>resources.cloudformation.amazonaws.com</code>). For more information about adding trust relationships, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/roles-managingrole-editing-console.html#roles-managingrole_edit-trust-policy">Modifying a role trust policy</a> in the <i>Identity and Access Management User Guide</i>.</p>
    /// <p>If your extension calls Amazon Web Services APIs in any of its handlers, you must create an <i> <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html">IAM execution role</a> </i> that includes the necessary permissions to call those Amazon Web Services APIs, and provision that execution role in your account. When CloudFormation needs to invoke the resource type handler, CloudFormation assumes this execution role to create a temporary session token, which it then passes to the resource type handler, thereby supplying your resource type with the appropriate credentials.</p>
    pub fn execution_role_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.execution_role_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role for CloudFormation to assume when invoking the extension.</p>
    /// <p>For CloudFormation to assume the specified execution role, the role must contain a trust relationship with the CloudFormation service principal (<code>resources.cloudformation.amazonaws.com</code>). For more information about adding trust relationships, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/roles-managingrole-editing-console.html#roles-managingrole_edit-trust-policy">Modifying a role trust policy</a> in the <i>Identity and Access Management User Guide</i>.</p>
    /// <p>If your extension calls Amazon Web Services APIs in any of its handlers, you must create an <i> <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html">IAM execution role</a> </i> that includes the necessary permissions to call those Amazon Web Services APIs, and provision that execution role in your account. When CloudFormation needs to invoke the resource type handler, CloudFormation assumes this execution role to create a temporary session token, which it then passes to the resource type handler, thereby supplying your resource type with the appropriate credentials.</p>
    pub fn set_execution_role_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.execution_role_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the IAM role for CloudFormation to assume when invoking the extension.</p>
    /// <p>For CloudFormation to assume the specified execution role, the role must contain a trust relationship with the CloudFormation service principal (<code>resources.cloudformation.amazonaws.com</code>). For more information about adding trust relationships, see <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/roles-managingrole-editing-console.html#roles-managingrole_edit-trust-policy">Modifying a role trust policy</a> in the <i>Identity and Access Management User Guide</i>.</p>
    /// <p>If your extension calls Amazon Web Services APIs in any of its handlers, you must create an <i> <a href="https://docs.aws.amazon.com/IAM/latest/UserGuide/id_roles.html">IAM execution role</a> </i> that includes the necessary permissions to call those Amazon Web Services APIs, and provision that execution role in your account. When CloudFormation needs to invoke the resource type handler, CloudFormation assumes this execution role to create a temporary session token, which it then passes to the resource type handler, thereby supplying your resource type with the appropriate credentials.</p>
    pub fn get_execution_role_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.execution_role_arn
    }
    /// <p>A unique identifier that acts as an idempotency key for this registration request. Specifying a client request token prevents CloudFormation from generating more than one version of an extension from the same registration request, even if the request is submitted multiple times.</p>
    pub fn client_request_token(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.client_request_token = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A unique identifier that acts as an idempotency key for this registration request. Specifying a client request token prevents CloudFormation from generating more than one version of an extension from the same registration request, even if the request is submitted multiple times.</p>
    pub fn set_client_request_token(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.client_request_token = input;
        self
    }
    /// <p>A unique identifier that acts as an idempotency key for this registration request. Specifying a client request token prevents CloudFormation from generating more than one version of an extension from the same registration request, even if the request is submitted multiple times.</p>
    pub fn get_client_request_token(&self) -> &::std::option::Option<::std::string::String> {
        &self.client_request_token
    }
    /// Consumes the builder and constructs a [`RegisterTypeInput`](crate::operation::register_type::RegisterTypeInput).
    pub fn build(
        self,
    ) -> ::std::result::Result<crate::operation::register_type::RegisterTypeInput, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::operation::register_type::RegisterTypeInput {
            r#type: self.r#type,
            type_name: self.type_name,
            schema_handler_package: self.schema_handler_package,
            logging_config: self.logging_config,
            execution_role_arn: self.execution_role_arn,
            client_request_token: self.client_request_token,
        })
    }
}
