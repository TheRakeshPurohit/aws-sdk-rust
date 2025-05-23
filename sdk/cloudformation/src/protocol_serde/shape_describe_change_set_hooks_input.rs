// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_describe_change_set_hooks_input_input_input(
    input: &crate::operation::describe_change_set_hooks::DescribeChangeSetHooksInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "DescribeChangeSetHooks", "2010-05-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("ChangeSetName");
    if let Some(var_2) = &input.change_set_name {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("StackName");
    if let Some(var_4) = &input.stack_name {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("NextToken");
    if let Some(var_6) = &input.next_token {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("LogicalResourceId");
    if let Some(var_8) = &input.logical_resource_id {
        scope_7.string(var_8);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
