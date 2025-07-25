// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`DeleteVolume`](crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`volume_id(impl Into<String>)`](crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder::volume_id) / [`set_volume_id(Option<String>)`](crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder::set_volume_id):<br>required: **true**<br><p>Identifier of the volume to delete.</p><br>
    /// - On success, responds with [`DeleteVolumeOutput`](crate::operation::delete_volume::DeleteVolumeOutput)
    /// - On failure, responds with [`SdkError<DeleteVolumeError>`](crate::operation::delete_volume::DeleteVolumeError)
    pub fn delete_volume(&self) -> crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder {
        crate::operation::delete_volume::builders::DeleteVolumeFluentBuilder::new(self.handle.clone())
    }
}
