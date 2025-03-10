// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub(crate) fn de_executor_configuration<'a, I>(
    tokens: &mut ::std::iter::Peekable<I>,
) -> ::std::result::Result<Option<crate::types::ExecutorConfiguration>, ::aws_smithy_json::deserialize::error::DeserializeError>
where
    I: Iterator<Item = Result<::aws_smithy_json::deserialize::Token<'a>, ::aws_smithy_json::deserialize::error::DeserializeError>>,
{
    match tokens.next().transpose()? {
        Some(::aws_smithy_json::deserialize::Token::ValueNull { .. }) => Ok(None),
        Some(::aws_smithy_json::deserialize::Token::StartObject { .. }) => {
            #[allow(unused_mut)]
            let mut builder = crate::types::builders::ExecutorConfigurationBuilder::default();
            loop {
                match tokens.next().transpose()? {
                    Some(::aws_smithy_json::deserialize::Token::EndObject { .. }) => break,
                    Some(::aws_smithy_json::deserialize::Token::ObjectKey { key, .. }) => match key.to_unescaped()?.as_ref() {
                        "lambdaExecutorConfiguration" => {
                            builder = builder.set_lambda_executor_configuration(
                                crate::protocol_serde::shape_lambda_executor_configuration::de_lambda_executor_configuration(tokens)?,
                            );
                        }
                        "jobWorkerExecutorConfiguration" => {
                            builder = builder.set_job_worker_executor_configuration(
                                crate::protocol_serde::shape_job_worker_executor_configuration::de_job_worker_executor_configuration(tokens)?,
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
            Ok(Some(builder.build()))
        }
        _ => Err(::aws_smithy_json::deserialize::error::DeserializeError::custom(
            "expected start object or null",
        )),
    }
}

pub fn ser_executor_configuration(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::ExecutorConfiguration,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.lambda_executor_configuration {
        #[allow(unused_mut)]
        let mut object_2 = object.key("lambdaExecutorConfiguration").start_object();
        crate::protocol_serde::shape_lambda_executor_configuration::ser_lambda_executor_configuration(&mut object_2, var_1)?;
        object_2.finish();
    }
    if let Some(var_3) = &input.job_worker_executor_configuration {
        #[allow(unused_mut)]
        let mut object_4 = object.key("jobWorkerExecutorConfiguration").start_object();
        crate::protocol_serde::shape_job_worker_executor_configuration::ser_job_worker_executor_configuration(&mut object_4, var_3)?;
        object_4.finish();
    }
    Ok(())
}
