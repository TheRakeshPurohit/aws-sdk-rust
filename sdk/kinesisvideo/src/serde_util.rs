// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn edge_config_correct_errors(mut builder: crate::types::builders::EdgeConfigBuilder) -> crate::types::builders::EdgeConfigBuilder {
    if builder.hub_device_arn.is_none() {
        builder.hub_device_arn = Some(Default::default())
    }
    if builder.recorder_config.is_none() {
        builder.recorder_config = {
            let builder = crate::types::builders::RecorderConfigBuilder::default();
            Some(crate::serde_util::recorder_config_correct_errors(builder).build())
        }
    }
    builder
}

pub(crate) fn image_generation_configuration_correct_errors(
    mut builder: crate::types::builders::ImageGenerationConfigurationBuilder,
) -> crate::types::builders::ImageGenerationConfigurationBuilder {
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::ConfigurationStatus>().ok()
    }
    if builder.image_selector_type.is_none() {
        builder.image_selector_type = "no value was set".parse::<crate::types::ImageSelectorType>().ok()
    }
    if builder.destination_config.is_none() {
        builder.destination_config = {
            let builder = crate::types::builders::ImageGenerationDestinationConfigBuilder::default();
            crate::serde_util::image_generation_destination_config_correct_errors(builder)
                .build()
                .ok()
        }
    }
    if builder.sampling_interval.is_none() {
        builder.sampling_interval = Some(Default::default())
    }
    if builder.format.is_none() {
        builder.format = "no value was set".parse::<crate::types::Format>().ok()
    }
    builder
}

pub(crate) fn media_storage_configuration_correct_errors(
    mut builder: crate::types::builders::MediaStorageConfigurationBuilder,
) -> crate::types::builders::MediaStorageConfigurationBuilder {
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::MediaStorageConfigurationStatus>().ok()
    }
    builder
}

pub(crate) fn notification_configuration_correct_errors(
    mut builder: crate::types::builders::NotificationConfigurationBuilder,
) -> crate::types::builders::NotificationConfigurationBuilder {
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::ConfigurationStatus>().ok()
    }
    if builder.destination_config.is_none() {
        builder.destination_config = {
            let builder = crate::types::builders::NotificationDestinationConfigBuilder::default();
            crate::serde_util::notification_destination_config_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn image_generation_destination_config_correct_errors(
    mut builder: crate::types::builders::ImageGenerationDestinationConfigBuilder,
) -> crate::types::builders::ImageGenerationDestinationConfigBuilder {
    if builder.uri.is_none() {
        builder.uri = Some(Default::default())
    }
    if builder.destination_region.is_none() {
        builder.destination_region = Some(Default::default())
    }
    builder
}

pub(crate) fn notification_destination_config_correct_errors(
    mut builder: crate::types::builders::NotificationDestinationConfigBuilder,
) -> crate::types::builders::NotificationDestinationConfigBuilder {
    if builder.uri.is_none() {
        builder.uri = Some(Default::default())
    }
    builder
}

pub(crate) fn recorder_config_correct_errors(
    mut builder: crate::types::builders::RecorderConfigBuilder,
) -> crate::types::builders::RecorderConfigBuilder {
    if builder.media_source_config.is_none() {
        builder.media_source_config = {
            let builder = crate::types::builders::MediaSourceConfigBuilder::default();
            crate::serde_util::media_source_config_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn uploader_config_correct_errors(
    mut builder: crate::types::builders::UploaderConfigBuilder,
) -> crate::types::builders::UploaderConfigBuilder {
    if builder.schedule_config.is_none() {
        builder.schedule_config = {
            let builder = crate::types::builders::ScheduleConfigBuilder::default();
            crate::serde_util::schedule_config_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn media_source_config_correct_errors(
    mut builder: crate::types::builders::MediaSourceConfigBuilder,
) -> crate::types::builders::MediaSourceConfigBuilder {
    if builder.media_uri_secret_arn.is_none() {
        builder.media_uri_secret_arn = Some(Default::default())
    }
    if builder.media_uri_type.is_none() {
        builder.media_uri_type = "no value was set".parse::<crate::types::MediaUriType>().ok()
    }
    builder
}

pub(crate) fn schedule_config_correct_errors(
    mut builder: crate::types::builders::ScheduleConfigBuilder,
) -> crate::types::builders::ScheduleConfigBuilder {
    if builder.schedule_expression.is_none() {
        builder.schedule_expression = Some(Default::default())
    }
    if builder.duration_in_seconds.is_none() {
        builder.duration_in_seconds = Some(Default::default())
    }
    builder
}
