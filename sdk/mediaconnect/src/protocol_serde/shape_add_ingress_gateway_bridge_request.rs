// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_add_ingress_gateway_bridge_request(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::AddIngressGatewayBridgeRequest,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.max_bitrate {
        object.key("maxBitrate").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_1).into()),
        );
    }
    if let Some(var_2) = &input.max_outputs {
        object.key("maxOutputs").number(
            #[allow(clippy::useless_conversion)]
            ::aws_smithy_types::Number::NegInt((*var_2).into()),
        );
    }
    Ok(())
}
