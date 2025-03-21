// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_modify_vpc_peering_connection_options_input_input_input(
    input: &crate::operation::modify_vpc_peering_connection_options::ModifyVpcPeeringConnectionOptionsInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "ModifyVpcPeeringConnectionOptions", "2016-11-15");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("AccepterPeeringConnectionOptions");
    if let Some(var_2) = &input.accepter_peering_connection_options {
        crate::protocol_serde::shape_peering_connection_options_request::ser_peering_connection_options_request(scope_1, var_2)?;
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DryRun");
    if let Some(var_4) = &input.dry_run {
        scope_3.boolean(*var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("RequesterPeeringConnectionOptions");
    if let Some(var_6) = &input.requester_peering_connection_options {
        crate::protocol_serde::shape_peering_connection_options_request::ser_peering_connection_options_request(scope_5, var_6)?;
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("VpcPeeringConnectionId");
    if let Some(var_8) = &input.vpc_peering_connection_id {
        scope_7.string(var_8);
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
