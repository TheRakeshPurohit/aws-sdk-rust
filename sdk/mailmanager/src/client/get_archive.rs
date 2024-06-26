// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`GetArchive`](crate::operation::get_archive::builders::GetArchiveFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`archive_id(impl Into<String>)`](crate::operation::get_archive::builders::GetArchiveFluentBuilder::archive_id) / [`set_archive_id(Option<String>)`](crate::operation::get_archive::builders::GetArchiveFluentBuilder::set_archive_id):<br>required: **true**<br><p>The identifier of the archive to retrieve.</p><br>
    /// - On success, responds with [`GetArchiveOutput`](crate::operation::get_archive::GetArchiveOutput) with field(s):
    ///   - [`archive_id(String)`](crate::operation::get_archive::GetArchiveOutput::archive_id): <p>The unique identifier of the archive.</p>
    ///   - [`archive_name(String)`](crate::operation::get_archive::GetArchiveOutput::archive_name): <p>The unique name assigned to the archive.</p>
    ///   - [`archive_arn(String)`](crate::operation::get_archive::GetArchiveOutput::archive_arn): <p>The Amazon Resource Name (ARN) of the archive.</p>
    ///   - [`archive_state(ArchiveState)`](crate::operation::get_archive::GetArchiveOutput::archive_state): <p>The current state of the archive:</p> <ul>  <li>   <p><code>ACTIVE</code> – The archive is ready and available for use.</p></li>  <li>   <p><code>PENDING_DELETION</code> – The archive has been marked for deletion and will be permanently deleted in 30 days. No further modifications can be made in this state.</p></li> </ul>
    ///   - [`retention(Option<ArchiveRetention>)`](crate::operation::get_archive::GetArchiveOutput::retention): <p>The retention period for emails in this archive.</p>
    ///   - [`created_timestamp(Option<DateTime>)`](crate::operation::get_archive::GetArchiveOutput::created_timestamp): <p>The timestamp of when the archive was created.</p>
    ///   - [`last_updated_timestamp(Option<DateTime>)`](crate::operation::get_archive::GetArchiveOutput::last_updated_timestamp): <p>The timestamp of when the archive was modified.</p>
    ///   - [`kms_key_arn(Option<String>)`](crate::operation::get_archive::GetArchiveOutput::kms_key_arn): <p>The Amazon Resource Name (ARN) of the KMS key used to encrypt the archive.</p>
    /// - On failure, responds with [`SdkError<GetArchiveError>`](crate::operation::get_archive::GetArchiveError)
    pub fn get_archive(&self) -> crate::operation::get_archive::builders::GetArchiveFluentBuilder {
        crate::operation::get_archive::builders::GetArchiveFluentBuilder::new(self.handle.clone())
    }
}
