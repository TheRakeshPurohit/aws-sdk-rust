// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// All possible error types for this service.
#[non_exhaustive]
#[derive(::std::fmt::Debug)]
pub enum Error {
    /// <p></p>
    AccessDeniedException(crate::types::error::AccessDeniedException),
    /// <p></p>
    BadGatewayException(crate::types::error::BadGatewayException),
    /// <p></p>
    ConflictException(crate::types::error::ConflictException),
    /// <p></p>
    DependencyFailedException(crate::types::error::DependencyFailedException),
    /// <p></p>
    InternalServerException(crate::types::error::InternalServerException),
    /// <p></p>
    ResourceNotFoundException(crate::types::error::ResourceNotFoundException),
    /// <p></p>
    ThrottlingException(crate::types::error::ThrottlingException),
    /// <p></p>
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
            Error::BadGatewayException(inner) => inner.fmt(f),
            Error::ConflictException(inner) => inner.fmt(f),
            Error::DependencyFailedException(inner) => inner.fmt(f),
            Error::InternalServerException(inner) => inner.fmt(f),
            Error::ResourceNotFoundException(inner) => inner.fmt(f),
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
            Self::BadGatewayException(inner) => inner.meta(),
            Self::ConflictException(inner) => inner.meta(),
            Self::DependencyFailedException(inner) => inner.meta(),
            Self::InternalServerException(inner) => inner.meta(),
            Self::ResourceNotFoundException(inner) => inner.meta(),
            Self::ThrottlingException(inner) => inner.meta(),
            Self::ValidationException(inner) => inner.meta(),
            Self::Unhandled(inner) => &inner.meta,
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_session::DeleteSessionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::delete_session::DeleteSessionError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::delete_session::DeleteSessionError> for Error {
    fn from(err: crate::operation::delete_session::DeleteSessionError) -> Self {
        match err {
            crate::operation::delete_session::DeleteSessionError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::delete_session::DeleteSessionError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::delete_session::DeleteSessionError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::delete_session::DeleteSessionError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::delete_session::DeleteSessionError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::delete_session::DeleteSessionError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::delete_session::DeleteSessionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_session::GetSessionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::get_session::GetSessionError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::get_session::GetSessionError> for Error {
    fn from(err: crate::operation::get_session::GetSessionError) -> Self {
        match err {
            crate::operation::get_session::GetSessionError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::get_session::GetSessionError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::get_session::GetSessionError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::get_session::GetSessionError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::get_session::GetSessionError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::get_session::GetSessionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::put_session::PutSessionError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::put_session::PutSessionError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::put_session::PutSessionError> for Error {
    fn from(err: crate::operation::put_session::PutSessionError) -> Self {
        match err {
            crate::operation::put_session::PutSessionError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::put_session::PutSessionError::BadGatewayException(inner) => Error::BadGatewayException(inner),
            crate::operation::put_session::PutSessionError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::put_session::PutSessionError::DependencyFailedException(inner) => Error::DependencyFailedException(inner),
            crate::operation::put_session::PutSessionError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::put_session::PutSessionError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::put_session::PutSessionError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::put_session::PutSessionError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::put_session::PutSessionError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::recognize_text::RecognizeTextError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::recognize_text::RecognizeTextError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::recognize_text::RecognizeTextError> for Error {
    fn from(err: crate::operation::recognize_text::RecognizeTextError) -> Self {
        match err {
            crate::operation::recognize_text::RecognizeTextError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::recognize_text::RecognizeTextError::BadGatewayException(inner) => Error::BadGatewayException(inner),
            crate::operation::recognize_text::RecognizeTextError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::recognize_text::RecognizeTextError::DependencyFailedException(inner) => Error::DependencyFailedException(inner),
            crate::operation::recognize_text::RecognizeTextError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::recognize_text::RecognizeTextError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::recognize_text::RecognizeTextError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::recognize_text::RecognizeTextError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::recognize_text::RecognizeTextError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::recognize_utterance::RecognizeUtteranceError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::recognize_utterance::RecognizeUtteranceError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::recognize_utterance::RecognizeUtteranceError> for Error {
    fn from(err: crate::operation::recognize_utterance::RecognizeUtteranceError) -> Self {
        match err {
            crate::operation::recognize_utterance::RecognizeUtteranceError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::recognize_utterance::RecognizeUtteranceError::BadGatewayException(inner) => Error::BadGatewayException(inner),
            crate::operation::recognize_utterance::RecognizeUtteranceError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::recognize_utterance::RecognizeUtteranceError::DependencyFailedException(inner) => {
                Error::DependencyFailedException(inner)
            }
            crate::operation::recognize_utterance::RecognizeUtteranceError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::recognize_utterance::RecognizeUtteranceError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::operation::recognize_utterance::RecognizeUtteranceError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::recognize_utterance::RecognizeUtteranceError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::recognize_utterance::RecognizeUtteranceError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::operation::start_conversation::StartConversationError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::operation::start_conversation::StartConversationError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::operation::start_conversation::StartConversationError> for Error {
    fn from(err: crate::operation::start_conversation::StartConversationError) -> Self {
        match err {
            crate::operation::start_conversation::StartConversationError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::operation::start_conversation::StartConversationError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::operation::start_conversation::StartConversationError::ResourceNotFoundException(inner) => Error::ResourceNotFoundException(inner),
            crate::operation::start_conversation::StartConversationError::ValidationException(inner) => Error::ValidationException(inner),
            crate::operation::start_conversation::StartConversationError::ConflictException(inner) => Error::ConflictException(inner),
            crate::operation::start_conversation::StartConversationError::BadGatewayException(inner) => Error::BadGatewayException(inner),
            crate::operation::start_conversation::StartConversationError::DependencyFailedException(inner) => Error::DependencyFailedException(inner),
            crate::operation::start_conversation::StartConversationError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::operation::start_conversation::StartConversationError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::types::error::StartConversationRequestEventStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::types::error::StartConversationRequestEventStreamError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::types::error::StartConversationRequestEventStreamError> for Error {
    fn from(err: crate::types::error::StartConversationRequestEventStreamError) -> Self {
        match err {
            crate::types::error::StartConversationRequestEventStreamError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl<R> From<::aws_smithy_runtime_api::client::result::SdkError<crate::types::error::StartConversationResponseEventStreamError, R>> for Error
where
    R: Send + Sync + std::fmt::Debug + 'static,
{
    fn from(err: ::aws_smithy_runtime_api::client::result::SdkError<crate::types::error::StartConversationResponseEventStreamError, R>) -> Self {
        match err {
            ::aws_smithy_runtime_api::client::result::SdkError::ServiceError(context) => Self::from(context.into_err()),
            _ => Error::Unhandled(crate::error::sealed_unhandled::Unhandled {
                meta: ::aws_smithy_types::error::metadata::ProvideErrorMetadata::meta(&err).clone(),
                source: err.into(),
            }),
        }
    }
}
impl From<crate::types::error::StartConversationResponseEventStreamError> for Error {
    fn from(err: crate::types::error::StartConversationResponseEventStreamError) -> Self {
        match err {
            crate::types::error::StartConversationResponseEventStreamError::AccessDeniedException(inner) => Error::AccessDeniedException(inner),
            crate::types::error::StartConversationResponseEventStreamError::ResourceNotFoundException(inner) => {
                Error::ResourceNotFoundException(inner)
            }
            crate::types::error::StartConversationResponseEventStreamError::ValidationException(inner) => Error::ValidationException(inner),
            crate::types::error::StartConversationResponseEventStreamError::ThrottlingException(inner) => Error::ThrottlingException(inner),
            crate::types::error::StartConversationResponseEventStreamError::InternalServerException(inner) => Error::InternalServerException(inner),
            crate::types::error::StartConversationResponseEventStreamError::ConflictException(inner) => Error::ConflictException(inner),
            crate::types::error::StartConversationResponseEventStreamError::DependencyFailedException(inner) => {
                Error::DependencyFailedException(inner)
            }
            crate::types::error::StartConversationResponseEventStreamError::BadGatewayException(inner) => Error::BadGatewayException(inner),
            crate::types::error::StartConversationResponseEventStreamError::Unhandled(inner) => Error::Unhandled(inner),
        }
    }
}
impl ::std::error::Error for Error {
    fn source(&self) -> std::option::Option<&(dyn ::std::error::Error + 'static)> {
        match self {
            Error::AccessDeniedException(inner) => inner.source(),
            Error::BadGatewayException(inner) => inner.source(),
            Error::ConflictException(inner) => inner.source(),
            Error::DependencyFailedException(inner) => inner.source(),
            Error::InternalServerException(inner) => inner.source(),
            Error::ResourceNotFoundException(inner) => inner.source(),
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
            Self::BadGatewayException(e) => e.request_id(),
            Self::ConflictException(e) => e.request_id(),
            Self::DependencyFailedException(e) => e.request_id(),
            Self::InternalServerException(e) => e.request_id(),
            Self::ResourceNotFoundException(e) => e.request_id(),
            Self::ThrottlingException(e) => e.request_id(),
            Self::ValidationException(e) => e.request_id(),
            Self::Unhandled(e) => e.meta.request_id(),
        }
    }
}
