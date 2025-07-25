// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <note>
/// <p>Amazon S3 Vectors is in preview release for Amazon S3 and is subject to change.</p>
/// </note>
/// <p>The attributes of a vector index.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct Index {
    /// <p>The name of the vector bucket that contains the vector index.</p>
    pub vector_bucket_name: ::std::string::String,
    /// <p>The name of the vector index.</p>
    pub index_name: ::std::string::String,
    /// <p>The Amazon Resource Name (ARN) of the vector index.</p>
    pub index_arn: ::std::string::String,
    /// <p>Date and time when the vector index was created.</p>
    pub creation_time: ::aws_smithy_types::DateTime,
    /// <p>The data type of the vectors inserted into the vector index.</p>
    pub data_type: crate::types::DataType,
    /// <p>The number of values in the vectors that are inserted into the vector index.</p>
    pub dimension: i32,
    /// <p>The distance metric to be used for similarity search.</p>
    pub distance_metric: crate::types::DistanceMetric,
    /// <p>The metadata configuration for the vector index.</p>
    pub metadata_configuration: ::std::option::Option<crate::types::MetadataConfiguration>,
}
impl Index {
    /// <p>The name of the vector bucket that contains the vector index.</p>
    pub fn vector_bucket_name(&self) -> &str {
        use std::ops::Deref;
        self.vector_bucket_name.deref()
    }
    /// <p>The name of the vector index.</p>
    pub fn index_name(&self) -> &str {
        use std::ops::Deref;
        self.index_name.deref()
    }
    /// <p>The Amazon Resource Name (ARN) of the vector index.</p>
    pub fn index_arn(&self) -> &str {
        use std::ops::Deref;
        self.index_arn.deref()
    }
    /// <p>Date and time when the vector index was created.</p>
    pub fn creation_time(&self) -> &::aws_smithy_types::DateTime {
        &self.creation_time
    }
    /// <p>The data type of the vectors inserted into the vector index.</p>
    pub fn data_type(&self) -> &crate::types::DataType {
        &self.data_type
    }
    /// <p>The number of values in the vectors that are inserted into the vector index.</p>
    pub fn dimension(&self) -> i32 {
        self.dimension
    }
    /// <p>The distance metric to be used for similarity search.</p>
    pub fn distance_metric(&self) -> &crate::types::DistanceMetric {
        &self.distance_metric
    }
    /// <p>The metadata configuration for the vector index.</p>
    pub fn metadata_configuration(&self) -> ::std::option::Option<&crate::types::MetadataConfiguration> {
        self.metadata_configuration.as_ref()
    }
}
impl Index {
    /// Creates a new builder-style object to manufacture [`Index`](crate::types::Index).
    pub fn builder() -> crate::types::builders::IndexBuilder {
        crate::types::builders::IndexBuilder::default()
    }
}

/// A builder for [`Index`](crate::types::Index).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct IndexBuilder {
    pub(crate) vector_bucket_name: ::std::option::Option<::std::string::String>,
    pub(crate) index_name: ::std::option::Option<::std::string::String>,
    pub(crate) index_arn: ::std::option::Option<::std::string::String>,
    pub(crate) creation_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) data_type: ::std::option::Option<crate::types::DataType>,
    pub(crate) dimension: ::std::option::Option<i32>,
    pub(crate) distance_metric: ::std::option::Option<crate::types::DistanceMetric>,
    pub(crate) metadata_configuration: ::std::option::Option<crate::types::MetadataConfiguration>,
}
impl IndexBuilder {
    /// <p>The name of the vector bucket that contains the vector index.</p>
    /// This field is required.
    pub fn vector_bucket_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.vector_bucket_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the vector bucket that contains the vector index.</p>
    pub fn set_vector_bucket_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.vector_bucket_name = input;
        self
    }
    /// <p>The name of the vector bucket that contains the vector index.</p>
    pub fn get_vector_bucket_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.vector_bucket_name
    }
    /// <p>The name of the vector index.</p>
    /// This field is required.
    pub fn index_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name of the vector index.</p>
    pub fn set_index_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_name = input;
        self
    }
    /// <p>The name of the vector index.</p>
    pub fn get_index_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.index_name
    }
    /// <p>The Amazon Resource Name (ARN) of the vector index.</p>
    /// This field is required.
    pub fn index_arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.index_arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the vector index.</p>
    pub fn set_index_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.index_arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) of the vector index.</p>
    pub fn get_index_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.index_arn
    }
    /// <p>Date and time when the vector index was created.</p>
    /// This field is required.
    pub fn creation_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.creation_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>Date and time when the vector index was created.</p>
    pub fn set_creation_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.creation_time = input;
        self
    }
    /// <p>Date and time when the vector index was created.</p>
    pub fn get_creation_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.creation_time
    }
    /// <p>The data type of the vectors inserted into the vector index.</p>
    /// This field is required.
    pub fn data_type(mut self, input: crate::types::DataType) -> Self {
        self.data_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The data type of the vectors inserted into the vector index.</p>
    pub fn set_data_type(mut self, input: ::std::option::Option<crate::types::DataType>) -> Self {
        self.data_type = input;
        self
    }
    /// <p>The data type of the vectors inserted into the vector index.</p>
    pub fn get_data_type(&self) -> &::std::option::Option<crate::types::DataType> {
        &self.data_type
    }
    /// <p>The number of values in the vectors that are inserted into the vector index.</p>
    /// This field is required.
    pub fn dimension(mut self, input: i32) -> Self {
        self.dimension = ::std::option::Option::Some(input);
        self
    }
    /// <p>The number of values in the vectors that are inserted into the vector index.</p>
    pub fn set_dimension(mut self, input: ::std::option::Option<i32>) -> Self {
        self.dimension = input;
        self
    }
    /// <p>The number of values in the vectors that are inserted into the vector index.</p>
    pub fn get_dimension(&self) -> &::std::option::Option<i32> {
        &self.dimension
    }
    /// <p>The distance metric to be used for similarity search.</p>
    /// This field is required.
    pub fn distance_metric(mut self, input: crate::types::DistanceMetric) -> Self {
        self.distance_metric = ::std::option::Option::Some(input);
        self
    }
    /// <p>The distance metric to be used for similarity search.</p>
    pub fn set_distance_metric(mut self, input: ::std::option::Option<crate::types::DistanceMetric>) -> Self {
        self.distance_metric = input;
        self
    }
    /// <p>The distance metric to be used for similarity search.</p>
    pub fn get_distance_metric(&self) -> &::std::option::Option<crate::types::DistanceMetric> {
        &self.distance_metric
    }
    /// <p>The metadata configuration for the vector index.</p>
    pub fn metadata_configuration(mut self, input: crate::types::MetadataConfiguration) -> Self {
        self.metadata_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The metadata configuration for the vector index.</p>
    pub fn set_metadata_configuration(mut self, input: ::std::option::Option<crate::types::MetadataConfiguration>) -> Self {
        self.metadata_configuration = input;
        self
    }
    /// <p>The metadata configuration for the vector index.</p>
    pub fn get_metadata_configuration(&self) -> &::std::option::Option<crate::types::MetadataConfiguration> {
        &self.metadata_configuration
    }
    /// Consumes the builder and constructs a [`Index`](crate::types::Index).
    /// This method will fail if any of the following fields are not set:
    /// - [`vector_bucket_name`](crate::types::builders::IndexBuilder::vector_bucket_name)
    /// - [`index_name`](crate::types::builders::IndexBuilder::index_name)
    /// - [`index_arn`](crate::types::builders::IndexBuilder::index_arn)
    /// - [`creation_time`](crate::types::builders::IndexBuilder::creation_time)
    /// - [`data_type`](crate::types::builders::IndexBuilder::data_type)
    /// - [`dimension`](crate::types::builders::IndexBuilder::dimension)
    /// - [`distance_metric`](crate::types::builders::IndexBuilder::distance_metric)
    pub fn build(self) -> ::std::result::Result<crate::types::Index, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::Index {
            vector_bucket_name: self.vector_bucket_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "vector_bucket_name",
                    "vector_bucket_name was not specified but it is required when building Index",
                )
            })?,
            index_name: self.index_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "index_name",
                    "index_name was not specified but it is required when building Index",
                )
            })?,
            index_arn: self.index_arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "index_arn",
                    "index_arn was not specified but it is required when building Index",
                )
            })?,
            creation_time: self.creation_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "creation_time",
                    "creation_time was not specified but it is required when building Index",
                )
            })?,
            data_type: self.data_type.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "data_type",
                    "data_type was not specified but it is required when building Index",
                )
            })?,
            dimension: self.dimension.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "dimension",
                    "dimension was not specified but it is required when building Index",
                )
            })?,
            distance_metric: self.distance_metric.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "distance_metric",
                    "distance_metric was not specified but it is required when building Index",
                )
            })?,
            metadata_configuration: self.metadata_configuration,
        })
    }
}
