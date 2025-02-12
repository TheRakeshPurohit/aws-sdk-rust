// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn access_denied_exception_correct_errors(
    mut builder: crate::types::error::builders::AccessDeniedExceptionBuilder,
) -> crate::types::error::builders::AccessDeniedExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn conflict_exception_correct_errors(
    mut builder: crate::types::error::builders::ConflictExceptionBuilder,
) -> crate::types::error::builders::ConflictExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn internal_server_exception_correct_errors(
    mut builder: crate::types::error::builders::InternalServerExceptionBuilder,
) -> crate::types::error::builders::InternalServerExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn throttling_exception_correct_errors(
    mut builder: crate::types::error::builders::ThrottlingExceptionBuilder,
) -> crate::types::error::builders::ThrottlingExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn validation_exception_correct_errors(
    mut builder: crate::types::error::builders::ValidationExceptionBuilder,
) -> crate::types::error::builders::ValidationExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn create_landing_zone_output_output_correct_errors(
    mut builder: crate::operation::create_landing_zone::builders::CreateLandingZoneOutputBuilder,
) -> crate::operation::create_landing_zone::builders::CreateLandingZoneOutputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.operation_identifier.is_none() {
        builder.operation_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn resource_not_found_exception_correct_errors(
    mut builder: crate::types::error::builders::ResourceNotFoundExceptionBuilder,
) -> crate::types::error::builders::ResourceNotFoundExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn delete_landing_zone_output_output_correct_errors(
    mut builder: crate::operation::delete_landing_zone::builders::DeleteLandingZoneOutputBuilder,
) -> crate::operation::delete_landing_zone::builders::DeleteLandingZoneOutputBuilder {
    if builder.operation_identifier.is_none() {
        builder.operation_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn service_quota_exceeded_exception_correct_errors(
    mut builder: crate::types::error::builders::ServiceQuotaExceededExceptionBuilder,
) -> crate::types::error::builders::ServiceQuotaExceededExceptionBuilder {
    if builder.message.is_none() {
        builder.message = Some(Default::default())
    }
    builder
}

pub(crate) fn disable_baseline_output_output_correct_errors(
    mut builder: crate::operation::disable_baseline::builders::DisableBaselineOutputBuilder,
) -> crate::operation::disable_baseline::builders::DisableBaselineOutputBuilder {
    if builder.operation_identifier.is_none() {
        builder.operation_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn disable_control_output_output_correct_errors(
    mut builder: crate::operation::disable_control::builders::DisableControlOutputBuilder,
) -> crate::operation::disable_control::builders::DisableControlOutputBuilder {
    if builder.operation_identifier.is_none() {
        builder.operation_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn enable_baseline_output_output_correct_errors(
    mut builder: crate::operation::enable_baseline::builders::EnableBaselineOutputBuilder,
) -> crate::operation::enable_baseline::builders::EnableBaselineOutputBuilder {
    if builder.operation_identifier.is_none() {
        builder.operation_identifier = Some(Default::default())
    }
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    builder
}

pub(crate) fn enable_control_output_output_correct_errors(
    mut builder: crate::operation::enable_control::builders::EnableControlOutputBuilder,
) -> crate::operation::enable_control::builders::EnableControlOutputBuilder {
    if builder.operation_identifier.is_none() {
        builder.operation_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn get_baseline_output_output_correct_errors(
    mut builder: crate::operation::get_baseline::builders::GetBaselineOutputBuilder,
) -> crate::operation::get_baseline::builders::GetBaselineOutputBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    builder
}

pub(crate) fn get_baseline_operation_output_output_correct_errors(
    mut builder: crate::operation::get_baseline_operation::builders::GetBaselineOperationOutputBuilder,
) -> crate::operation::get_baseline_operation::builders::GetBaselineOperationOutputBuilder {
    if builder.baseline_operation.is_none() {
        builder.baseline_operation = {
            let builder = crate::types::builders::BaselineOperationBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn get_control_operation_output_output_correct_errors(
    mut builder: crate::operation::get_control_operation::builders::GetControlOperationOutputBuilder,
) -> crate::operation::get_control_operation::builders::GetControlOperationOutputBuilder {
    if builder.control_operation.is_none() {
        builder.control_operation = {
            let builder = crate::types::builders::ControlOperationBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn get_enabled_control_output_output_correct_errors(
    mut builder: crate::operation::get_enabled_control::builders::GetEnabledControlOutputBuilder,
) -> crate::operation::get_enabled_control::builders::GetEnabledControlOutputBuilder {
    if builder.enabled_control_details.is_none() {
        builder.enabled_control_details = {
            let builder = crate::types::builders::EnabledControlDetailsBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn get_landing_zone_output_output_correct_errors(
    mut builder: crate::operation::get_landing_zone::builders::GetLandingZoneOutputBuilder,
) -> crate::operation::get_landing_zone::builders::GetLandingZoneOutputBuilder {
    if builder.landing_zone.is_none() {
        builder.landing_zone = {
            let builder = crate::types::builders::LandingZoneDetailBuilder::default();
            crate::serde_util::landing_zone_detail_correct_errors(builder).build().ok()
        }
    }
    builder
}

pub(crate) fn get_landing_zone_operation_output_output_correct_errors(
    mut builder: crate::operation::get_landing_zone_operation::builders::GetLandingZoneOperationOutputBuilder,
) -> crate::operation::get_landing_zone_operation::builders::GetLandingZoneOperationOutputBuilder {
    if builder.operation_details.is_none() {
        builder.operation_details = {
            let builder = crate::types::builders::LandingZoneOperationDetailBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn list_baselines_output_output_correct_errors(
    mut builder: crate::operation::list_baselines::builders::ListBaselinesOutputBuilder,
) -> crate::operation::list_baselines::builders::ListBaselinesOutputBuilder {
    if builder.baselines.is_none() {
        builder.baselines = Some(Default::default())
    }
    builder
}

pub(crate) fn list_control_operations_output_output_correct_errors(
    mut builder: crate::operation::list_control_operations::builders::ListControlOperationsOutputBuilder,
) -> crate::operation::list_control_operations::builders::ListControlOperationsOutputBuilder {
    if builder.control_operations.is_none() {
        builder.control_operations = Some(Default::default())
    }
    builder
}

pub(crate) fn list_enabled_baselines_output_output_correct_errors(
    mut builder: crate::operation::list_enabled_baselines::builders::ListEnabledBaselinesOutputBuilder,
) -> crate::operation::list_enabled_baselines::builders::ListEnabledBaselinesOutputBuilder {
    if builder.enabled_baselines.is_none() {
        builder.enabled_baselines = Some(Default::default())
    }
    builder
}

pub(crate) fn list_enabled_controls_output_output_correct_errors(
    mut builder: crate::operation::list_enabled_controls::builders::ListEnabledControlsOutputBuilder,
) -> crate::operation::list_enabled_controls::builders::ListEnabledControlsOutputBuilder {
    if builder.enabled_controls.is_none() {
        builder.enabled_controls = Some(Default::default())
    }
    builder
}

pub(crate) fn list_landing_zone_operations_output_output_correct_errors(
    mut builder: crate::operation::list_landing_zone_operations::builders::ListLandingZoneOperationsOutputBuilder,
) -> crate::operation::list_landing_zone_operations::builders::ListLandingZoneOperationsOutputBuilder {
    if builder.landing_zone_operations.is_none() {
        builder.landing_zone_operations = Some(Default::default())
    }
    builder
}

pub(crate) fn list_landing_zones_output_output_correct_errors(
    mut builder: crate::operation::list_landing_zones::builders::ListLandingZonesOutputBuilder,
) -> crate::operation::list_landing_zones::builders::ListLandingZonesOutputBuilder {
    if builder.landing_zones.is_none() {
        builder.landing_zones = Some(Default::default())
    }
    builder
}

pub(crate) fn list_tags_for_resource_output_output_correct_errors(
    mut builder: crate::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder,
) -> crate::operation::list_tags_for_resource::builders::ListTagsForResourceOutputBuilder {
    if builder.tags.is_none() {
        builder.tags = Some(Default::default())
    }
    builder
}

pub(crate) fn reset_enabled_baseline_output_output_correct_errors(
    mut builder: crate::operation::reset_enabled_baseline::builders::ResetEnabledBaselineOutputBuilder,
) -> crate::operation::reset_enabled_baseline::builders::ResetEnabledBaselineOutputBuilder {
    if builder.operation_identifier.is_none() {
        builder.operation_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn reset_enabled_control_output_output_correct_errors(
    mut builder: crate::operation::reset_enabled_control::builders::ResetEnabledControlOutputBuilder,
) -> crate::operation::reset_enabled_control::builders::ResetEnabledControlOutputBuilder {
    if builder.operation_identifier.is_none() {
        builder.operation_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn reset_landing_zone_output_output_correct_errors(
    mut builder: crate::operation::reset_landing_zone::builders::ResetLandingZoneOutputBuilder,
) -> crate::operation::reset_landing_zone::builders::ResetLandingZoneOutputBuilder {
    if builder.operation_identifier.is_none() {
        builder.operation_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn update_enabled_baseline_output_output_correct_errors(
    mut builder: crate::operation::update_enabled_baseline::builders::UpdateEnabledBaselineOutputBuilder,
) -> crate::operation::update_enabled_baseline::builders::UpdateEnabledBaselineOutputBuilder {
    if builder.operation_identifier.is_none() {
        builder.operation_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn update_enabled_control_output_output_correct_errors(
    mut builder: crate::operation::update_enabled_control::builders::UpdateEnabledControlOutputBuilder,
) -> crate::operation::update_enabled_control::builders::UpdateEnabledControlOutputBuilder {
    if builder.operation_identifier.is_none() {
        builder.operation_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn update_landing_zone_output_output_correct_errors(
    mut builder: crate::operation::update_landing_zone::builders::UpdateLandingZoneOutputBuilder,
) -> crate::operation::update_landing_zone::builders::UpdateLandingZoneOutputBuilder {
    if builder.operation_identifier.is_none() {
        builder.operation_identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn landing_zone_detail_correct_errors(
    mut builder: crate::types::builders::LandingZoneDetailBuilder,
) -> crate::types::builders::LandingZoneDetailBuilder {
    if builder.version.is_none() {
        builder.version = Some(Default::default())
    }
    if builder.manifest.is_none() {
        builder.manifest = Some(Default::default())
    }
    builder
}

pub(crate) fn enabled_baseline_details_correct_errors(
    mut builder: crate::types::builders::EnabledBaselineDetailsBuilder,
) -> crate::types::builders::EnabledBaselineDetailsBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.baseline_identifier.is_none() {
        builder.baseline_identifier = Some(Default::default())
    }
    if builder.target_identifier.is_none() {
        builder.target_identifier = Some(Default::default())
    }
    if builder.status_summary.is_none() {
        builder.status_summary = {
            let builder = crate::types::builders::EnablementStatusSummaryBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn baseline_summary_correct_errors(
    mut builder: crate::types::builders::BaselineSummaryBuilder,
) -> crate::types::builders::BaselineSummaryBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.name.is_none() {
        builder.name = Some(Default::default())
    }
    builder
}

pub(crate) fn enabled_baseline_summary_correct_errors(
    mut builder: crate::types::builders::EnabledBaselineSummaryBuilder,
) -> crate::types::builders::EnabledBaselineSummaryBuilder {
    if builder.arn.is_none() {
        builder.arn = Some(Default::default())
    }
    if builder.baseline_identifier.is_none() {
        builder.baseline_identifier = Some(Default::default())
    }
    if builder.target_identifier.is_none() {
        builder.target_identifier = Some(Default::default())
    }
    if builder.status_summary.is_none() {
        builder.status_summary = {
            let builder = crate::types::builders::EnablementStatusSummaryBuilder::default();
            Some(builder.build())
        }
    }
    builder
}

pub(crate) fn enabled_baseline_parameter_summary_correct_errors(
    mut builder: crate::types::builders::EnabledBaselineParameterSummaryBuilder,
) -> crate::types::builders::EnabledBaselineParameterSummaryBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}

pub(crate) fn enabled_control_parameter_summary_correct_errors(
    mut builder: crate::types::builders::EnabledControlParameterSummaryBuilder,
) -> crate::types::builders::EnabledControlParameterSummaryBuilder {
    if builder.key.is_none() {
        builder.key = Some(Default::default())
    }
    if builder.value.is_none() {
        builder.value = Some(Default::default())
    }
    builder
}
