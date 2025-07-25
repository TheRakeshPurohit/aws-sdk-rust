# aws-sdk-opsworks

Welcome to the _OpsWorks Stacks API Reference_. This guide provides descriptions, syntax, and usage examples for OpsWorks Stacks actions and data types, including common parameters and error codes.

OpsWorks Stacks is an application management service that provides an integrated experience for managing the complete application lifecycle. For information about OpsWorks, see the [OpsWorks](http://aws.amazon.com/opsworks/) information page.

__SDKs and CLI__

Use the OpsWorks Stacks API by using the Command Line Interface (CLI) or by using one of the Amazon Web Services SDKs to implement applications in your preferred language. For more information, see:
  - [CLI](https://docs.aws.amazon.com/cli/latest/userguide/cli-chap-welcome.html)
  - [SDK for Java](https://docs.aws.amazon.com/AWSJavaSDK/latest/javadoc/com/amazonaws/services/opsworks/AWSOpsWorksClient.html)
  - [SDK for .NET](https://docs.aws.amazon.com/sdkfornet/v3/apidocs/items/OpsWorks/NOpsWorks.html)
  - [SDK for PHP](https://docs.aws.amazon.com/aws-sdk-php/v3/api/class-Aws.OpsWorks.OpsWorksClient.html)
  - [SDK for Ruby](http://docs.aws.amazon.com/sdkforruby/api/)
  - [Amazon Web Services SDK for Node.js](http://aws.amazon.com/documentation/sdkforjavascript/)
  - [SDK for Python (Boto)](http://docs.pythonboto.org/en/latest/ref/opsworks.html)

__Endpoints__

OpsWorks Stacks supports the following endpoints, all HTTPS. You must connect to one of the following endpoints. Stacks can only be accessed or managed within the endpoint in which they are created.
  - opsworks.us-east-1.amazonaws.com
  - opsworks.us-east-2.amazonaws.com
  - opsworks.us-west-1.amazonaws.com
  - opsworks.us-west-2.amazonaws.com
  - opsworks.ca-central-1.amazonaws.com (API only; not available in the Amazon Web Services Management Console)
  - opsworks.eu-west-1.amazonaws.com
  - opsworks.eu-west-2.amazonaws.com
  - opsworks.eu-west-3.amazonaws.com
  - opsworks.eu-central-1.amazonaws.com
  - opsworks.ap-northeast-1.amazonaws.com
  - opsworks.ap-northeast-2.amazonaws.com
  - opsworks.ap-south-1.amazonaws.com
  - opsworks.ap-southeast-1.amazonaws.com
  - opsworks.ap-southeast-2.amazonaws.com
  - opsworks.sa-east-1.amazonaws.com

__Chef Versions__

When you call CreateStack, CloneStack, or UpdateStack we recommend you use the ConfigurationManager parameter to specify the Chef version. The recommended and default value for Linux stacks is currently 12. Windows stacks use Chef 12.2. For more information, see [Chef Versions](https://docs.aws.amazon.com/opsworks/latest/userguide/workingcookbook-chef11.html).

## Getting Started

> Examples are available for many services and operations, check out the
> [examples folder in GitHub](https://github.com/awslabs/aws-sdk-rust/tree/main/examples).

The SDK provides one crate per AWS service. You must add [Tokio](https://crates.io/crates/tokio)
as a dependency within your Rust project to execute asynchronous code. To add `aws-sdk-opsworks` to
your project, add the following to your **Cargo.toml** file:

```toml
[dependencies]
aws-config = { version = "1.1.7", features = ["behavior-version-latest"] }
aws-sdk-opsworks = "1.78.0"
tokio = { version = "1", features = ["full"] }
```

Then in code, a client can be created with the following:

```rust,no_run
use aws_sdk_opsworks as opsworks;

#[::tokio::main]
async fn main() -> Result<(), opsworks::Error> {
    let config = aws_config::load_from_env().await;
    let client = aws_sdk_opsworks::Client::new(&config);

    // ... make some calls with the client

    Ok(())
}
```

See the [client documentation](https://docs.rs/aws-sdk-opsworks/latest/aws_sdk_opsworks/client/struct.Client.html)
for information on what calls can be made, and the inputs and outputs for each of those calls.

## Using the SDK

Until the SDK is released, we will be adding information about using the SDK to the
[Developer Guide](https://docs.aws.amazon.com/sdk-for-rust/latest/dg/welcome.html). Feel free to suggest
additional sections for the guide by opening an issue and describing what you are trying to do.

## Getting Help

* [GitHub discussions](https://github.com/awslabs/aws-sdk-rust/discussions) - For ideas, RFCs & general questions
* [GitHub issues](https://github.com/awslabs/aws-sdk-rust/issues/new/choose) - For bug reports & feature requests
* [Generated Docs (latest version)](https://awslabs.github.io/aws-sdk-rust/)
* [Usage examples](https://github.com/awslabs/aws-sdk-rust/tree/main/examples)

## License

This project is licensed under the Apache-2.0 License.

