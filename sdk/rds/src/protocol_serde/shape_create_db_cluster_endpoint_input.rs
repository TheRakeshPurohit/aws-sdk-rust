// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_create_db_cluster_endpoint_input_input_input(
    input: &crate::operation::create_db_cluster_endpoint::CreateDbClusterEndpointInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    #[allow(unused_mut)]
    let mut writer = ::aws_smithy_query::QueryWriter::new(&mut out, "CreateDBClusterEndpoint", "2014-10-31");
    #[allow(unused_mut)]
    let mut scope_1 = writer.prefix("DBClusterIdentifier");
    if let Some(var_2) = &input.db_cluster_identifier {
        scope_1.string(var_2);
    }
    #[allow(unused_mut)]
    let mut scope_3 = writer.prefix("DBClusterEndpointIdentifier");
    if let Some(var_4) = &input.db_cluster_endpoint_identifier {
        scope_3.string(var_4);
    }
    #[allow(unused_mut)]
    let mut scope_5 = writer.prefix("EndpointType");
    if let Some(var_6) = &input.endpoint_type {
        scope_5.string(var_6);
    }
    #[allow(unused_mut)]
    let mut scope_7 = writer.prefix("StaticMembers");
    if let Some(var_8) = &input.static_members {
        let mut list_10 = scope_7.start_list(false, None);
        for item_9 in var_8 {
            #[allow(unused_mut)]
            let mut entry_11 = list_10.entry();
            entry_11.string(item_9);
        }
        list_10.finish();
    }
    #[allow(unused_mut)]
    let mut scope_12 = writer.prefix("ExcludedMembers");
    if let Some(var_13) = &input.excluded_members {
        let mut list_15 = scope_12.start_list(false, None);
        for item_14 in var_13 {
            #[allow(unused_mut)]
            let mut entry_16 = list_15.entry();
            entry_16.string(item_14);
        }
        list_15.finish();
    }
    #[allow(unused_mut)]
    let mut scope_17 = writer.prefix("Tags");
    if let Some(var_18) = &input.tags {
        let mut list_20 = scope_17.start_list(false, Some("Tag"));
        for item_19 in var_18 {
            #[allow(unused_mut)]
            let mut entry_21 = list_20.entry();
            crate::protocol_serde::shape_tag::ser_tag(entry_21, item_19)?;
        }
        list_20.finish();
    }
    writer.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}
