// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_allow_custom_routing_traffic_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::allow_custom_routing_traffic::AllowCustomRoutingTrafficInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.endpoint_group_arn {
        object.key("EndpointGroupArn").string(var_1.as_str());
    }
    if let Some(var_2) = &input.endpoint_id {
        object.key("EndpointId").string(var_2.as_str());
    }
    if let Some(var_3) = &input.destination_addresses {
        let mut array_4 = object.key("DestinationAddresses").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.destination_ports {
        let mut array_7 = object.key("DestinationPorts").start_array();
        for item_8 in var_6 {
            {
                array_7.value().number(
                    #[allow(clippy::useless_conversion)]
                    ::aws_smithy_types::Number::NegInt((*item_8).into()),
                );
            }
        }
        array_7.finish();
    }
    if let Some(var_9) = &input.allow_all_traffic_to_endpoint {
        object.key("AllowAllTrafficToEndpoint").boolean(*var_9);
    }
    Ok(())
}
