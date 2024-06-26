// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn instance_correct_errors(mut builder: crate::types::builders::InstanceBuilder) -> crate::types::builders::InstanceBuilder {
    if builder.id.is_none() {
        builder.id = Some(Default::default())
    }
    builder
}

pub(crate) fn dns_config_correct_errors(mut builder: crate::types::builders::DnsConfigBuilder) -> crate::types::builders::DnsConfigBuilder {
    if builder.dns_records.is_none() {
        builder.dns_records = Some(Default::default())
    }
    builder
}

pub(crate) fn health_check_config_correct_errors(
    mut builder: crate::types::builders::HealthCheckConfigBuilder,
) -> crate::types::builders::HealthCheckConfigBuilder {
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::HealthCheckType>().ok()
    }
    builder
}

pub(crate) fn tag_correct_errors(mut builder: crate::types::builders::TagBuilder) -> crate::types::builders::TagBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}

pub(crate) fn dns_record_correct_errors(mut builder: crate::types::builders::DnsRecordBuilder) -> crate::types::builders::DnsRecordBuilder {
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::RecordType>().ok()
    }
    if builder.ttl.is_none() {
        builder.ttl = Some(Default::default())
    }
    builder
}

pub(crate) fn soa_correct_errors(mut builder: crate::types::builders::SoaBuilder) -> crate::types::builders::SoaBuilder {
    if builder.ttl.is_none() {
        builder.ttl = Some(Default::default())
    }
    builder
}
