// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(unused_mut)]
pub fn ser_reserved_instance_limit_price(
    mut writer: ::aws_smithy_query::QueryValueWriter,
    input: &crate::types::ReservedInstanceLimitPrice,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("Amount");
    if let Some(var_2) = &input.amount {
        scope_1.number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::Float((*var_2).into()),
        );
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("CurrencyCode");
    if let Some(var_4) = &input.currency_code {
        scope_3.string(var_4.as_str());
    }
    Ok(())
}
