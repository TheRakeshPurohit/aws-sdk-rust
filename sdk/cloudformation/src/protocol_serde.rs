// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn type_erase_result<O, E>(
    result: ::std::result::Result<O, E>,
) -> ::std::result::Result<
    ::aws_smithy_runtime_api::client::interceptors::context::Output,
    ::aws_smithy_runtime_api::client::orchestrator::OrchestratorError<::aws_smithy_runtime_api::client::interceptors::context::Error>,
>
where
    O: ::std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
    E: ::std::error::Error + std::fmt::Debug + ::std::marker::Send + ::std::marker::Sync + 'static,
{
    result
        .map(|output| ::aws_smithy_runtime_api::client::interceptors::context::Output::erase(output))
        .map_err(|error| ::aws_smithy_runtime_api::client::interceptors::context::Error::erase(error))
        .map_err(::std::convert::Into::into)
}

pub fn parse_http_error_metadata(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    response_body: &[u8],
) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_xml::decode::XmlDecodeError> {
    crate::rest_xml_wrapped_errors::parse_error_metadata(response_body)
}

pub(crate) mod shape_activate_organizations_access;

pub(crate) mod shape_activate_organizations_access_input;

pub(crate) mod shape_activate_type;

pub(crate) mod shape_activate_type_input;

pub(crate) mod shape_batch_describe_type_configurations;

pub(crate) mod shape_batch_describe_type_configurations_input;

pub(crate) mod shape_cancel_update_stack;

pub(crate) mod shape_cancel_update_stack_input;

pub(crate) mod shape_continue_update_rollback;

pub(crate) mod shape_continue_update_rollback_input;

pub(crate) mod shape_create_change_set;

pub(crate) mod shape_create_change_set_input;

pub(crate) mod shape_create_generated_template;

pub(crate) mod shape_create_generated_template_input;

pub(crate) mod shape_create_stack;

pub(crate) mod shape_create_stack_input;

pub(crate) mod shape_create_stack_instances;

pub(crate) mod shape_create_stack_instances_input;

pub(crate) mod shape_create_stack_refactor;

pub(crate) mod shape_create_stack_refactor_input;

pub(crate) mod shape_create_stack_set;

pub(crate) mod shape_create_stack_set_input;

pub(crate) mod shape_deactivate_organizations_access;

pub(crate) mod shape_deactivate_organizations_access_input;

pub(crate) mod shape_deactivate_type;

pub(crate) mod shape_deactivate_type_input;

pub(crate) mod shape_delete_change_set;

pub(crate) mod shape_delete_change_set_input;

pub(crate) mod shape_delete_generated_template;

pub(crate) mod shape_delete_generated_template_input;

pub(crate) mod shape_delete_stack;

pub(crate) mod shape_delete_stack_input;

pub(crate) mod shape_delete_stack_instances;

pub(crate) mod shape_delete_stack_instances_input;

pub(crate) mod shape_delete_stack_set;

pub(crate) mod shape_delete_stack_set_input;

pub(crate) mod shape_deregister_type;

pub(crate) mod shape_deregister_type_input;

pub(crate) mod shape_describe_account_limits;

pub(crate) mod shape_describe_account_limits_input;

pub(crate) mod shape_describe_change_set;

pub(crate) mod shape_describe_change_set_hooks;

pub(crate) mod shape_describe_change_set_hooks_input;

pub(crate) mod shape_describe_change_set_input;

pub(crate) mod shape_describe_generated_template;

pub(crate) mod shape_describe_generated_template_input;

pub(crate) mod shape_describe_organizations_access;

pub(crate) mod shape_describe_organizations_access_input;

pub(crate) mod shape_describe_publisher;

pub(crate) mod shape_describe_publisher_input;

pub(crate) mod shape_describe_resource_scan;

pub(crate) mod shape_describe_resource_scan_input;

pub(crate) mod shape_describe_stack_drift_detection_status;

pub(crate) mod shape_describe_stack_drift_detection_status_input;

pub(crate) mod shape_describe_stack_events;

pub(crate) mod shape_describe_stack_events_input;

pub(crate) mod shape_describe_stack_instance;

pub(crate) mod shape_describe_stack_instance_input;

pub(crate) mod shape_describe_stack_refactor;

pub(crate) mod shape_describe_stack_refactor_input;

pub(crate) mod shape_describe_stack_resource;

pub(crate) mod shape_describe_stack_resource_drifts;

pub(crate) mod shape_describe_stack_resource_drifts_input;

pub(crate) mod shape_describe_stack_resource_input;

pub(crate) mod shape_describe_stack_resources;

pub(crate) mod shape_describe_stack_resources_input;

pub(crate) mod shape_describe_stack_set;

pub(crate) mod shape_describe_stack_set_input;

pub(crate) mod shape_describe_stack_set_operation;

pub(crate) mod shape_describe_stack_set_operation_input;

pub(crate) mod shape_describe_stacks;

pub(crate) mod shape_describe_stacks_input;

pub(crate) mod shape_describe_type;

pub(crate) mod shape_describe_type_input;

pub(crate) mod shape_describe_type_registration;

pub(crate) mod shape_describe_type_registration_input;

pub(crate) mod shape_detect_stack_drift;

pub(crate) mod shape_detect_stack_drift_input;

pub(crate) mod shape_detect_stack_resource_drift;

pub(crate) mod shape_detect_stack_resource_drift_input;

pub(crate) mod shape_detect_stack_set_drift;

pub(crate) mod shape_detect_stack_set_drift_input;

pub(crate) mod shape_estimate_template_cost;

pub(crate) mod shape_estimate_template_cost_input;

pub(crate) mod shape_execute_change_set;

pub(crate) mod shape_execute_change_set_input;

pub(crate) mod shape_execute_stack_refactor;

pub(crate) mod shape_execute_stack_refactor_input;

pub(crate) mod shape_get_generated_template;

pub(crate) mod shape_get_generated_template_input;

pub(crate) mod shape_get_stack_policy;

pub(crate) mod shape_get_stack_policy_input;

pub(crate) mod shape_get_template;

pub(crate) mod shape_get_template_input;

pub(crate) mod shape_get_template_summary;

pub(crate) mod shape_get_template_summary_input;

pub(crate) mod shape_import_stacks_to_stack_set;

pub(crate) mod shape_import_stacks_to_stack_set_input;

pub(crate) mod shape_list_change_sets;

pub(crate) mod shape_list_change_sets_input;

pub(crate) mod shape_list_exports;

pub(crate) mod shape_list_exports_input;

pub(crate) mod shape_list_generated_templates;

pub(crate) mod shape_list_generated_templates_input;

pub(crate) mod shape_list_hook_results;

pub(crate) mod shape_list_hook_results_input;

pub(crate) mod shape_list_imports;

pub(crate) mod shape_list_imports_input;

pub(crate) mod shape_list_resource_scan_related_resources;

pub(crate) mod shape_list_resource_scan_related_resources_input;

pub(crate) mod shape_list_resource_scan_resources;

pub(crate) mod shape_list_resource_scan_resources_input;

pub(crate) mod shape_list_resource_scans;

pub(crate) mod shape_list_resource_scans_input;

pub(crate) mod shape_list_stack_instance_resource_drifts;

pub(crate) mod shape_list_stack_instance_resource_drifts_input;

pub(crate) mod shape_list_stack_instances;

pub(crate) mod shape_list_stack_instances_input;

pub(crate) mod shape_list_stack_refactor_actions;

pub(crate) mod shape_list_stack_refactor_actions_input;

pub(crate) mod shape_list_stack_refactors;

pub(crate) mod shape_list_stack_refactors_input;

pub(crate) mod shape_list_stack_resources;

pub(crate) mod shape_list_stack_resources_input;

pub(crate) mod shape_list_stack_set_auto_deployment_targets;

pub(crate) mod shape_list_stack_set_auto_deployment_targets_input;

pub(crate) mod shape_list_stack_set_operation_results;

pub(crate) mod shape_list_stack_set_operation_results_input;

pub(crate) mod shape_list_stack_set_operations;

pub(crate) mod shape_list_stack_set_operations_input;

pub(crate) mod shape_list_stack_sets;

pub(crate) mod shape_list_stack_sets_input;

pub(crate) mod shape_list_stacks;

pub(crate) mod shape_list_stacks_input;

pub(crate) mod shape_list_type_registrations;

pub(crate) mod shape_list_type_registrations_input;

pub(crate) mod shape_list_type_versions;

pub(crate) mod shape_list_type_versions_input;

pub(crate) mod shape_list_types;

pub(crate) mod shape_list_types_input;

pub(crate) mod shape_publish_type;

pub(crate) mod shape_publish_type_input;

pub(crate) mod shape_record_handler_progress;

pub(crate) mod shape_record_handler_progress_input;

pub(crate) mod shape_register_publisher;

pub(crate) mod shape_register_publisher_input;

pub(crate) mod shape_register_type;

pub(crate) mod shape_register_type_input;

pub(crate) mod shape_rollback_stack;

pub(crate) mod shape_rollback_stack_input;

pub(crate) mod shape_set_stack_policy;

pub(crate) mod shape_set_stack_policy_input;

pub(crate) mod shape_set_type_configuration;

pub(crate) mod shape_set_type_configuration_input;

pub(crate) mod shape_set_type_default_version;

pub(crate) mod shape_set_type_default_version_input;

pub(crate) mod shape_signal_resource;

pub(crate) mod shape_signal_resource_input;

pub(crate) mod shape_start_resource_scan;

pub(crate) mod shape_start_resource_scan_input;

pub(crate) mod shape_stop_stack_set_operation;

pub(crate) mod shape_stop_stack_set_operation_input;

pub(crate) mod shape_test_type;

pub(crate) mod shape_test_type_input;

pub(crate) mod shape_update_generated_template;

pub(crate) mod shape_update_generated_template_input;

pub(crate) mod shape_update_stack;

pub(crate) mod shape_update_stack_input;

pub(crate) mod shape_update_stack_instances;

pub(crate) mod shape_update_stack_instances_input;

pub(crate) mod shape_update_stack_set;

pub(crate) mod shape_update_stack_set_input;

pub(crate) mod shape_update_termination_protection;

pub(crate) mod shape_update_termination_protection_input;

pub(crate) mod shape_validate_template;

pub(crate) mod shape_validate_template_input;

pub(crate) mod shape_already_exists_exception;

pub(crate) mod shape_auto_deployment;

pub(crate) mod shape_cfn_registry_exception;

pub(crate) mod shape_change_set_not_found_exception;

pub(crate) mod shape_concurrent_resources_limit_exceeded_exception;

pub(crate) mod shape_created_but_modified_exception;

pub(crate) mod shape_deployment_targets;

pub(crate) mod shape_generated_template_not_found_exception;

pub(crate) mod shape_hook_result_not_found_exception;

pub(crate) mod shape_insufficient_capabilities_exception;

pub(crate) mod shape_invalid_change_set_status_exception;

pub(crate) mod shape_invalid_operation_exception;

pub(crate) mod shape_invalid_state_transition_exception;

pub(crate) mod shape_limit_exceeded_exception;

pub(crate) mod shape_logging_config;

pub(crate) mod shape_managed_execution;

pub(crate) mod shape_name_already_exists_exception;

pub(crate) mod shape_operation_id_already_exists_exception;

pub(crate) mod shape_operation_in_progress_exception;

pub(crate) mod shape_operation_not_found_exception;

pub(crate) mod shape_operation_result_filter;

pub(crate) mod shape_operation_status_check_failed_exception;

pub(crate) mod shape_parameter;

pub(crate) mod shape_resource_definition;

pub(crate) mod shape_resource_mapping;

pub(crate) mod shape_resource_scan_in_progress_exception;

pub(crate) mod shape_resource_scan_limit_exceeded_exception;

pub(crate) mod shape_resource_scan_not_found_exception;

pub(crate) mod shape_resource_to_import;

pub(crate) mod shape_rollback_configuration;

pub(crate) mod shape_scan_filter;

pub(crate) mod shape_scanned_resource_identifier;

pub(crate) mod shape_stack_definition;

pub(crate) mod shape_stack_instance_filter;

pub(crate) mod shape_stack_instance_not_found_exception;

pub(crate) mod shape_stack_not_found_exception;

pub(crate) mod shape_stack_refactor_not_found_exception;

pub(crate) mod shape_stack_set_not_empty_exception;

pub(crate) mod shape_stack_set_not_found_exception;

pub(crate) mod shape_stack_set_operation_preferences;

pub(crate) mod shape_stale_request_exception;

pub(crate) mod shape_tag;

pub(crate) mod shape_template_configuration;

pub(crate) mod shape_template_summary_config;

pub(crate) mod shape_token_already_exists_exception;

pub(crate) mod shape_type_configuration_identifier;

pub(crate) mod shape_type_configuration_not_found_exception;

pub(crate) mod shape_type_filters;

pub(crate) mod shape_type_not_found_exception;

pub(crate) mod shape_account_limit_list;

pub(crate) mod shape_batch_describe_type_configurations_errors;

pub(crate) mod shape_capabilities;

pub(crate) mod shape_change_set_hooks;

pub(crate) mod shape_change_set_summaries;

pub(crate) mod shape_changes;

pub(crate) mod shape_exports;

pub(crate) mod shape_hook_result_summaries;

pub(crate) mod shape_imports;

pub(crate) mod shape_notification_arns;

pub(crate) mod shape_parameter_declarations;

pub(crate) mod shape_parameters;

pub(crate) mod shape_registration_token_list;

pub(crate) mod shape_related_resources;

pub(crate) mod shape_required_activated_types;

pub(crate) mod shape_resource_details;

pub(crate) mod shape_resource_identifier_summaries;

pub(crate) mod shape_resource_location;

pub(crate) mod shape_resource_scan_summaries;

pub(crate) mod shape_resource_types;

pub(crate) mod shape_rollback_trigger;

pub(crate) mod shape_scan_filters;

pub(crate) mod shape_scanned_resources;

pub(crate) mod shape_stack_events;

pub(crate) mod shape_stack_ids;

pub(crate) mod shape_stack_instance;

pub(crate) mod shape_stack_instance_resource_drifts_summaries;

pub(crate) mod shape_stack_instance_summaries;

pub(crate) mod shape_stack_refactor_actions;

pub(crate) mod shape_stack_refactor_summaries;

pub(crate) mod shape_stack_resource_detail;

pub(crate) mod shape_stack_resource_drift;

pub(crate) mod shape_stack_resource_drifts;

pub(crate) mod shape_stack_resource_summaries;

pub(crate) mod shape_stack_resources;

pub(crate) mod shape_stack_set;

pub(crate) mod shape_stack_set_auto_deployment_target_summaries;

pub(crate) mod shape_stack_set_operation;

pub(crate) mod shape_stack_set_operation_result_summaries;

pub(crate) mod shape_stack_set_operation_summaries;

pub(crate) mod shape_stack_set_summaries;

pub(crate) mod shape_stack_summaries;

pub(crate) mod shape_stacks;

pub(crate) mod shape_stage_list;

pub(crate) mod shape_tags;

pub(crate) mod shape_template_parameters;

pub(crate) mod shape_template_progress;

pub(crate) mod shape_template_summaries;

pub(crate) mod shape_transforms_list;

pub(crate) mod shape_type_configuration_details_list;

pub(crate) mod shape_type_summaries;

pub(crate) mod shape_type_version_summaries;

pub(crate) mod shape_unprocessed_type_configurations;

pub(crate) mod shape_warnings;

pub(crate) mod shape_account_limit;

pub(crate) mod shape_batch_describe_type_configurations_error;

pub(crate) mod shape_change;

pub(crate) mod shape_change_set_hook;

pub(crate) mod shape_change_set_summary;

pub(crate) mod shape_export;

pub(crate) mod shape_hook_result_summary;

pub(crate) mod shape_module_info;

pub(crate) mod shape_organizational_unit_id_list;

pub(crate) mod shape_parameter_declaration;

pub(crate) mod shape_physical_resource_id_context;

pub(crate) mod shape_property_differences;

pub(crate) mod shape_region_list;

pub(crate) mod shape_required_activated_type;

pub(crate) mod shape_resource_detail;

pub(crate) mod shape_resource_identifier_summary;

pub(crate) mod shape_resource_scan_summary;

pub(crate) mod shape_rollback_triggers;

pub(crate) mod shape_scanned_resource;

pub(crate) mod shape_stack;

pub(crate) mod shape_stack_event;

pub(crate) mod shape_stack_instance_comprehensive_status;

pub(crate) mod shape_stack_instance_resource_drifts_summary;

pub(crate) mod shape_stack_instance_summary;

pub(crate) mod shape_stack_refactor_action;

pub(crate) mod shape_stack_refactor_summary;

pub(crate) mod shape_stack_resource;

pub(crate) mod shape_stack_resource_drift_information;

pub(crate) mod shape_stack_resource_summary;

pub(crate) mod shape_stack_set_auto_deployment_target_summary;

pub(crate) mod shape_stack_set_drift_detection_details;

pub(crate) mod shape_stack_set_operation_result_summary;

pub(crate) mod shape_stack_set_operation_status_details;

pub(crate) mod shape_stack_set_operation_summary;

pub(crate) mod shape_stack_set_summary;

pub(crate) mod shape_stack_summary;

pub(crate) mod shape_template_parameter;

pub(crate) mod shape_template_summary;

pub(crate) mod shape_type_configuration_details;

pub(crate) mod shape_type_summary;

pub(crate) mod shape_type_version_summary;

pub(crate) mod shape_account_gate_result;

pub(crate) mod shape_account_list;

pub(crate) mod shape_change_set_hook_target_details;

pub(crate) mod shape_jazz_resource_identifier_properties;

pub(crate) mod shape_logical_resource_ids;

pub(crate) mod shape_outputs;

pub(crate) mod shape_parameter_constraints;

pub(crate) mod shape_physical_resource_id_context_key_value_pair;

pub(crate) mod shape_property_difference;

pub(crate) mod shape_resource_change;

pub(crate) mod shape_resource_identifier_properties;

pub(crate) mod shape_resource_identifiers;

pub(crate) mod shape_resource_type_filters;

pub(crate) mod shape_stack_drift_information;

pub(crate) mod shape_stack_drift_information_summary;

pub(crate) mod shape_stack_refactor_tag_resources;

pub(crate) mod shape_stack_refactor_untag_resources;

pub(crate) mod shape_stack_resource_drift_information_summary;

pub(crate) mod shape_supported_major_versions;

pub(crate) mod shape_warning_details;

pub(crate) mod shape_allowed_values;

pub(crate) mod shape_change_set_hook_resource_target_details;

pub(crate) mod shape_output;

pub(crate) mod shape_resource_change_details;

pub(crate) mod shape_scope;

pub(crate) mod shape_warning_detail;

pub(crate) mod shape_resource_change_detail;

pub(crate) mod shape_warning_properties;

pub(crate) mod shape_resource_target_definition;

pub(crate) mod shape_warning_property;
