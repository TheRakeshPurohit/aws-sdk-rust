// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn replication_configuration_correct_errors(
    mut builder: crate::types::builders::ReplicationConfigurationBuilder,
) -> crate::types::builders::ReplicationConfigurationBuilder {
    if builder.rules.is_none() {
        builder.rules = Some(Default::default())
    }
    builder
}

pub(crate) fn encryption_configuration_correct_errors(
    mut builder: crate::types::builders::EncryptionConfigurationBuilder,
) -> crate::types::builders::EncryptionConfigurationBuilder {
    if builder.encryption_type.is_none() {
        builder.encryption_type = "no value was set".parse::<crate::types::EncryptionType>().ok()
    }
    builder
}

pub(crate) fn encryption_configuration_for_repository_creation_template_correct_errors(
    mut builder: crate::types::builders::EncryptionConfigurationForRepositoryCreationTemplateBuilder,
) -> crate::types::builders::EncryptionConfigurationForRepositoryCreationTemplateBuilder {
    if builder.encryption_type.is_none() {
        builder.encryption_type = "no value was set".parse::<crate::types::EncryptionType>().ok()
    }
    builder
}

pub(crate) fn image_tag_mutability_exclusion_filter_correct_errors(
    mut builder: crate::types::builders::ImageTagMutabilityExclusionFilterBuilder,
) -> crate::types::builders::ImageTagMutabilityExclusionFilterBuilder {
    if builder.filter_type.is_none() {
        builder.filter_type = "no value was set".parse::<crate::types::ImageTagMutabilityExclusionFilterType>().ok()
    }
    if builder.filter.is_none() {
        builder.filter = Some(Default::default())
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

pub(crate) fn registry_scanning_rule_correct_errors(
    mut builder: crate::types::builders::RegistryScanningRuleBuilder,
) -> crate::types::builders::RegistryScanningRuleBuilder {
    if builder.scan_frequency.is_none() {
        builder.scan_frequency = "no value was set".parse::<crate::types::ScanFrequency>().ok()
    }
    if builder.repository_filters.is_none() {
        builder.repository_filters = Some(Default::default())
    }
    builder
}

pub(crate) fn replication_rule_correct_errors(
    mut builder: crate::types::builders::ReplicationRuleBuilder,
) -> crate::types::builders::ReplicationRuleBuilder {
    if builder.destinations.is_none() {
        builder.destinations = Some(Default::default())
    }
    builder
}

pub(crate) fn scanning_repository_filter_correct_errors(
    mut builder: crate::types::builders::ScanningRepositoryFilterBuilder,
) -> crate::types::builders::ScanningRepositoryFilterBuilder {
    if builder.filter.is_none() {
        builder.filter = Some(Default::default())
    }
    if builder.filter_type.is_none() {
        builder.filter_type = "no value was set".parse::<crate::types::ScanningRepositoryFilterType>().ok()
    }
    builder
}

pub(crate) fn attribute_correct_errors(mut builder: crate::types::builders::AttributeBuilder) -> crate::types::builders::AttributeBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    builder
}

pub(crate) fn replication_destination_correct_errors(
    mut builder: crate::types::builders::ReplicationDestinationBuilder,
) -> crate::types::builders::ReplicationDestinationBuilder {
    if builder.region.is_none() {
        builder.region = Some(Default::default())
    }
    if builder.registry_id.is_none() {
        builder.registry_id = Some(Default::default())
    }
    builder
}

pub(crate) fn repository_filter_correct_errors(
    mut builder: crate::types::builders::RepositoryFilterBuilder,
) -> crate::types::builders::RepositoryFilterBuilder {
    if builder.filter.is_none() {
        builder.filter = Some(Default::default())
    }
    if builder.filter_type.is_none() {
        builder.filter_type = "no value was set".parse::<crate::types::RepositoryFilterType>().ok()
    }
    builder
}
