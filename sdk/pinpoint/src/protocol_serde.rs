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
    response_headers: &::aws_smithy_runtime_api::http::Headers,
    response_body: &[u8],
) -> ::std::result::Result<::aws_smithy_types::error::metadata::Builder, ::aws_smithy_json::deserialize::error::DeserializeError> {
    crate::json_errors::parse_error_metadata(response_body, response_headers)
}

pub(crate) mod shape_create_app;

pub fn rest_json_unset_struct_payload() -> ::std::vec::Vec<u8> {
    b"{}"[..].into()
}

pub(crate) mod shape_create_app_input;

pub(crate) mod shape_create_campaign;

pub(crate) mod shape_create_campaign_input;

pub(crate) mod shape_create_email_template;

pub(crate) mod shape_create_email_template_input;

pub(crate) mod shape_create_export_job;

pub(crate) mod shape_create_export_job_input;

pub(crate) mod shape_create_import_job;

pub(crate) mod shape_create_import_job_input;

pub(crate) mod shape_create_in_app_template;

pub(crate) mod shape_create_in_app_template_input;

pub(crate) mod shape_create_journey;

pub(crate) mod shape_create_journey_input;

pub(crate) mod shape_create_push_template;

pub(crate) mod shape_create_push_template_input;

pub(crate) mod shape_create_recommender_configuration;

pub(crate) mod shape_create_recommender_configuration_input;

pub(crate) mod shape_create_segment;

pub(crate) mod shape_create_segment_input;

pub(crate) mod shape_create_sms_template;

pub(crate) mod shape_create_sms_template_input;

pub(crate) mod shape_create_voice_template;

pub(crate) mod shape_create_voice_template_input;

pub(crate) mod shape_delete_adm_channel;

pub(crate) mod shape_delete_apns_channel;

pub(crate) mod shape_delete_apns_sandbox_channel;

pub(crate) mod shape_delete_apns_voip_channel;

pub(crate) mod shape_delete_apns_voip_sandbox_channel;

pub(crate) mod shape_delete_app;

pub(crate) mod shape_delete_baidu_channel;

pub(crate) mod shape_delete_campaign;

pub(crate) mod shape_delete_email_channel;

pub(crate) mod shape_delete_email_template;

pub(crate) mod shape_delete_endpoint;

pub(crate) mod shape_delete_event_stream;

pub(crate) mod shape_delete_gcm_channel;

pub(crate) mod shape_delete_in_app_template;

pub(crate) mod shape_delete_journey;

pub(crate) mod shape_delete_push_template;

pub(crate) mod shape_delete_recommender_configuration;

pub(crate) mod shape_delete_segment;

pub(crate) mod shape_delete_sms_channel;

pub(crate) mod shape_delete_sms_template;

pub(crate) mod shape_delete_user_endpoints;

pub(crate) mod shape_delete_voice_channel;

pub(crate) mod shape_delete_voice_template;

pub(crate) mod shape_get_adm_channel;

pub(crate) mod shape_get_apns_channel;

pub(crate) mod shape_get_apns_sandbox_channel;

pub(crate) mod shape_get_apns_voip_channel;

pub(crate) mod shape_get_apns_voip_sandbox_channel;

pub(crate) mod shape_get_app;

pub(crate) mod shape_get_application_date_range_kpi;

pub(crate) mod shape_get_application_settings;

pub(crate) mod shape_get_apps;

pub(crate) mod shape_get_baidu_channel;

pub(crate) mod shape_get_campaign;

pub(crate) mod shape_get_campaign_activities;

pub(crate) mod shape_get_campaign_date_range_kpi;

pub(crate) mod shape_get_campaign_version;

pub(crate) mod shape_get_campaign_versions;

pub(crate) mod shape_get_campaigns;

pub(crate) mod shape_get_channels;

pub(crate) mod shape_get_email_channel;

pub(crate) mod shape_get_email_template;

pub(crate) mod shape_get_endpoint;

pub(crate) mod shape_get_event_stream;

pub(crate) mod shape_get_export_job;

pub(crate) mod shape_get_export_jobs;

pub(crate) mod shape_get_gcm_channel;

pub(crate) mod shape_get_import_job;

pub(crate) mod shape_get_import_jobs;

pub(crate) mod shape_get_in_app_messages;

pub(crate) mod shape_get_in_app_template;

pub(crate) mod shape_get_journey;

pub(crate) mod shape_get_journey_date_range_kpi;

pub(crate) mod shape_get_journey_execution_activity_metrics;

pub(crate) mod shape_get_journey_execution_metrics;

pub(crate) mod shape_get_journey_run_execution_activity_metrics;

pub(crate) mod shape_get_journey_run_execution_metrics;

pub(crate) mod shape_get_journey_runs;

pub(crate) mod shape_get_push_template;

pub(crate) mod shape_get_recommender_configuration;

pub(crate) mod shape_get_recommender_configurations;

pub(crate) mod shape_get_segment;

pub(crate) mod shape_get_segment_export_jobs;

pub(crate) mod shape_get_segment_import_jobs;

pub(crate) mod shape_get_segment_version;

pub(crate) mod shape_get_segment_versions;

pub(crate) mod shape_get_segments;

pub(crate) mod shape_get_sms_channel;

pub(crate) mod shape_get_sms_template;

pub(crate) mod shape_get_user_endpoints;

pub(crate) mod shape_get_voice_channel;

pub(crate) mod shape_get_voice_template;

pub(crate) mod shape_list_journeys;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_list_template_versions;

pub(crate) mod shape_list_templates;

pub(crate) mod shape_phone_number_validate;

pub(crate) mod shape_phone_number_validate_input;

pub(crate) mod shape_put_event_stream;

pub(crate) mod shape_put_event_stream_input;

pub(crate) mod shape_put_events;

pub(crate) mod shape_put_events_input;

pub(crate) mod shape_remove_attributes;

pub(crate) mod shape_remove_attributes_input;

pub(crate) mod shape_send_messages;

pub(crate) mod shape_send_messages_input;

pub(crate) mod shape_send_otp_message;

pub(crate) mod shape_send_otp_message_input;

pub(crate) mod shape_send_users_messages;

pub(crate) mod shape_send_users_messages_input;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_adm_channel;

pub(crate) mod shape_update_adm_channel_input;

pub(crate) mod shape_update_apns_channel;

pub(crate) mod shape_update_apns_channel_input;

pub(crate) mod shape_update_apns_sandbox_channel;

pub(crate) mod shape_update_apns_sandbox_channel_input;

pub(crate) mod shape_update_apns_voip_channel;

pub(crate) mod shape_update_apns_voip_channel_input;

pub(crate) mod shape_update_apns_voip_sandbox_channel;

pub(crate) mod shape_update_apns_voip_sandbox_channel_input;

pub(crate) mod shape_update_application_settings;

pub(crate) mod shape_update_application_settings_input;

pub(crate) mod shape_update_baidu_channel;

pub(crate) mod shape_update_baidu_channel_input;

pub(crate) mod shape_update_campaign;

pub(crate) mod shape_update_campaign_input;

pub(crate) mod shape_update_email_channel;

pub(crate) mod shape_update_email_channel_input;

pub(crate) mod shape_update_email_template;

pub(crate) mod shape_update_email_template_input;

pub(crate) mod shape_update_endpoint;

pub(crate) mod shape_update_endpoint_input;

pub(crate) mod shape_update_endpoints_batch;

pub(crate) mod shape_update_endpoints_batch_input;

pub(crate) mod shape_update_gcm_channel;

pub(crate) mod shape_update_gcm_channel_input;

pub(crate) mod shape_update_in_app_template;

pub(crate) mod shape_update_in_app_template_input;

pub(crate) mod shape_update_journey;

pub(crate) mod shape_update_journey_input;

pub(crate) mod shape_update_journey_state;

pub(crate) mod shape_update_journey_state_input;

pub(crate) mod shape_update_push_template;

pub(crate) mod shape_update_push_template_input;

pub(crate) mod shape_update_recommender_configuration;

pub(crate) mod shape_update_recommender_configuration_input;

pub(crate) mod shape_update_segment;

pub(crate) mod shape_update_segment_input;

pub(crate) mod shape_update_sms_channel;

pub(crate) mod shape_update_sms_channel_input;

pub(crate) mod shape_update_sms_template;

pub(crate) mod shape_update_sms_template_input;

pub(crate) mod shape_update_template_active_version;

pub(crate) mod shape_update_template_active_version_input;

pub(crate) mod shape_update_voice_channel;

pub(crate) mod shape_update_voice_channel_input;

pub(crate) mod shape_update_voice_template;

pub(crate) mod shape_update_voice_template_input;

pub(crate) mod shape_verify_otp_message;

pub(crate) mod shape_verify_otp_message_input;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_bad_request_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_app_output;

pub(crate) mod shape_create_campaign_output;

pub(crate) mod shape_create_email_template_output;

pub(crate) mod shape_create_export_job_output;

pub(crate) mod shape_create_import_job_output;

pub(crate) mod shape_create_in_app_template_output;

pub(crate) mod shape_create_journey_output;

pub(crate) mod shape_create_push_template_output;

pub(crate) mod shape_create_recommender_configuration_output;

pub(crate) mod shape_create_segment_output;

pub(crate) mod shape_create_sms_template_output;

pub(crate) mod shape_create_voice_template_output;

pub(crate) mod shape_delete_adm_channel_output;

pub(crate) mod shape_delete_apns_channel_output;

pub(crate) mod shape_delete_apns_sandbox_channel_output;

pub(crate) mod shape_delete_apns_voip_channel_output;

pub(crate) mod shape_delete_apns_voip_sandbox_channel_output;

pub(crate) mod shape_delete_app_output;

pub(crate) mod shape_delete_baidu_channel_output;

pub(crate) mod shape_delete_campaign_output;

pub(crate) mod shape_delete_email_channel_output;

pub(crate) mod shape_delete_email_template_output;

pub(crate) mod shape_delete_endpoint_output;

pub(crate) mod shape_delete_event_stream_output;

pub(crate) mod shape_delete_gcm_channel_output;

pub(crate) mod shape_delete_in_app_template_output;

pub(crate) mod shape_delete_journey_output;

pub(crate) mod shape_delete_push_template_output;

pub(crate) mod shape_delete_recommender_configuration_output;

pub(crate) mod shape_delete_segment_output;

pub(crate) mod shape_delete_sms_channel_output;

pub(crate) mod shape_delete_sms_template_output;

pub(crate) mod shape_delete_user_endpoints_output;

pub(crate) mod shape_delete_voice_channel_output;

pub(crate) mod shape_delete_voice_template_output;

pub(crate) mod shape_forbidden_exception;

pub(crate) mod shape_get_adm_channel_output;

pub(crate) mod shape_get_apns_channel_output;

pub(crate) mod shape_get_apns_sandbox_channel_output;

pub(crate) mod shape_get_apns_voip_channel_output;

pub(crate) mod shape_get_apns_voip_sandbox_channel_output;

pub(crate) mod shape_get_app_output;

pub(crate) mod shape_get_application_date_range_kpi_output;

pub(crate) mod shape_get_application_settings_output;

pub(crate) mod shape_get_apps_output;

pub(crate) mod shape_get_baidu_channel_output;

pub(crate) mod shape_get_campaign_activities_output;

pub(crate) mod shape_get_campaign_date_range_kpi_output;

pub(crate) mod shape_get_campaign_output;

pub(crate) mod shape_get_campaign_version_output;

pub(crate) mod shape_get_campaign_versions_output;

pub(crate) mod shape_get_campaigns_output;

pub(crate) mod shape_get_channels_output;

pub(crate) mod shape_get_email_channel_output;

pub(crate) mod shape_get_email_template_output;

pub(crate) mod shape_get_endpoint_output;

pub(crate) mod shape_get_event_stream_output;

pub(crate) mod shape_get_export_job_output;

pub(crate) mod shape_get_export_jobs_output;

pub(crate) mod shape_get_gcm_channel_output;

pub(crate) mod shape_get_import_job_output;

pub(crate) mod shape_get_import_jobs_output;

pub(crate) mod shape_get_in_app_messages_output;

pub(crate) mod shape_get_in_app_template_output;

pub(crate) mod shape_get_journey_date_range_kpi_output;

pub(crate) mod shape_get_journey_execution_activity_metrics_output;

pub(crate) mod shape_get_journey_execution_metrics_output;

pub(crate) mod shape_get_journey_output;

pub(crate) mod shape_get_journey_run_execution_activity_metrics_output;

pub(crate) mod shape_get_journey_run_execution_metrics_output;

pub(crate) mod shape_get_journey_runs_output;

pub(crate) mod shape_get_push_template_output;

pub(crate) mod shape_get_recommender_configuration_output;

pub(crate) mod shape_get_recommender_configurations_output;

pub(crate) mod shape_get_segment_export_jobs_output;

pub(crate) mod shape_get_segment_import_jobs_output;

pub(crate) mod shape_get_segment_output;

pub(crate) mod shape_get_segment_version_output;

pub(crate) mod shape_get_segment_versions_output;

pub(crate) mod shape_get_segments_output;

pub(crate) mod shape_get_sms_channel_output;

pub(crate) mod shape_get_sms_template_output;

pub(crate) mod shape_get_user_endpoints_output;

pub(crate) mod shape_get_voice_channel_output;

pub(crate) mod shape_get_voice_template_output;

pub(crate) mod shape_internal_server_error_exception;

pub(crate) mod shape_list_journeys_output;

pub(crate) mod shape_list_tags_for_resource_output;

pub(crate) mod shape_list_template_versions_output;

pub(crate) mod shape_list_templates_output;

pub(crate) mod shape_method_not_allowed_exception;

pub(crate) mod shape_not_found_exception;

pub(crate) mod shape_payload_too_large_exception;

pub(crate) mod shape_phone_number_validate_output;

pub(crate) mod shape_put_event_stream_output;

pub(crate) mod shape_put_events_output;

pub(crate) mod shape_remove_attributes_output;

pub(crate) mod shape_send_messages_output;

pub(crate) mod shape_send_otp_message_output;

pub(crate) mod shape_send_users_messages_output;

pub(crate) mod shape_too_many_requests_exception;

pub(crate) mod shape_update_adm_channel_output;

pub(crate) mod shape_update_apns_channel_output;

pub(crate) mod shape_update_apns_sandbox_channel_output;

pub(crate) mod shape_update_apns_voip_channel_output;

pub(crate) mod shape_update_apns_voip_sandbox_channel_output;

pub(crate) mod shape_update_application_settings_output;

pub(crate) mod shape_update_baidu_channel_output;

pub(crate) mod shape_update_campaign_output;

pub(crate) mod shape_update_email_channel_output;

pub(crate) mod shape_update_email_template_output;

pub(crate) mod shape_update_endpoint_output;

pub(crate) mod shape_update_endpoints_batch_output;

pub(crate) mod shape_update_gcm_channel_output;

pub(crate) mod shape_update_in_app_template_output;

pub(crate) mod shape_update_journey_output;

pub(crate) mod shape_update_journey_state_output;

pub(crate) mod shape_update_push_template_output;

pub(crate) mod shape_update_recommender_configuration_output;

pub(crate) mod shape_update_segment_output;

pub(crate) mod shape_update_sms_channel_output;

pub(crate) mod shape_update_sms_template_output;

pub(crate) mod shape_update_template_active_version_output;

pub(crate) mod shape_update_voice_channel_output;

pub(crate) mod shape_update_voice_template_output;

pub(crate) mod shape_verify_otp_message_output;

pub(crate) mod shape_activities_response;

pub(crate) mod shape_adm_channel_request;

pub(crate) mod shape_adm_channel_response;

pub(crate) mod shape_apns_channel_request;

pub(crate) mod shape_apns_channel_response;

pub(crate) mod shape_apns_sandbox_channel_request;

pub(crate) mod shape_apns_sandbox_channel_response;

pub(crate) mod shape_apns_voip_channel_request;

pub(crate) mod shape_apns_voip_channel_response;

pub(crate) mod shape_apns_voip_sandbox_channel_request;

pub(crate) mod shape_apns_voip_sandbox_channel_response;

pub(crate) mod shape_application_date_range_kpi_response;

pub(crate) mod shape_application_response;

pub(crate) mod shape_application_settings_resource;

pub(crate) mod shape_applications_response;

pub(crate) mod shape_attributes_resource;

pub(crate) mod shape_baidu_channel_request;

pub(crate) mod shape_baidu_channel_response;

pub(crate) mod shape_campaign_date_range_kpi_response;

pub(crate) mod shape_campaign_response;

pub(crate) mod shape_campaigns_response;

pub(crate) mod shape_channels_response;

pub(crate) mod shape_create_application_request;

pub(crate) mod shape_create_recommender_configuration_shape;

pub(crate) mod shape_create_template_message_body;

pub(crate) mod shape_email_channel_request;

pub(crate) mod shape_email_channel_response;

pub(crate) mod shape_email_template_request;

pub(crate) mod shape_email_template_response;

pub(crate) mod shape_endpoint_batch_request;

pub(crate) mod shape_endpoint_request;

pub(crate) mod shape_endpoint_response;

pub(crate) mod shape_endpoints_response;

pub(crate) mod shape_event_stream;

pub(crate) mod shape_events_request;

pub(crate) mod shape_events_response;

pub(crate) mod shape_export_job_request;

pub(crate) mod shape_export_job_response;

pub(crate) mod shape_export_jobs_response;

pub(crate) mod shape_gcm_channel_request;

pub(crate) mod shape_gcm_channel_response;

pub(crate) mod shape_import_job_request;

pub(crate) mod shape_import_job_response;

pub(crate) mod shape_import_jobs_response;

pub(crate) mod shape_in_app_messages_response;

pub(crate) mod shape_in_app_template_request;

pub(crate) mod shape_in_app_template_response;

pub(crate) mod shape_journey_date_range_kpi_response;

pub(crate) mod shape_journey_execution_activity_metrics_response;

pub(crate) mod shape_journey_execution_metrics_response;

pub(crate) mod shape_journey_response;

pub(crate) mod shape_journey_run_execution_activity_metrics_response;

pub(crate) mod shape_journey_run_execution_metrics_response;

pub(crate) mod shape_journey_runs_response;

pub(crate) mod shape_journey_state_request;

pub(crate) mod shape_journeys_response;

pub(crate) mod shape_list_recommender_configurations_response;

pub(crate) mod shape_message_body;

pub(crate) mod shape_message_request;

pub(crate) mod shape_message_response;

pub(crate) mod shape_number_validate_request;

pub(crate) mod shape_number_validate_response;

pub(crate) mod shape_push_notification_template_request;

pub(crate) mod shape_push_notification_template_response;

pub(crate) mod shape_recommender_configuration_response;

pub(crate) mod shape_segment_response;

pub(crate) mod shape_segments_response;

pub(crate) mod shape_send_otp_message_request_parameters;

pub(crate) mod shape_send_users_message_request;

pub(crate) mod shape_send_users_message_response;

pub(crate) mod shape_sms_channel_request;

pub(crate) mod shape_sms_channel_response;

pub(crate) mod shape_sms_template_request;

pub(crate) mod shape_sms_template_response;

pub(crate) mod shape_tags_model;

pub(crate) mod shape_template_active_version_request;

pub(crate) mod shape_template_create_message_body;

pub(crate) mod shape_template_versions_response;

pub(crate) mod shape_templates_response;

pub(crate) mod shape_update_attributes_request;

pub(crate) mod shape_update_recommender_configuration_shape;

pub(crate) mod shape_verification_response;

pub(crate) mod shape_verify_otp_message_request_parameters;

pub(crate) mod shape_voice_channel_request;

pub(crate) mod shape_voice_channel_response;

pub(crate) mod shape_voice_template_request;

pub(crate) mod shape_voice_template_response;

pub(crate) mod shape_write_application_settings_request;

pub(crate) mod shape_write_campaign_request;

pub(crate) mod shape_write_event_stream;

pub(crate) mod shape_write_journey_request;

pub(crate) mod shape_write_segment_request;

pub(crate) mod shape_activity;

pub(crate) mod shape_address_configuration;

pub(crate) mod shape_android_push_notification_template;

pub(crate) mod shape_apns_push_notification_template;

pub(crate) mod shape_application_settings_journey_limits;

pub(crate) mod shape_campaign_hook;

pub(crate) mod shape_campaign_limits;

pub(crate) mod shape_closed_days;

pub(crate) mod shape_custom_delivery_configuration;

pub(crate) mod shape_default_push_notification_template;

pub(crate) mod shape_direct_message_configuration;

pub(crate) mod shape_endpoint_batch_item;

pub(crate) mod shape_endpoint_demographic;

pub(crate) mod shape_endpoint_location;

pub(crate) mod shape_endpoint_send_configuration;

pub(crate) mod shape_endpoint_user;

pub(crate) mod shape_events_batch;

pub(crate) mod shape_in_app_message_content;

pub(crate) mod shape_journey_channel_settings;

pub(crate) mod shape_journey_limits;

pub(crate) mod shape_journey_schedule;

pub(crate) mod shape_message_configuration;

pub(crate) mod shape_message_header;

pub(crate) mod shape_open_hours;

pub(crate) mod shape_quiet_time;

pub(crate) mod shape_schedule;

pub(crate) mod shape_segment_dimensions;

pub(crate) mod shape_segment_group_list;

pub(crate) mod shape_start_condition;

pub(crate) mod shape_template_configuration;

pub(crate) mod shape_write_treatment_resource;

pub(crate) mod shape_adm_message;

pub(crate) mod shape_apns_message;

pub(crate) mod shape_attribute_dimension;

pub(crate) mod shape_baidu_message;

pub(crate) mod shape_base_kpi_result;

pub(crate) mod shape_campaign_custom_message;

pub(crate) mod shape_campaign_email_message;

pub(crate) mod shape_campaign_event_filter;

pub(crate) mod shape_campaign_in_app_message;

pub(crate) mod shape_campaign_sms_message;

pub(crate) mod shape_campaign_state;

pub(crate) mod shape_closed_days_rule;

pub(crate) mod shape_conditional_split_activity;

pub(crate) mod shape_contact_center_activity;

pub(crate) mod shape_custom_message_activity;

pub(crate) mod shape_default_message;

pub(crate) mod shape_default_push_notification_message;

pub(crate) mod shape_email_message;

pub(crate) mod shape_email_message_activity;

pub(crate) mod shape_event;

pub(crate) mod shape_event_start_condition;

pub(crate) mod shape_export_job_resource;

pub(crate) mod shape_gcm_message;

pub(crate) mod shape_holdout_activity;

pub(crate) mod shape_import_job_resource;

pub(crate) mod shape_in_app_message_body_config;

pub(crate) mod shape_in_app_message_button;

pub(crate) mod shape_in_app_message_header_config;

pub(crate) mod shape_journey_timeframe_cap;

pub(crate) mod shape_list_of_activity_response;

pub(crate) mod shape_list_of_application_response;

pub(crate) mod shape_list_of_campaign_response;

pub(crate) mod shape_list_of_endpoint_response;

pub(crate) mod shape_list_of_export_job_response;

pub(crate) mod shape_list_of_import_job_response;

pub(crate) mod shape_list_of_in_app_message_campaign;

pub(crate) mod shape_list_of_in_app_message_content;

pub(crate) mod shape_list_of_journey_response;

pub(crate) mod shape_list_of_journey_run_response;

pub(crate) mod shape_list_of_message_header;

pub(crate) mod shape_list_of_recommender_configuration_response;

pub(crate) mod shape_list_of_segment_response;

pub(crate) mod shape_list_of_string;

pub(crate) mod shape_list_of_template_response;

pub(crate) mod shape_list_of_template_version_response;

pub(crate) mod shape_list_of_timezone_estimation_methods_element;

pub(crate) mod shape_list_of_treatment_resource;

pub(crate) mod shape_map_of_activity;

pub(crate) mod shape_map_of_channel_response;

pub(crate) mod shape_map_of_double;

pub(crate) mod shape_map_of_endpoint_message_result;

pub(crate) mod shape_map_of_item_response;

pub(crate) mod shape_map_of_list_of_string;

pub(crate) mod shape_map_of_map_of_endpoint_message_result;

pub(crate) mod shape_map_of_message_result;

pub(crate) mod shape_map_of_string;

pub(crate) mod shape_message;

pub(crate) mod shape_metric_dimension;

pub(crate) mod shape_multi_conditional_split_activity;

pub(crate) mod shape_open_hours_rule;

pub(crate) mod shape_public_endpoint;

pub(crate) mod shape_push_message_activity;

pub(crate) mod shape_random_split_activity;

pub(crate) mod shape_segment_behaviors;

pub(crate) mod shape_segment_condition;

pub(crate) mod shape_segment_demographics;

pub(crate) mod shape_segment_group;

pub(crate) mod shape_segment_import_resource;

pub(crate) mod shape_segment_location;

pub(crate) mod shape_sms_message;

pub(crate) mod shape_sms_message_activity;

pub(crate) mod shape_template;

pub(crate) mod shape_voice_message;

pub(crate) mod shape_wait_activity;

pub(crate) mod shape_activity_response;

pub(crate) mod shape_channel_response;

pub(crate) mod shape_condition;

pub(crate) mod shape_default_button_configuration;

pub(crate) mod shape_endpoint_message_result;

pub(crate) mod shape_event_dimensions;

pub(crate) mod shape_event_filter;

pub(crate) mod shape_gps_point_dimension;

pub(crate) mod shape_in_app_message_campaign;

pub(crate) mod shape_item_response;

pub(crate) mod shape_journey_custom_message;

pub(crate) mod shape_journey_email_message;

pub(crate) mod shape_journey_push_message;

pub(crate) mod shape_journey_run_response;

pub(crate) mod shape_journey_sms_message;

pub(crate) mod shape_list_of_closed_days_rules;

pub(crate) mod shape_list_of_endpoint_types_element;

pub(crate) mod shape_list_of_result_row;

pub(crate) mod shape_list_of_segment_group;

pub(crate) mod shape_map_of_attribute_dimension;

pub(crate) mod shape_map_of_integer;

pub(crate) mod shape_map_of_list_of_open_hours_rules;

pub(crate) mod shape_map_of_metric_dimension;

pub(crate) mod shape_message_result;

pub(crate) mod shape_multi_conditional_branch;

pub(crate) mod shape_override_button_configuration;

pub(crate) mod shape_random_split_entry;

pub(crate) mod shape_raw_email;

pub(crate) mod shape_recency_dimension;

pub(crate) mod shape_segment_reference;

pub(crate) mod shape_session;

pub(crate) mod shape_set_dimension;

pub(crate) mod shape_simple_email;

pub(crate) mod shape_template_response;

pub(crate) mod shape_template_version_response;

pub(crate) mod shape_treatment_resource;

pub(crate) mod shape_wait_time;

pub(crate) mod shape_endpoint_item_response;

pub(crate) mod shape_gps_coordinates;

pub(crate) mod shape_in_app_campaign_schedule;

pub(crate) mod shape_in_app_message;

pub(crate) mod shape_list_of_open_hours_rules;

pub(crate) mod shape_map_of_event_item_response;

pub(crate) mod shape_result_row;

pub(crate) mod shape_simple_condition;

pub(crate) mod shape_simple_email_part;

pub(crate) mod shape_event_condition;

pub(crate) mod shape_event_item_response;

pub(crate) mod shape_list_of_multi_conditional_branch;

pub(crate) mod shape_list_of_random_split_entry;

pub(crate) mod shape_list_of_result_row_value;

pub(crate) mod shape_list_of_segment_dimensions;

pub(crate) mod shape_list_of_segment_reference;

pub(crate) mod shape_list_of_simple_condition;

pub(crate) mod shape_result_row_value;
