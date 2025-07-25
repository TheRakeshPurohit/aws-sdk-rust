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

pub(crate) mod shape_cancel_session;

pub(crate) mod shape_create_approval_team;

pub(crate) mod shape_create_identity_source;

pub(crate) mod shape_delete_identity_source;

pub(crate) mod shape_delete_inactive_approval_team_version;

pub(crate) mod shape_get_approval_team;

pub(crate) mod shape_get_identity_source;

pub(crate) mod shape_get_policy_version;

pub(crate) mod shape_get_resource_policy;

pub(crate) mod shape_get_session;

pub(crate) mod shape_list_approval_teams;

pub(crate) mod shape_list_identity_sources;

pub(crate) mod shape_list_policies;

pub(crate) mod shape_list_policy_versions;

pub(crate) mod shape_list_resource_policies;

pub(crate) mod shape_list_sessions;

pub(crate) mod shape_list_tags_for_resource;

pub(crate) mod shape_start_active_approval_team_deletion;

pub(crate) mod shape_tag_resource;

pub(crate) mod shape_untag_resource;

pub(crate) mod shape_update_approval_team;

pub(crate) fn or_empty_doc(data: &[u8]) -> &[u8] {
    if data.is_empty() {
        b"{}"
    } else {
        data
    }
}

pub(crate) mod shape_access_denied_exception;

pub(crate) mod shape_conflict_exception;

pub(crate) mod shape_create_approval_team_input;

pub(crate) mod shape_create_identity_source_input;

pub(crate) mod shape_get_resource_policy_input;

pub(crate) mod shape_internal_server_exception;

pub(crate) mod shape_invalid_parameter_exception;

pub(crate) mod shape_list_sessions_input;

pub(crate) mod shape_resource_not_found_exception;

pub(crate) mod shape_service_quota_exceeded_exception;

pub(crate) mod shape_start_active_approval_team_deletion_input;

pub(crate) mod shape_tag_resource_input;

pub(crate) mod shape_throttling_exception;

pub(crate) mod shape_too_many_tags_exception;

pub(crate) mod shape_untag_resource_input;

pub(crate) mod shape_update_approval_team_input;

pub(crate) mod shape_validation_exception;

pub(crate) mod shape_approval_strategy;

pub(crate) mod shape_approval_strategy_response;

pub(crate) mod shape_approval_team_request_approver;

pub(crate) mod shape_filter;

pub(crate) mod shape_get_approval_team_response_approvers;

pub(crate) mod shape_get_session_response_approver_responses;

pub(crate) mod shape_identity_source_parameters;

pub(crate) mod shape_identity_source_parameters_for_get;

pub(crate) mod shape_identity_sources;

pub(crate) mod shape_list_approval_teams_response_approval_teams;

pub(crate) mod shape_list_resource_policies_response_resource_policies;

pub(crate) mod shape_list_sessions_response_sessions;

pub(crate) mod shape_pending_update;

pub(crate) mod shape_policies;

pub(crate) mod shape_policies_references;

pub(crate) mod shape_policy_reference;

pub(crate) mod shape_policy_version;

pub(crate) mod shape_policy_versions;

pub(crate) mod shape_session_metadata;

pub(crate) mod shape_tags;

pub(crate) mod shape_get_approval_team_response_approver;

pub(crate) mod shape_get_session_response_approver_response;

pub(crate) mod shape_iam_identity_center;

pub(crate) mod shape_iam_identity_center_for_get;

pub(crate) mod shape_identity_source_for_list;

pub(crate) mod shape_list_approval_teams_response_approval_team;

pub(crate) mod shape_list_resource_policies_response_resource_policy;

pub(crate) mod shape_list_sessions_response_session;

pub(crate) mod shape_mof_n_approval_strategy;

pub(crate) mod shape_policy;

pub(crate) mod shape_policy_version_summary;

pub(crate) mod shape_identity_source_parameters_for_list;

pub(crate) mod shape_iam_identity_center_for_list;
