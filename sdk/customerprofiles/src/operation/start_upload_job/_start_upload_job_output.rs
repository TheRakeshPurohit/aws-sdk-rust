// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct StartUploadJobOutput {
    _request_id: Option<String>,
}
impl ::aws_types::request_id::RequestId for StartUploadJobOutput {
    fn request_id(&self) -> Option<&str> {
        self._request_id.as_deref()
    }
}
impl StartUploadJobOutput {
    /// Creates a new builder-style object to manufacture [`StartUploadJobOutput`](crate::operation::start_upload_job::StartUploadJobOutput).
    pub fn builder() -> crate::operation::start_upload_job::builders::StartUploadJobOutputBuilder {
        crate::operation::start_upload_job::builders::StartUploadJobOutputBuilder::default()
    }
}

/// A builder for [`StartUploadJobOutput`](crate::operation::start_upload_job::StartUploadJobOutput).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct StartUploadJobOutputBuilder {
    _request_id: Option<String>,
}
impl StartUploadJobOutputBuilder {
    pub(crate) fn _request_id(mut self, request_id: impl Into<String>) -> Self {
        self._request_id = Some(request_id.into());
        self
    }

    pub(crate) fn _set_request_id(&mut self, request_id: Option<String>) -> &mut Self {
        self._request_id = request_id;
        self
    }
    /// Consumes the builder and constructs a [`StartUploadJobOutput`](crate::operation::start_upload_job::StartUploadJobOutput).
    pub fn build(self) -> crate::operation::start_upload_job::StartUploadJobOutput {
        crate::operation::start_upload_job::StartUploadJobOutput {
            _request_id: self._request_id,
        }
    }
}
