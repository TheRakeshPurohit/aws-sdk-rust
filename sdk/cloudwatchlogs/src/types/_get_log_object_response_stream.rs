// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>A stream of structured log data returned by the GetLogObject operation. This stream contains log events with their associated metadata and extracted fields.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub enum GetLogObjectResponseStream {
    /// <p>A structure containing the extracted fields from a log event. These fields are extracted based on the log format and can be used for structured querying and analysis.</p>
    Fields(crate::types::FieldsData),
    /// The `Unknown` variant represents cases where new union variant was received. Consider upgrading the SDK to the latest available version.
    /// An unknown enum variant
    ///
    /// _Note: If you encounter this error, consider upgrading your SDK to the latest version._
    /// The `Unknown` variant represents cases where the server sent a value that wasn't recognized
    /// by the client. This can happen when the server adds new functionality, but the client has not been updated.
    /// To investigate this, consider turning on debug logging to print the raw HTTP response.
    #[non_exhaustive]
    Unknown,
}
impl GetLogObjectResponseStream {
    #[allow(irrefutable_let_patterns)]
    /// Tries to convert the enum instance into [`Fields`](crate::types::GetLogObjectResponseStream::Fields), extracting the inner [`FieldsData`](crate::types::FieldsData).
    /// Returns `Err(&Self)` if it can't be converted.
    pub fn as_fields(&self) -> ::std::result::Result<&crate::types::FieldsData, &Self> {
        if let GetLogObjectResponseStream::Fields(val) = &self {
            ::std::result::Result::Ok(val)
        } else {
            ::std::result::Result::Err(self)
        }
    }
    /// Returns true if this is a [`Fields`](crate::types::GetLogObjectResponseStream::Fields).
    pub fn is_fields(&self) -> bool {
        self.as_fields().is_ok()
    }
    /// Returns true if the enum instance is the `Unknown` variant.
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }
}
