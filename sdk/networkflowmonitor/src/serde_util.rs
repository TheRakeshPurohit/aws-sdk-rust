// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn create_monitor_output_output_correct_errors(
    mut builder: crate::operation::create_monitor::builders::CreateMonitorOutputBuilder,
) -> crate::operation::create_monitor::builders::CreateMonitorOutputBuilder {
    if builder.monitor_arn.is_none() {
        builder.monitor_arn = Some(Default::default())
    }
    if builder.monitor_name.is_none() {
        builder.monitor_name = Some(Default::default())
    }
    if builder.monitor_status.is_none() {
        builder.monitor_status = "no value was set".parse::<crate::types::MonitorStatus>().ok()
    }
    if builder.local_resources.is_none() {
        builder.local_resources = Some(Default::default())
    }
    if builder.remote_resources.is_none() {
        builder.remote_resources = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.modified_at.is_none() {
        builder.modified_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn create_scope_output_output_correct_errors(
    mut builder: crate::operation::create_scope::builders::CreateScopeOutputBuilder,
) -> crate::operation::create_scope::builders::CreateScopeOutputBuilder {
    if builder.scope_id.is_none() {
        builder.scope_id = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::ScopeStatus>().ok()
    }
    if builder.scope_arn.is_none() {
        builder.scope_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn get_monitor_output_output_correct_errors(
    mut builder: crate::operation::get_monitor::builders::GetMonitorOutputBuilder,
) -> crate::operation::get_monitor::builders::GetMonitorOutputBuilder {
    if builder.monitor_arn.is_none() {
        builder.monitor_arn = Some(Default::default())
    }
    if builder.monitor_name.is_none() {
        builder.monitor_name = Some(Default::default())
    }
    if builder.monitor_status.is_none() {
        builder.monitor_status = "no value was set".parse::<crate::types::MonitorStatus>().ok()
    }
    if builder.local_resources.is_none() {
        builder.local_resources = Some(Default::default())
    }
    if builder.remote_resources.is_none() {
        builder.remote_resources = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.modified_at.is_none() {
        builder.modified_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn get_query_results_workload_insights_top_contributors_data_output_output_correct_errors(mut builder: crate::operation::get_query_results_workload_insights_top_contributors_data::builders::GetQueryResultsWorkloadInsightsTopContributorsDataOutputBuilder) -> crate::operation::get_query_results_workload_insights_top_contributors_data::builders::GetQueryResultsWorkloadInsightsTopContributorsDataOutputBuilder{
    if builder.unit.is_none() {
        builder.unit = "no value was set".parse::<crate::types::MetricUnit>().ok()
    }
    if builder.datapoints.is_none() {
        builder.datapoints = Some(Default::default())
    }
    builder
}

pub(crate) fn get_query_status_monitor_top_contributors_output_output_correct_errors(
    mut builder: crate::operation::get_query_status_monitor_top_contributors::builders::GetQueryStatusMonitorTopContributorsOutputBuilder,
) -> crate::operation::get_query_status_monitor_top_contributors::builders::GetQueryStatusMonitorTopContributorsOutputBuilder {
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::QueryStatus>().ok()
    }
    builder
}

pub(crate) fn get_query_status_workload_insights_top_contributors_output_output_correct_errors(
    mut builder: crate::operation::get_query_status_workload_insights_top_contributors::builders::GetQueryStatusWorkloadInsightsTopContributorsOutputBuilder,
) -> crate::operation::get_query_status_workload_insights_top_contributors::builders::GetQueryStatusWorkloadInsightsTopContributorsOutputBuilder {
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::QueryStatus>().ok()
    }
    builder
}

pub(crate) fn get_query_status_workload_insights_top_contributors_data_output_output_correct_errors(mut builder: crate::operation::get_query_status_workload_insights_top_contributors_data::builders::GetQueryStatusWorkloadInsightsTopContributorsDataOutputBuilder) -> crate::operation::get_query_status_workload_insights_top_contributors_data::builders::GetQueryStatusWorkloadInsightsTopContributorsDataOutputBuilder{
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::QueryStatus>().ok()
    }
    builder
}

pub(crate) fn get_scope_output_output_correct_errors(
    mut builder: crate::operation::get_scope::builders::GetScopeOutputBuilder,
) -> crate::operation::get_scope::builders::GetScopeOutputBuilder {
    if builder.scope_id.is_none() {
        builder.scope_id = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::ScopeStatus>().ok()
    }
    if builder.scope_arn.is_none() {
        builder.scope_arn = Some(Default::default())
    }
    if builder.targets.is_none() {
        builder.targets = Some(Default::default())
    }
    builder
}

pub(crate) fn list_monitors_output_output_correct_errors(
    mut builder: crate::operation::list_monitors::builders::ListMonitorsOutputBuilder,
) -> crate::operation::list_monitors::builders::ListMonitorsOutputBuilder {
    if builder.monitors.is_none() {
        builder.monitors = Some(Default::default())
    }
    builder
}

pub(crate) fn list_scopes_output_output_correct_errors(
    mut builder: crate::operation::list_scopes::builders::ListScopesOutputBuilder,
) -> crate::operation::list_scopes::builders::ListScopesOutputBuilder {
    if builder.scopes.is_none() {
        builder.scopes = Some(Default::default())
    }
    builder
}

pub(crate) fn start_query_monitor_top_contributors_output_output_correct_errors(
    mut builder: crate::operation::start_query_monitor_top_contributors::builders::StartQueryMonitorTopContributorsOutputBuilder,
) -> crate::operation::start_query_monitor_top_contributors::builders::StartQueryMonitorTopContributorsOutputBuilder {
    if builder.query_id.is_none() {
        builder.query_id = Some(Default::default())
    }
    builder
}

pub(crate) fn start_query_workload_insights_top_contributors_output_output_correct_errors(
    mut builder: crate::operation::start_query_workload_insights_top_contributors::builders::StartQueryWorkloadInsightsTopContributorsOutputBuilder,
) -> crate::operation::start_query_workload_insights_top_contributors::builders::StartQueryWorkloadInsightsTopContributorsOutputBuilder {
    if builder.query_id.is_none() {
        builder.query_id = Some(Default::default())
    }
    builder
}

pub(crate) fn start_query_workload_insights_top_contributors_data_output_output_correct_errors(
    mut builder: crate::operation::start_query_workload_insights_top_contributors_data::builders::StartQueryWorkloadInsightsTopContributorsDataOutputBuilder,
) -> crate::operation::start_query_workload_insights_top_contributors_data::builders::StartQueryWorkloadInsightsTopContributorsDataOutputBuilder {
    if builder.query_id.is_none() {
        builder.query_id = Some(Default::default())
    }
    builder
}

pub(crate) fn update_monitor_output_output_correct_errors(
    mut builder: crate::operation::update_monitor::builders::UpdateMonitorOutputBuilder,
) -> crate::operation::update_monitor::builders::UpdateMonitorOutputBuilder {
    if builder.monitor_arn.is_none() {
        builder.monitor_arn = Some(Default::default())
    }
    if builder.monitor_name.is_none() {
        builder.monitor_name = Some(Default::default())
    }
    if builder.monitor_status.is_none() {
        builder.monitor_status = "no value was set".parse::<crate::types::MonitorStatus>().ok()
    }
    if builder.local_resources.is_none() {
        builder.local_resources = Some(Default::default())
    }
    if builder.remote_resources.is_none() {
        builder.remote_resources = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.modified_at.is_none() {
        builder.modified_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn update_scope_output_output_correct_errors(
    mut builder: crate::operation::update_scope::builders::UpdateScopeOutputBuilder,
) -> crate::operation::update_scope::builders::UpdateScopeOutputBuilder {
    if builder.scope_id.is_none() {
        builder.scope_id = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::ScopeStatus>().ok()
    }
    if builder.scope_arn.is_none() {
        builder.scope_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn monitor_local_resource_correct_errors(
    mut builder: crate::types::builders::MonitorLocalResourceBuilder,
) -> crate::types::builders::MonitorLocalResourceBuilder {
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::MonitorLocalResourceType>().ok()
    }
    if builder.identifier.is_none() {
        builder.identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn monitor_remote_resource_correct_errors(
    mut builder: crate::types::builders::MonitorRemoteResourceBuilder,
) -> crate::types::builders::MonitorRemoteResourceBuilder {
    if builder.r#type.is_none() {
        builder.r#type = "no value was set".parse::<crate::types::MonitorRemoteResourceType>().ok()
    }
    if builder.identifier.is_none() {
        builder.identifier = Some(Default::default())
    }
    builder
}

pub(crate) fn monitor_summary_correct_errors(
    mut builder: crate::types::builders::MonitorSummaryBuilder,
) -> crate::types::builders::MonitorSummaryBuilder {
    if builder.monitor_arn.is_none() {
        builder.monitor_arn = Some(Default::default())
    }
    if builder.monitor_name.is_none() {
        builder.monitor_name = Some(Default::default())
    }
    if builder.monitor_status.is_none() {
        builder.monitor_status = "no value was set".parse::<crate::types::MonitorStatus>().ok()
    }
    builder
}

pub(crate) fn scope_summary_correct_errors(mut builder: crate::types::builders::ScopeSummaryBuilder) -> crate::types::builders::ScopeSummaryBuilder {
    if builder.scope_id.is_none() {
        builder.scope_id = Some(Default::default())
    }
    if builder.status.is_none() {
        builder.status = "no value was set".parse::<crate::types::ScopeStatus>().ok()
    }
    if builder.scope_arn.is_none() {
        builder.scope_arn = Some(Default::default())
    }
    builder
}

pub(crate) fn target_resource_correct_errors(
    mut builder: crate::types::builders::TargetResourceBuilder,
) -> crate::types::builders::TargetResourceBuilder {
    if builder.target_identifier.is_none() {
        builder.target_identifier = {
            let builder = crate::types::builders::TargetIdentifierBuilder::default();
            crate::serde_util::target_identifier_correct_errors(builder).build().ok()
        }
    }
    if builder.region.is_none() {
        builder.region = Some(Default::default())
    }
    builder
}

pub(crate) fn workload_insights_top_contributors_data_point_correct_errors(
    mut builder: crate::types::builders::WorkloadInsightsTopContributorsDataPointBuilder,
) -> crate::types::builders::WorkloadInsightsTopContributorsDataPointBuilder {
    if builder.timestamps.is_none() {
        builder.timestamps = Some(Default::default())
    }
    if builder.values.is_none() {
        builder.values = Some(Default::default())
    }
    if builder.label.is_none() {
        builder.label = Some(Default::default())
    }
    builder
}

pub(crate) fn target_identifier_correct_errors(
    mut builder: crate::types::builders::TargetIdentifierBuilder,
) -> crate::types::builders::TargetIdentifierBuilder {
    if builder.target_id.is_none() {
        builder.target_id = Some(crate::types::TargetId::Unknown)
    }
    if builder.target_type.is_none() {
        builder.target_type = "no value was set".parse::<crate::types::TargetType>().ok()
    }
    builder
}
