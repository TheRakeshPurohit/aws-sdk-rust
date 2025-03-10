// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn de_get_folder_http_error(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_folder::GetFolderOutput, crate::operation::get_folder::GetFolderError> {
    #[allow(unused_mut)]
    let mut generic_builder = crate::protocol_serde::parse_http_error_metadata(_response_status, _response_headers, _response_body)
        .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
    generic_builder = ::aws_types::request_id::apply_request_id(generic_builder, _response_headers);
    let generic = generic_builder.build();
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::operation::get_folder::GetFolderError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "CommitDoesNotExistException" => crate::operation::get_folder::GetFolderError::CommitDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::CommitDoesNotExistExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_commit_does_not_exist_exception::de_commit_does_not_exist_exception_json_err(_response_body, output)
                        .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionIntegrityChecksFailedException" => crate::operation::get_folder::GetFolderError::EncryptionIntegrityChecksFailedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionIntegrityChecksFailedExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_integrity_checks_failed_exception::de_encryption_integrity_checks_failed_exception_json_err(_response_body, output).map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionKeyAccessDeniedException" => crate::operation::get_folder::GetFolderError::EncryptionKeyAccessDeniedException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionKeyAccessDeniedExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_key_access_denied_exception::de_encryption_key_access_denied_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionKeyDisabledException" => crate::operation::get_folder::GetFolderError::EncryptionKeyDisabledException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionKeyDisabledExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_key_disabled_exception::de_encryption_key_disabled_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionKeyNotFoundException" => crate::operation::get_folder::GetFolderError::EncryptionKeyNotFoundException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionKeyNotFoundExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_key_not_found_exception::de_encryption_key_not_found_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "EncryptionKeyUnavailableException" => crate::operation::get_folder::GetFolderError::EncryptionKeyUnavailableException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::EncryptionKeyUnavailableExceptionBuilder::default();
                output = crate::protocol_serde::shape_encryption_key_unavailable_exception::de_encryption_key_unavailable_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "FolderDoesNotExistException" => crate::operation::get_folder::GetFolderError::FolderDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::FolderDoesNotExistExceptionBuilder::default();
                output =
                    crate::protocol_serde::shape_folder_does_not_exist_exception::de_folder_does_not_exist_exception_json_err(_response_body, output)
                        .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidCommitException" => crate::operation::get_folder::GetFolderError::InvalidCommitException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidCommitExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_commit_exception::de_invalid_commit_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidPathException" => crate::operation::get_folder::GetFolderError::InvalidPathException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidPathExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_path_exception::de_invalid_path_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "InvalidRepositoryNameException" => crate::operation::get_folder::GetFolderError::InvalidRepositoryNameException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::InvalidRepositoryNameExceptionBuilder::default();
                output = crate::protocol_serde::shape_invalid_repository_name_exception::de_invalid_repository_name_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "PathRequiredException" => crate::operation::get_folder::GetFolderError::PathRequiredException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::PathRequiredExceptionBuilder::default();
                output = crate::protocol_serde::shape_path_required_exception::de_path_required_exception_json_err(_response_body, output)
                    .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RepositoryDoesNotExistException" => crate::operation::get_folder::GetFolderError::RepositoryDoesNotExistException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RepositoryDoesNotExistExceptionBuilder::default();
                output = crate::protocol_serde::shape_repository_does_not_exist_exception::de_repository_does_not_exist_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        "RepositoryNameRequiredException" => crate::operation::get_folder::GetFolderError::RepositoryNameRequiredException({
            #[allow(unused_mut)]
            let mut tmp = {
                #[allow(unused_mut)]
                let mut output = crate::types::error::builders::RepositoryNameRequiredExceptionBuilder::default();
                output = crate::protocol_serde::shape_repository_name_required_exception::de_repository_name_required_exception_json_err(
                    _response_body,
                    output,
                )
                .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
                let output = output.meta(generic);
                output.build()
            };
            if tmp.message.is_none() {
                tmp.message = _error_message;
            }
            tmp
        }),
        _ => crate::operation::get_folder::GetFolderError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn de_get_folder_http_response(
    _response_status: u16,
    _response_headers: &::aws_smithy_runtime_api::http::Headers,
    _response_body: &[u8],
) -> std::result::Result<crate::operation::get_folder::GetFolderOutput, crate::operation::get_folder::GetFolderError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::operation::get_folder::builders::GetFolderOutputBuilder::default();
        output = crate::protocol_serde::shape_get_folder::de_get_folder(_response_body, output)
            .map_err(crate::operation::get_folder::GetFolderError::unhandled)?;
        output._set_request_id(::aws_types::request_id::RequestId::request_id(_response_headers).map(str::to_string));
        crate::serde_util::get_folder_output_output_correct_errors(output)
            .build()
            .map_err(crate::operation::get_folder::GetFolderError::unhandled)?
    })
}

pub fn ser_get_folder_input(
    input: &crate::operation::get_folder::GetFolderInput,
) -> ::std::result::Result<::aws_smithy_types::body::SdkBody, ::aws_smithy_types::error::operation::SerializationError> {
    let mut out = String::new();
    let mut object = ::aws_smithy_json::serialize::JsonObjectWriter::new(&mut out);
    crate::protocol_serde::shape_get_folder_input::ser_get_folder_input_input(&mut object, input)?;
    object.finish();
    Ok(::aws_smithy_types::body::SdkBody::from(out))
}

pub(crate) fn de_get_folder(
    value: &[u8],
    mut builder: crate::operation::get_folder::builders::GetFolderOutputBuilder,
) -> ::std::result::Result<crate::operation::get_folder::builders::GetFolderOutputBuilder, ::aws_smithy_json::deserialize::error::DeserializeError> {
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
                "folderPath" => {
                    builder = builder.set_folder_path(
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
                "subFolders" => {
                    builder = builder.set_sub_folders(crate::protocol_serde::shape_folder_list::de_folder_list(tokens)?);
                }
                "files" => {
                    builder = builder.set_files(crate::protocol_serde::shape_file_list::de_file_list(tokens)?);
                }
                "symbolicLinks" => {
                    builder = builder.set_symbolic_links(crate::protocol_serde::shape_symbolic_link_list::de_symbolic_link_list(tokens)?);
                }
                "subModules" => {
                    builder = builder.set_sub_modules(crate::protocol_serde::shape_sub_module_list::de_sub_module_list(tokens)?);
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
