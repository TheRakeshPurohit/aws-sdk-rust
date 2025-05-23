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

pub(crate) mod shape_describe_affected_accounts_for_organization;

pub(crate) mod shape_describe_affected_entities;

pub(crate) mod shape_describe_affected_entities_for_organization;

pub(crate) mod shape_describe_entity_aggregates;

pub(crate) mod shape_describe_entity_aggregates_for_organization;

pub(crate) mod shape_describe_event_aggregates;

pub(crate) mod shape_describe_event_details;

pub(crate) mod shape_describe_event_details_for_organization;

pub(crate) mod shape_describe_event_types;

pub(crate) mod shape_describe_events;

pub(crate) mod shape_describe_events_for_organization;

pub(crate) mod shape_describe_health_service_status_for_organization;

pub(crate) mod shape_disable_health_service_access_for_organization;

pub(crate) mod shape_enable_health_service_access_for_organization;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_concurrent_modification_exception;

pub(crate) mod shape_describe_affected_accounts_for_organization_input;

pub(crate) mod shape_describe_affected_entities_for_organization_input;

pub(crate) mod shape_describe_affected_entities_input;

pub(crate) mod shape_describe_entity_aggregates_for_organization_input;

pub(crate) mod shape_describe_entity_aggregates_input;

pub(crate) mod shape_describe_event_aggregates_input;

pub(crate) mod shape_describe_event_details_for_organization_input;

pub(crate) mod shape_describe_event_details_input;

pub(crate) mod shape_describe_event_types_input;

pub(crate) mod shape_describe_events_for_organization_input;

pub(crate) mod shape_describe_events_input;

pub(crate) mod shape_invalid_pagination_token;

pub(crate) mod shape_unsupported_locale;

pub(crate) mod shape_affected_accounts_list;

pub(crate) mod shape_describe_affected_entities_for_organization_failed_set;

pub(crate) mod shape_describe_event_details_failed_set;

pub(crate) mod shape_describe_event_details_for_organization_failed_set;

pub(crate) mod shape_describe_event_details_for_organization_successful_set;

pub(crate) mod shape_describe_event_details_successful_set;

pub(crate) mod shape_entity_account_filter;

pub(crate) mod shape_entity_aggregate_list;

pub(crate) mod shape_entity_filter;

pub(crate) mod shape_entity_list;

pub(crate) mod shape_event_account_filter;

pub(crate) mod shape_event_aggregate_list;

pub(crate) mod shape_event_filter;

pub(crate) mod shape_event_list;

pub(crate) mod shape_event_type_filter;

pub(crate) mod shape_event_type_list;

pub(crate) mod shape_organization_entity_aggregates_list;

pub(crate) mod shape_organization_event_filter;

pub(crate) mod shape_organization_event_list;

pub(crate) mod shape_affected_entity;

pub(crate) mod shape_date_time_range;

pub(crate) mod shape_entity_aggregate;

pub(crate) mod shape_event;

pub(crate) mod shape_event_aggregate;

pub(crate) mod shape_event_details;

pub(crate) mod shape_event_details_error_item;

pub(crate) mod shape_event_type;

pub(crate) mod shape_organization_affected_entities_error_item;

pub(crate) mod shape_organization_entity_aggregate;

pub(crate) mod shape_organization_event;

pub(crate) mod shape_organization_event_details;

pub(crate) mod shape_organization_event_details_error_item;

pub(crate) mod shape_account_entity_aggregates_list;

pub(crate) mod shape_entity_metadata;

pub(crate) mod shape_entity_statuses;

pub(crate) mod shape_event_description;

pub(crate) mod shape_event_metadata;

pub(crate) mod shape_tag_set;

pub(crate) mod shape_account_entity_aggregate;
