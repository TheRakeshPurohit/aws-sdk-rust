// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_template_input_input_input(
    input: &crate::operation::create_template::CreateTemplateInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateTemplate", "2010-12-01");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Template");
    if let Some(var_2) = &input.template {
        crate::protocol_serde::shape_template::ser_template(scope_1, var_2)?;
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
