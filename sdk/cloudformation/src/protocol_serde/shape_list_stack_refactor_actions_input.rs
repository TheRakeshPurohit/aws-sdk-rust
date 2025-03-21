// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_list_stack_refactor_actions_input_input_input(
    input: &crate::operation::list_stack_refactor_actions::ListStackRefactorActionsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ListStackRefactorActions", "2010-05-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("StackRefactorId");
    if let Some(var_2) = &input.stack_refactor_id {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("NextToken");
    if let Some(var_4) = &input.next_token {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("MaxResults");
    if let Some(var_6) = &input.max_results {
        scope_5.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_6).into()),
        );
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
