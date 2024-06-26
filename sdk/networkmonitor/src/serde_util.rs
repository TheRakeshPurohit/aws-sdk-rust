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
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::MonitorState>().ok()
    }
    builder
}

pub(crate) fn create_probe_output_output_correct_errors(
    mut builder: crate::operation::create_probe::builders::CreateProbeOutputBuilder,
) -> crate::operation::create_probe::builders::CreateProbeOutputBuilder {
    if builder.source_arn.is_none() {
        builder.source_arn = Some(Default::default())
    }
    if builder.destination.is_none() {
        builder.destination = Some(Default::default())
    }
    if builder.protocol.is_none() {
        builder.protocol = "no value was set".parse::<crate::types::Protocol>().ok()
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
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::MonitorState>().ok()
    }
    if builder.aggregation_period.is_none() {
        builder.aggregation_period = Some(Default::default())
    }
    if builder.created_at.is_none() {
        builder.created_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    if builder.modified_at.is_none() {
        builder.modified_at = Some(::aws_smithy_types::DateTime::from_fractional_secs(0, 0_f64))
    }
    builder
}

pub(crate) fn get_probe_output_output_correct_errors(
    mut builder: crate::operation::get_probe::builders::GetProbeOutputBuilder,
) -> crate::operation::get_probe::builders::GetProbeOutputBuilder {
    if builder.source_arn.is_none() {
        builder.source_arn = Some(Default::default())
    }
    if builder.destination.is_none() {
        builder.destination = Some(Default::default())
    }
    if builder.protocol.is_none() {
        builder.protocol = "no value was set".parse::<crate::types::Protocol>().ok()
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

pub(crate) fn update_monitor_output_output_correct_errors(
    mut builder: crate::operation::update_monitor::builders::UpdateMonitorOutputBuilder,
) -> crate::operation::update_monitor::builders::UpdateMonitorOutputBuilder {
    if builder.monitor_arn.is_none() {
        builder.monitor_arn = Some(Default::default())
    }
    if builder.monitor_name.is_none() {
        builder.monitor_name = Some(Default::default())
    }
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::MonitorState>().ok()
    }
    builder
}

pub(crate) fn update_probe_output_output_correct_errors(
    mut builder: crate::operation::update_probe::builders::UpdateProbeOutputBuilder,
) -> crate::operation::update_probe::builders::UpdateProbeOutputBuilder {
    if builder.source_arn.is_none() {
        builder.source_arn = Some(Default::default())
    }
    if builder.destination.is_none() {
        builder.destination = Some(Default::default())
    }
    if builder.protocol.is_none() {
        builder.protocol = "no value was set".parse::<crate::types::Protocol>().ok()
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
    if builder.state.is_none() {
        builder.state = "no value was set".parse::<crate::types::MonitorState>().ok()
    }
    builder
}

pub(crate) fn probe_correct_errors(mut builder: crate::types::builders::ProbeBuilder) -> crate::types::builders::ProbeBuilder {
    if builder.source_arn.is_none() {
        builder.source_arn = Some(Default::default())
    }
    if builder.destination.is_none() {
        builder.destination = Some(Default::default())
    }
    if builder.protocol.is_none() {
        builder.protocol = "no value was set".parse::<crate::types::Protocol>().ok()
    }
    builder
}
