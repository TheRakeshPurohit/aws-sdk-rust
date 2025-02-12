// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Matcher union: {"output":{"path":"status","expected":"ACTIVE","comparator":"stringEquals"}}
pub(crate) fn match_get_cluster_a0b9c099115634691(
    _result: ::std::result::Result<&crate::operation::get_cluster::GetClusterOutput, &crate::operation::get_cluster::GetClusterError>,
) -> bool {
    fn path_traversal<'a>(_output: &'a crate::operation::get_cluster::GetClusterOutput) -> ::std::option::Option<&'a crate::types::ClusterStatus> {
        let _fld_1 = &_output.status;
        ::std::option::Option::Some(_fld_1)
    }
    _result
        .as_ref()
        .ok()
        .and_then(|output| path_traversal(output))
        .map(|value| {
            let _tmp_2 = value.as_str();
            let right = "ACTIVE";
            let _cmp_1 = _tmp_2 == right;
            _cmp_1
        })
        .unwrap_or_default()
}

/// Matcher union: {"errorType":"ResourceNotFoundException"}
pub(crate) fn match_get_cluster_1cce2c05524fb92d4(
    _result: ::std::result::Result<&crate::operation::get_cluster::GetClusterOutput, &crate::operation::get_cluster::GetClusterError>,
) -> bool {
    if let ::std::result::Result::Err(err) = _result {
        if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(err) {
            return code == "ResourceNotFoundException";
        }
    }
    false
}
