// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_file_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::delete_file::DeleteFileOutput, crate::operation::delete_file::DeleteFileError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::delete_file::DeleteFileError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "BranchDoesNotExistException" => crate::operation::delete_file::DeleteFileError::BranchDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BranchDoesNotExistExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_branch_does_not_exist_exception::de_branch_does_not_exist_exception_json_err(_response_body, output)
                        .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "BranchNameIsTagNameException" => crate::operation::delete_file::DeleteFileError::BranchNameIsTagNameException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BranchNameIsTagNameExceptionBuilder::default();
                output = crate::protocol_serde::shape_branch_name_is_tag_name_exception::de_branch_name_is_tag_name_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "BranchNameRequiredException" => crate::operation::delete_file::DeleteFileError::BranchNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::BranchNameRequiredExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_branch_name_required_exception::de_branch_name_required_exception_json_err(_response_body, output)
                        .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "CommitMessageLengthExceededException" => crate::operation::delete_file::DeleteFileError::CommitMessageLengthExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CommitMessageLengthExceededExceptionBuilder::default();
                output = crate::protocol_serde::shape_commit_message_length_exceeded_exception::de_commit_message_length_exceeded_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionIntegrityChecksFailedException" => crate::operation::delete_file::DeleteFileError::EncryptionIntegrityChecksFailedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionIntegrityChecksFailedExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_integrity_checks_failed_exception::de_encryption_integrity_checks_failed_exception_json_err(_response_body, output).map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionKeyAccessDeniedException" => crate::operation::delete_file::DeleteFileError::EncryptionKeyAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionKeyAccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_key_access_denied_exception::de_encryption_key_access_denied_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionKeyDisabledException" => crate::operation::delete_file::DeleteFileError::EncryptionKeyDisabledException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionKeyDisabledExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_key_disabled_exception::de_encryption_key_disabled_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionKeyNotFoundException" => crate::operation::delete_file::DeleteFileError::EncryptionKeyNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionKeyNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_key_not_found_exception::de_encryption_key_not_found_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionKeyUnavailableException" => crate::operation::delete_file::DeleteFileError::EncryptionKeyUnavailableException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionKeyUnavailableExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_key_unavailable_exception::de_encryption_key_unavailable_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "FileDoesNotExistException" => crate::operation::delete_file::DeleteFileError::FileDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::FileDoesNotExistExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_file_does_not_exist_exception::de_file_does_not_exist_exception_json_err(_response_body, output)
                        .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidBranchNameException" => crate::operation::delete_file::DeleteFileError::InvalidBranchNameException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidBranchNameExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_invalid_branch_name_exception::de_invalid_branch_name_exception_json_err(_response_body, output)
                        .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidEmailException" => crate::operation::delete_file::DeleteFileError::InvalidEmailException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidEmailExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_email_exception::de_invalid_email_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidParentCommitIdException" => crate::operation::delete_file::DeleteFileError::InvalidParentCommitIdException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidParentCommitIdExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_parent_commit_id_exception::de_invalid_parent_commit_id_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidPathException" => crate::operation::delete_file::DeleteFileError::InvalidPathException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidPathExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_path_exception::de_invalid_path_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidRepositoryNameException" => crate::operation::delete_file::DeleteFileError::InvalidRepositoryNameException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRepositoryNameExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_repository_name_exception::de_invalid_repository_name_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "NameLengthExceededException" => crate::operation::delete_file::DeleteFileError::NameLengthExceededException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::NameLengthExceededExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_name_length_exceeded_exception::de_name_length_exceeded_exception_json_err(_response_body, output)
                        .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ParentCommitDoesNotExistException" => crate::operation::delete_file::DeleteFileError::ParentCommitDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ParentCommitDoesNotExistExceptionBuilder::default();
                output = crate::protocol_serde::shape_parent_commit_does_not_exist_exception::de_parent_commit_does_not_exist_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ParentCommitIdOutdatedException" => crate::operation::delete_file::DeleteFileError::ParentCommitIdOutdatedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ParentCommitIdOutdatedExceptionBuilder::default();
                output = crate::protocol_serde::shape_parent_commit_id_outdated_exception::de_parent_commit_id_outdated_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "ParentCommitIdRequiredException" => crate::operation::delete_file::DeleteFileError::ParentCommitIdRequiredException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::ParentCommitIdRequiredExceptionBuilder::default();
                output = crate::protocol_serde::shape_parent_commit_id_required_exception::de_parent_commit_id_required_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "PathRequiredException" => crate::operation::delete_file::DeleteFileError::PathRequiredException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::PathRequiredExceptionBuilder::default();
                output = crate::protocol_serde::shape_path_required_exception::de_path_required_exception_json_err(_response_body, output)
                    .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RepositoryDoesNotExistException" => crate::operation::delete_file::DeleteFileError::RepositoryDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RepositoryDoesNotExistExceptionBuilder::default();
                output = crate::protocol_serde::shape_repository_does_not_exist_exception::de_repository_does_not_exist_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RepositoryNameRequiredException" => crate::operation::delete_file::DeleteFileError::RepositoryNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RepositoryNameRequiredExceptionBuilder::default();
                output = crate::protocol_serde::shape_repository_name_required_exception::de_repository_name_required_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::delete_file::DeleteFileError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_delete_file_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::delete_file::DeleteFileOutput, crate::operation::delete_file::DeleteFileError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::delete_file::builders::DeleteFileOutputBuilder::default();
        output = crate::protocol_serde::shape_delete_file::de_delete_file(_response_body, output)
            .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::delete_file_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::delete_file::DeleteFileError::unhandled)?
    })
}

pub fn ser_delete_file_input(
    input: &crate::operation::delete_file::DeleteFileInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_delete_file_input::ser_delete_file_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_delete_file(
    value: &[u8],
    mut builder: crate::operation::delete_file::builders::DeleteFileOutputBuilder,
) -> ::std::result::Result<crate::operation::delete_file::builders::DeleteFileOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError>
{
    let mut tokens_owned = ::aws_smithy_json::deserialize::json_token_iter(crate::protocol_serde::or_empty_doc(value)).peekable();
    let tokens = &mut tokens_owned;
    ::aws_smithy_json::deserialize::token::expect_start_object(tokens.next())?;
    loop {
        match tokens.next().transpose()? {
            Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
            Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                "commitId" => {
                    builder = builder.set_commit_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "blobId" => {
                    builder = builder.set_blob_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "treeId" => {
                    builder = builder.set_tree_id(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                "filePath" => {
                    builder = builder.set_file_path(
                        ::aws_smithy_json::deserialize::token::expect_string_or_null(tokens.next())?
                            .map(|s| s.to_unescaped().map(|u| u.into_owned()))
                            .transpose()?,
                    );
                }
                _ => ::aws_smithy_json::deserialize::token::skip_value(tokens)?,
            },
            other => {
                return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(format!(
                    "expected object key or end object, found: {:?}",
                    other
                )))
            }
        }
    }
    if tokens.next().is_some() {
        return Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "found more JSON tokens after completing parsing",
        ));
    }
    Ok(builder)
}
