// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DescribeBotReplica`](crate::operation::describe_bot_replica::builders::DescribeBotReplicaFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`bot_id(impl Into<String>)`](crate::operation::describe_bot_replica::builders::DescribeBotReplicaFluentBuilder::bot_id) / [`set_bot_id(Option<String>)`](crate::operation::describe_bot_replica::builders::DescribeBotReplicaFluentBuilder::set_bot_id):<br>required: **true**<br><p>The request for the unique bot ID of the replicated bot being monitored.</p><br>
    ///   - [`replica_region(impl Into<String>)`](crate::operation::describe_bot_replica::builders::DescribeBotReplicaFluentBuilder::replica_region) / [`set_replica_region(Option<String>)`](crate::operation::describe_bot_replica::builders::DescribeBotReplicaFluentBuilder::set_replica_region):<br>required: **true**<br><p>The request for the region of the replicated bot being monitored.</p><br>
    /// - On success, responds with [`DescribeBotReplicaOutput`](crate::operation::describe_bot_replica::DescribeBotReplicaOutput) with field(s):
    ///   - [`bot_id(Option<String>)`](crate::operation::describe_bot_replica::DescribeBotReplicaOutput::bot_id): <p>The unique bot ID of the replicated bot being monitored.</p>
    ///   - [`replica_region(Option<String>)`](crate::operation::describe_bot_replica::DescribeBotReplicaOutput::replica_region): <p>The region of the replicated bot being monitored.</p>
    ///   - [`source_region(Option<String>)`](crate::operation::describe_bot_replica::DescribeBotReplicaOutput::source_region): <p>The source region of the replicated bot being monitored.</p>
    ///   - [`creation_date_time(Option<DateTime>)`](crate::operation::describe_bot_replica::DescribeBotReplicaOutput::creation_date_time): <p>The creation date and time of the replicated bot being monitored.</p>
    ///   - [`bot_replica_status(Option<BotReplicaStatus>)`](crate::operation::describe_bot_replica::DescribeBotReplicaOutput::bot_replica_status): <p>The operational status of the replicated bot being monitored.</p>
    ///   - [`failure_reasons(Option<Vec::<String>>)`](crate::operation::describe_bot_replica::DescribeBotReplicaOutput::failure_reasons): <p>The failure reasons the bot being monitored failed to replicate.</p>
    /// - On failure, responds with [`SdkError<DescribeBotReplicaError>`](crate::operation::describe_bot_replica::DescribeBotReplicaError)
    pub fn describe_bot_replica(&self) -> crate::operation::describe_bot_replica::builders::DescribeBotReplicaFluentBuilder {
        crate::operation::describe_bot_replica::builders::DescribeBotReplicaFluentBuilder::new(self.handle.clone())
    }
}
