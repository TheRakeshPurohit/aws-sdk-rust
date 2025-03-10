// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_checkout_license_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::checkout_license::CheckoutLicenseInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.product_sku {
        object.key("ProductSKU").string(var_1.as_str());
    }
    if let Some(var_2) = &input.checkout_type {
        object.key("CheckoutType").string(var_2.as_str());
    }
    if let Some(var_3) = &input.key_fingerprint {
        object.key("KeyFingerprint").string(var_3.as_str());
    }
    if let Some(var_4) = &input.entitlements {
        let mut array_5 = object.key("Entitlements").start_array();
        for item_6 in var_4 {
            {
                #[allow(unused_mut)]
                let mut object_7 = array_5.value().start_object();
                crate::protocol_serde::shape_entitlement_data::ser_entitlement_data(&mut object_7, item_6)?;
                object_7.finish();
            }
        }
        array_5.finish();
    }
    if let Some(var_8) = &input.client_token {
        object.key("ClientToken").string(var_8.as_str());
    }
    if let Some(var_9) = &input.beneficiary {
        object.key("Beneficiary").string(var_9.as_str());
    }
    if let Some(var_10) = &input.node_id {
        object.key("NodeId").string(var_10.as_str());
    }
    Ok(())
}
