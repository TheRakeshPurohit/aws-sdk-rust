// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_policy_http_payload(
    payload: &::std::option::Option<crate::types::VaultLockPolicy>,
) -> ::std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::BuildError> {
    let payload = match payload.as_ref() {
        Some(t) => t,
        None => return Ok(crate::protocol_serde::rest_json_unset_struct_payload()),
    };
    Ok(crate::protocol_serde::shape_initiate_vault_lock_input::ser_policy_payload(payload)?)
}

pub fn ser_policy_payload(
    input: &crate::types::VaultLockPolicy,
) -> std::result::Result<::std::vec::Vec<u8>, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_vault_lock_policy::ser_vault_lock_policy(&mut object, input)?;
    object.finish();
    Ok(out.into_bytes())
}
