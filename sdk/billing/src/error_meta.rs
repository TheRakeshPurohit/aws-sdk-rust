// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p>You don't have sufficient access to perform this action.</p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p>The requested operation would cause a conflict with the current state of a service resource associated with the request. Resolve the conflict before retrying this request.</p>
    ConflictException(crate::types::error::ConflictException),
    /// <p>The request processing failed because of an unknown error, exception, or failure.</p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p>The specified ARN in the request doesn't exist.</p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p>You've reached the limit of resources you can create, or exceeded the size of an individual resource.</p>
    ServiceQuotaExceededException(crate::types::error::ServiceQuotaExceededException),
    /// <p>The request was denied due to request throttling.</p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p>The input fails to satisfy the constraints specified by an Amazon Web Services service.</p>
    ValidationException(crate::types::error::ValidationException),
    /// An unexpected error occurred (e.g., invalid JSON returned by the service or an unknown error code).
    #[deprecated(note = "Matching `Unhandled` directly is not forwards compatible. Instead, match using a \
    variable wildcard pattern and check `.code()`:
     \
    &nbsp;&nbsp;&nbsp;`err if err.code() == Some(\"SpecificExceptionCode\") => { /* handle the error */ }`
     \
    See [`ProvideErrorMetadata`](#impl-ProvideErrorMetadata-for-Error) for what information is available for the error.")]
    Unhandled(crate::error::sealed_unhandled::Unhandled),
}
impl ::std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::AccessDeniedException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
            Error::ServiceQuotaExceededException(inner) => inner.fmt(f),
            Error::ThrottlingException(inner) => inner.fmt(f),
            Error::ValidationException(inner) => inner.fmt(f),
            Error::Unhandled(_) => {
                if let ::std::option::Option::Some(code) = ::aws_smithy_types::error::metadata::ProvideErrorMetadata::code(self) {
                    write!(f, "unhandled error ({code})")
                } else {
                    f.write_str("unhandled error")
                }
            }
        }
    }
}
impl From<::aws_smithy_types::error::operation::BuildError> for Error {
    fn from(value: ::aws_smithy_types::error::operation::BuildError) -> Self {
        Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
            source: value.into(),
            meta: ::std::default::Default::default(),
        })
    }
}
impl ::aws_smithy_types::error::metadata::ProvideErrorMetadata for Error {
    fn meta(&self) -> &::aws_smithy_types::error::metadata::ErrorMetadata {
        match self {
            Self::AccessDeniedException(inner) => inner.meta(),
            Self::ConflictException(inner) => inner.meta(),
            Self::InternalServerException(inner) => inner.meta(),
            Self::ResourceNotFoundException(inner) => inner.meta(),
            Self::ServiceQuotaExceededException(inner) => inner.meta(),
            Self::ThrottlingException(inner) => inner.meta(),
            Self::ValidationException(inner) => inner.meta(),
            Self::Unhandled(inner) => &inner.meta,
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_billing_view::CreateBillingViewError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::create_billing_view::CreateBillingViewError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::create_billing_view::CreateBillingViewError> for Error {
    fn from(err: crate::operation::create_billing_view::CreateBillingViewError) -> Self {
        match err {
            crate::operation::create_billing_view::CreateBillingViewError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::create_billing_view::CreateBillingViewError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::create_billing_view::CreateBillingViewError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::create_billing_view::CreateBillingViewError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::create_billing_view::CreateBillingViewError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::create_billing_view::CreateBillingViewError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::create_billing_view::CreateBillingViewError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_billing_view::DeleteBillingViewError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_billing_view::DeleteBillingViewError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::delete_billing_view::DeleteBillingViewError> for Error {
    fn from(err: crate::operation::delete_billing_view::DeleteBillingViewError) -> Self {
        match err {
            crate::operation::delete_billing_view::DeleteBillingViewError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::delete_billing_view::DeleteBillingViewError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::delete_billing_view::DeleteBillingViewError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::delete_billing_view::DeleteBillingViewError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::delete_billing_view::DeleteBillingViewError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::delete_billing_view::DeleteBillingViewError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_billing_view::GetBillingViewError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_billing_view::GetBillingViewError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_billing_view::GetBillingViewError> for Error {
    fn from(err: crate::operation::get_billing_view::GetBillingViewError) -> Self {
        match err {
            crate::operation::get_billing_view::GetBillingViewError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::get_billing_view::GetBillingViewError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_billing_view::GetBillingViewError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_billing_view::GetBillingViewError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::get_billing_view::GetBillingViewError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_billing_view::GetBillingViewError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_resource_policy::GetResourcePolicyError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_resource_policy::GetResourcePolicyError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_resource_policy::GetResourcePolicyError> for Error {
    fn from(err: crate::operation::get_resource_policy::GetResourcePolicyError) -> Self {
        match err {
            crate::operation::get_resource_policy::GetResourcePolicyError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::get_resource_policy::GetResourcePolicyError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_resource_policy::GetResourcePolicyError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::get_resource_policy::GetResourcePolicyError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::get_resource_policy::GetResourcePolicyError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_resource_policy::GetResourcePolicyError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_billing_views::ListBillingViewsError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_billing_views::ListBillingViewsError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_billing_views::ListBillingViewsError> for Error {
    fn from(err: crate::operation::list_billing_views::ListBillingViewsError) -> Self {
        match err {
            crate::operation::list_billing_views::ListBillingViewsError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::list_billing_views::ListBillingViewsError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::list_billing_views::ListBillingViewsError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_billing_views::ListBillingViewsError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_billing_views::ListBillingViewsError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R>
    From<
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_source_views_for_billing_view::ListSourceViewsForBillingViewError,
            R,
        >,
    > for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(
        err: ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::list_source_views_for_billing_view::ListSourceViewsForBillingViewError,
            R,
        >,
    ) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_source_views_for_billing_view::ListSourceViewsForBillingViewError> for Error {
    fn from(err: crate::operation::list_source_views_for_billing_view::ListSourceViewsForBillingViewError) -> Self {
        match err {
            crate::operation::list_source_views_for_billing_view::ListSourceViewsForBillingViewError::AccessDeniedException(inner) => {
                Error::AccessDeniedException(inner)
            }
            crate::operation::list_source_views_for_billing_view::ListSourceViewsForBillingViewError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::list_source_views_for_billing_view::ListSourceViewsForBillingViewError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::list_source_views_for_billing_view::ListSourceViewsForBillingViewError::ThrottlingException(inner) => {
                Error::ThrottlingException(inner)
            }
            crate::operation::list_source_views_for_billing_view::ListSourceViewsForBillingViewError::ValidationException(inner) => {
                Error::ValidationException(inner)
            }
            crate::operation::list_source_views_for_billing_view::ListSourceViewsForBillingViewError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_tags_for_resource::ListTagsForResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::list_tags_for_resource::ListTagsForResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::list_tags_for_resource::ListTagsForResourceError> for Error {
    fn from(err: crate::operation::list_tags_for_resource::ListTagsForResourceError) -> Self {
        match err {
            crate::operation::list_tags_for_resource::ListTagsForResourceError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::InternalServerException(inner) => {
                Error::InternalServerException(inner)
            }
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::list_tags_for_resource::ListTagsForResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::tag_resource::TagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::tag_resource::TagResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::tag_resource::TagResourceError> for Error {
    fn from(err: crate::operation::tag_resource::TagResourceError) -> Self {
        match err {
            crate::operation::tag_resource::TagResourceError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::tag_resource::TagResourceError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::tag_resource::TagResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::tag_resource::TagResourceError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::tag_resource::TagResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::tag_resource::TagResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::untag_resource::UntagResourceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::untag_resource::UntagResourceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::untag_resource::UntagResourceError> for Error {
    fn from(err: crate::operation::untag_resource::UntagResourceError) -> Self {
        match err {
            crate::operation::untag_resource::UntagResourceError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::untag_resource::UntagResourceError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::untag_resource::UntagResourceError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::untag_resource::UntagResourceError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::untag_resource::UntagResourceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::untag_resource::UntagResourceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::update_billing_view::UpdateBillingViewError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::update_billing_view::UpdateBillingViewError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::update_billing_view::UpdateBillingViewError> for Error {
    fn from(err: crate::operation::update_billing_view::UpdateBillingViewError) -> Self {
        match err {
            crate::operation::update_billing_view::UpdateBillingViewError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::update_billing_view::UpdateBillingViewError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::update_billing_view::UpdateBillingViewError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::update_billing_view::UpdateBillingViewError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::update_billing_view::UpdateBillingViewError::ServiceQuotaExceededException(inner) => {
                Error::ServiceQuotaExceededException(inner)
            }
            crate::operation::update_billing_view::UpdateBillingViewError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::update_billing_view::UpdateBillingViewError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::update_billing_view::UpdateBillingViewError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::AccessDeniedException(inner) => inner.source(),
            Error::ConflictException(inner) => inner.source(),
            Error::InternalServerException(inner) => inner.source(),
            Error::ResourceNotFoundException(inner) => inner.source(),
            Error::ServiceQuotaExceededException(inner) => inner.source(),
            Error::ThrottlingException(inner) => inner.source(),
            Error::ValidationException(inner) => inner.source(),
            Error::Unhandled(inner) => ::std::option::Option::Some(&*inner.source),
        }
    }
}
impl ::aws_types::request_id::RequestId for Error {
    fn request_id(&self) -> Option<&str> {
        match self {
            Self::AccessDeniedException(e) => e.request_id(),
            Self::ConflictException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ServiceQuotaExceededException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.meta.request_id(),
        }
    }
}
