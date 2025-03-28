// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_integration_identifier(
    object_2: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::IntegrationIdentifier,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    match input {
        crate::types::IntegrationIdentifier::CustomerProfiles(inner) => {
            #[allow(unused_mut)]
            let mut object_1 = object_2.key("customerProfiles").start_object();
            crate::protocol_serde::shape_customer_profiles_integration_identifier::ser_customer_profiles_integration_identifier(
                &mut object_1,
                inner,
            )?;
            object_1.finish();
        }
        crate::types::IntegrationIdentifier::QConnect(inner) => {
            #[allow(unused_mut)]
            let mut object_2 = object_2.key("qConnect").start_object();
            crate::protocol_serde::shape_q_connect_integration_identifier::ser_q_connect_integration_identifier(&mut object_2, inner)?;
            object_2.finish();
        }
        crate::types::IntegrationIdentifier::Unknown => {
            return Err(::aws_smithy_types::error::operation::SerializationError::unknown_variant(
                "IntegrationIdentifier",
            ))
        }
    }
    Ok(())
}
