// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_enable_organizations_root_credentials_management_input_input_input(
    input: &crate::operation::enable_organizations_root_credentials_management::EnableOrganizationsRootCredentialsManagementInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let _ = input;
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "EnableOrganizationsRootCredentialsManagement", "2010-05-08");
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
