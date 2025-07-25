// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The configuration of the origin endpoint.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct OriginEndpointListConfiguration {
    /// <p>The Amazon Resource Name (ARN) associated with the resource.</p>
    pub arn: ::std::string::String,
    /// <p>The name that describes the channel group. The name is the primary identifier for the channel group, and must be unique for your account in the AWS Region.</p>
    pub channel_group_name: ::std::string::String,
    /// <p>The name that describes the channel. The name is the primary identifier for the channel, and must be unique for your account in the AWS Region and channel group.</p>
    pub channel_name: ::std::string::String,
    /// <p>The name that describes the origin endpoint. The name is the primary identifier for the origin endpoint, and and must be unique for your account in the AWS Region and channel.</p>
    pub origin_endpoint_name: ::std::string::String,
    /// <p>The type of container attached to this origin endpoint. A container type is a file format that encapsulates one or more media streams, such as audio and video, into a single file.</p>
    pub container_type: crate::types::ContainerType,
    /// <p>Any descriptive information that you want to add to the origin endpoint for future identification purposes.</p>
    pub description: ::std::option::Option<::std::string::String>,
    /// <p>The date and time the origin endpoint was created.</p>
    pub created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>The date and time the origin endpoint was modified.</p>
    pub modified_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    /// <p>An HTTP live streaming (HLS) manifest configuration.</p>
    pub hls_manifests: ::std::option::Option<::std::vec::Vec<crate::types::ListHlsManifestConfiguration>>,
    /// <p>A low-latency HLS manifest configuration.</p>
    pub low_latency_hls_manifests: ::std::option::Option<::std::vec::Vec<crate::types::ListLowLatencyHlsManifestConfiguration>>,
    /// <p>A DASH manifest configuration.</p>
    pub dash_manifests: ::std::option::Option<::std::vec::Vec<crate::types::ListDashManifestConfiguration>>,
    /// <p>A list of Microsoft Smooth Streaming (MSS) manifest configurations associated with the origin endpoint. Each configuration represents a different MSS streaming option available from this endpoint.</p>
    pub mss_manifests: ::std::option::Option<::std::vec::Vec<crate::types::ListMssManifestConfiguration>>,
    /// <p>The failover settings for the endpoint.</p>
    pub force_endpoint_error_configuration: ::std::option::Option<crate::types::ForceEndpointErrorConfiguration>,
}
impl OriginEndpointListConfiguration {
    /// <p>The Amazon Resource Name (ARN) associated with the resource.</p>
    pub fn arn(&self) -> &str {
        use std::ops::Deref;
        self.arn.deref()
    }
    /// <p>The name that describes the channel group. The name is the primary identifier for the channel group, and must be unique for your account in the AWS Region.</p>
    pub fn channel_group_name(&self) -> &str {
        use std::ops::Deref;
        self.channel_group_name.deref()
    }
    /// <p>The name that describes the channel. The name is the primary identifier for the channel, and must be unique for your account in the AWS Region and channel group.</p>
    pub fn channel_name(&self) -> &str {
        use std::ops::Deref;
        self.channel_name.deref()
    }
    /// <p>The name that describes the origin endpoint. The name is the primary identifier for the origin endpoint, and and must be unique for your account in the AWS Region and channel.</p>
    pub fn origin_endpoint_name(&self) -> &str {
        use std::ops::Deref;
        self.origin_endpoint_name.deref()
    }
    /// <p>The type of container attached to this origin endpoint. A container type is a file format that encapsulates one or more media streams, such as audio and video, into a single file.</p>
    pub fn container_type(&self) -> &crate::types::ContainerType {
        &self.container_type
    }
    /// <p>Any descriptive information that you want to add to the origin endpoint for future identification purposes.</p>
    pub fn description(&self) -> ::std::option::Option<&str> {
        self.description.as_deref()
    }
    /// <p>The date and time the origin endpoint was created.</p>
    pub fn created_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.created_at.as_ref()
    }
    /// <p>The date and time the origin endpoint was modified.</p>
    pub fn modified_at(&self) -> ::std::option::Option<&::aws_smithy_types::DateTime> {
        self.modified_at.as_ref()
    }
    /// <p>An HTTP live streaming (HLS) manifest configuration.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.hls_manifests.is_none()`.
    pub fn hls_manifests(&self) -> &[crate::types::ListHlsManifestConfiguration] {
        self.hls_manifests.as_deref().unwrap_or_default()
    }
    /// <p>A low-latency HLS manifest configuration.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.low_latency_hls_manifests.is_none()`.
    pub fn low_latency_hls_manifests(&self) -> &[crate::types::ListLowLatencyHlsManifestConfiguration] {
        self.low_latency_hls_manifests.as_deref().unwrap_or_default()
    }
    /// <p>A DASH manifest configuration.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.dash_manifests.is_none()`.
    pub fn dash_manifests(&self) -> &[crate::types::ListDashManifestConfiguration] {
        self.dash_manifests.as_deref().unwrap_or_default()
    }
    /// <p>A list of Microsoft Smooth Streaming (MSS) manifest configurations associated with the origin endpoint. Each configuration represents a different MSS streaming option available from this endpoint.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.mss_manifests.is_none()`.
    pub fn mss_manifests(&self) -> &[crate::types::ListMssManifestConfiguration] {
        self.mss_manifests.as_deref().unwrap_or_default()
    }
    /// <p>The failover settings for the endpoint.</p>
    pub fn force_endpoint_error_configuration(&self) -> ::std::option::Option<&crate::types::ForceEndpointErrorConfiguration> {
        self.force_endpoint_error_configuration.as_ref()
    }
}
impl OriginEndpointListConfiguration {
    /// Creates a new builder-style object to manufacture [`OriginEndpointListConfiguration`](crate::types::OriginEndpointListConfiguration).
    pub fn builder() -> crate::types::builders::OriginEndpointListConfigurationBuilder {
        crate::types::builders::OriginEndpointListConfigurationBuilder::default()
    }
}

/// A builder for [`OriginEndpointListConfiguration`](crate::types::OriginEndpointListConfiguration).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct OriginEndpointListConfigurationBuilder {
    pub(crate) arn: ::std::option::Option<::std::string::String>,
    pub(crate) channel_group_name: ::std::option::Option<::std::string::String>,
    pub(crate) channel_name: ::std::option::Option<::std::string::String>,
    pub(crate) origin_endpoint_name: ::std::option::Option<::std::string::String>,
    pub(crate) container_type: ::std::option::Option<crate::types::ContainerType>,
    pub(crate) description: ::std::option::Option<::std::string::String>,
    pub(crate) created_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) modified_at: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) hls_manifests: ::std::option::Option<::std::vec::Vec<crate::types::ListHlsManifestConfiguration>>,
    pub(crate) low_latency_hls_manifests: ::std::option::Option<::std::vec::Vec<crate::types::ListLowLatencyHlsManifestConfiguration>>,
    pub(crate) dash_manifests: ::std::option::Option<::std::vec::Vec<crate::types::ListDashManifestConfiguration>>,
    pub(crate) mss_manifests: ::std::option::Option<::std::vec::Vec<crate::types::ListMssManifestConfiguration>>,
    pub(crate) force_endpoint_error_configuration: ::std::option::Option<crate::types::ForceEndpointErrorConfiguration>,
}
impl OriginEndpointListConfigurationBuilder {
    /// <p>The Amazon Resource Name (ARN) associated with the resource.</p>
    /// This field is required.
    pub fn arn(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.arn = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the resource.</p>
    pub fn set_arn(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.arn = input;
        self
    }
    /// <p>The Amazon Resource Name (ARN) associated with the resource.</p>
    pub fn get_arn(&self) -> &::std::option::Option<::std::string::String> {
        &self.arn
    }
    /// <p>The name that describes the channel group. The name is the primary identifier for the channel group, and must be unique for your account in the AWS Region.</p>
    /// This field is required.
    pub fn channel_group_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_group_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name that describes the channel group. The name is the primary identifier for the channel group, and must be unique for your account in the AWS Region.</p>
    pub fn set_channel_group_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_group_name = input;
        self
    }
    /// <p>The name that describes the channel group. The name is the primary identifier for the channel group, and must be unique for your account in the AWS Region.</p>
    pub fn get_channel_group_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.channel_group_name
    }
    /// <p>The name that describes the channel. The name is the primary identifier for the channel, and must be unique for your account in the AWS Region and channel group.</p>
    /// This field is required.
    pub fn channel_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.channel_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name that describes the channel. The name is the primary identifier for the channel, and must be unique for your account in the AWS Region and channel group.</p>
    pub fn set_channel_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.channel_name = input;
        self
    }
    /// <p>The name that describes the channel. The name is the primary identifier for the channel, and must be unique for your account in the AWS Region and channel group.</p>
    pub fn get_channel_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.channel_name
    }
    /// <p>The name that describes the origin endpoint. The name is the primary identifier for the origin endpoint, and and must be unique for your account in the AWS Region and channel.</p>
    /// This field is required.
    pub fn origin_endpoint_name(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.origin_endpoint_name = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The name that describes the origin endpoint. The name is the primary identifier for the origin endpoint, and and must be unique for your account in the AWS Region and channel.</p>
    pub fn set_origin_endpoint_name(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.origin_endpoint_name = input;
        self
    }
    /// <p>The name that describes the origin endpoint. The name is the primary identifier for the origin endpoint, and and must be unique for your account in the AWS Region and channel.</p>
    pub fn get_origin_endpoint_name(&self) -> &::std::option::Option<::std::string::String> {
        &self.origin_endpoint_name
    }
    /// <p>The type of container attached to this origin endpoint. A container type is a file format that encapsulates one or more media streams, such as audio and video, into a single file.</p>
    /// This field is required.
    pub fn container_type(mut self, input: crate::types::ContainerType) -> Self {
        self.container_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>The type of container attached to this origin endpoint. A container type is a file format that encapsulates one or more media streams, such as audio and video, into a single file.</p>
    pub fn set_container_type(mut self, input: ::std::option::Option<crate::types::ContainerType>) -> Self {
        self.container_type = input;
        self
    }
    /// <p>The type of container attached to this origin endpoint. A container type is a file format that encapsulates one or more media streams, such as audio and video, into a single file.</p>
    pub fn get_container_type(&self) -> &::std::option::Option<crate::types::ContainerType> {
        &self.container_type
    }
    /// <p>Any descriptive information that you want to add to the origin endpoint for future identification purposes.</p>
    pub fn description(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.description = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>Any descriptive information that you want to add to the origin endpoint for future identification purposes.</p>
    pub fn set_description(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.description = input;
        self
    }
    /// <p>Any descriptive information that you want to add to the origin endpoint for future identification purposes.</p>
    pub fn get_description(&self) -> &::std::option::Option<::std::string::String> {
        &self.description
    }
    /// <p>The date and time the origin endpoint was created.</p>
    pub fn created_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.created_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time the origin endpoint was created.</p>
    pub fn set_created_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.created_at = input;
        self
    }
    /// <p>The date and time the origin endpoint was created.</p>
    pub fn get_created_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.created_at
    }
    /// <p>The date and time the origin endpoint was modified.</p>
    pub fn modified_at(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.modified_at = ::std::option::Option::Some(input);
        self
    }
    /// <p>The date and time the origin endpoint was modified.</p>
    pub fn set_modified_at(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.modified_at = input;
        self
    }
    /// <p>The date and time the origin endpoint was modified.</p>
    pub fn get_modified_at(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.modified_at
    }
    /// Appends an item to `hls_manifests`.
    ///
    /// To override the contents of this collection use [`set_hls_manifests`](Self::set_hls_manifests).
    ///
    /// <p>An HTTP live streaming (HLS) manifest configuration.</p>
    pub fn hls_manifests(mut self, input: crate::types::ListHlsManifestConfiguration) -> Self {
        let mut v = self.hls_manifests.unwrap_or_default();
        v.push(input);
        self.hls_manifests = ::std::option::Option::Some(v);
        self
    }
    /// <p>An HTTP live streaming (HLS) manifest configuration.</p>
    pub fn set_hls_manifests(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ListHlsManifestConfiguration>>) -> Self {
        self.hls_manifests = input;
        self
    }
    /// <p>An HTTP live streaming (HLS) manifest configuration.</p>
    pub fn get_hls_manifests(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ListHlsManifestConfiguration>> {
        &self.hls_manifests
    }
    /// Appends an item to `low_latency_hls_manifests`.
    ///
    /// To override the contents of this collection use [`set_low_latency_hls_manifests`](Self::set_low_latency_hls_manifests).
    ///
    /// <p>A low-latency HLS manifest configuration.</p>
    pub fn low_latency_hls_manifests(mut self, input: crate::types::ListLowLatencyHlsManifestConfiguration) -> Self {
        let mut v = self.low_latency_hls_manifests.unwrap_or_default();
        v.push(input);
        self.low_latency_hls_manifests = ::std::option::Option::Some(v);
        self
    }
    /// <p>A low-latency HLS manifest configuration.</p>
    pub fn set_low_latency_hls_manifests(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::ListLowLatencyHlsManifestConfiguration>>,
    ) -> Self {
        self.low_latency_hls_manifests = input;
        self
    }
    /// <p>A low-latency HLS manifest configuration.</p>
    pub fn get_low_latency_hls_manifests(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ListLowLatencyHlsManifestConfiguration>> {
        &self.low_latency_hls_manifests
    }
    /// Appends an item to `dash_manifests`.
    ///
    /// To override the contents of this collection use [`set_dash_manifests`](Self::set_dash_manifests).
    ///
    /// <p>A DASH manifest configuration.</p>
    pub fn dash_manifests(mut self, input: crate::types::ListDashManifestConfiguration) -> Self {
        let mut v = self.dash_manifests.unwrap_or_default();
        v.push(input);
        self.dash_manifests = ::std::option::Option::Some(v);
        self
    }
    /// <p>A DASH manifest configuration.</p>
    pub fn set_dash_manifests(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ListDashManifestConfiguration>>) -> Self {
        self.dash_manifests = input;
        self
    }
    /// <p>A DASH manifest configuration.</p>
    pub fn get_dash_manifests(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ListDashManifestConfiguration>> {
        &self.dash_manifests
    }
    /// Appends an item to `mss_manifests`.
    ///
    /// To override the contents of this collection use [`set_mss_manifests`](Self::set_mss_manifests).
    ///
    /// <p>A list of Microsoft Smooth Streaming (MSS) manifest configurations associated with the origin endpoint. Each configuration represents a different MSS streaming option available from this endpoint.</p>
    pub fn mss_manifests(mut self, input: crate::types::ListMssManifestConfiguration) -> Self {
        let mut v = self.mss_manifests.unwrap_or_default();
        v.push(input);
        self.mss_manifests = ::std::option::Option::Some(v);
        self
    }
    /// <p>A list of Microsoft Smooth Streaming (MSS) manifest configurations associated with the origin endpoint. Each configuration represents a different MSS streaming option available from this endpoint.</p>
    pub fn set_mss_manifests(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::ListMssManifestConfiguration>>) -> Self {
        self.mss_manifests = input;
        self
    }
    /// <p>A list of Microsoft Smooth Streaming (MSS) manifest configurations associated with the origin endpoint. Each configuration represents a different MSS streaming option available from this endpoint.</p>
    pub fn get_mss_manifests(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::ListMssManifestConfiguration>> {
        &self.mss_manifests
    }
    /// <p>The failover settings for the endpoint.</p>
    pub fn force_endpoint_error_configuration(mut self, input: crate::types::ForceEndpointErrorConfiguration) -> Self {
        self.force_endpoint_error_configuration = ::std::option::Option::Some(input);
        self
    }
    /// <p>The failover settings for the endpoint.</p>
    pub fn set_force_endpoint_error_configuration(mut self, input: ::std::option::Option<crate::types::ForceEndpointErrorConfiguration>) -> Self {
        self.force_endpoint_error_configuration = input;
        self
    }
    /// <p>The failover settings for the endpoint.</p>
    pub fn get_force_endpoint_error_configuration(&self) -> &::std::option::Option<crate::types::ForceEndpointErrorConfiguration> {
        &self.force_endpoint_error_configuration
    }
    /// Consumes the builder and constructs a [`OriginEndpointListConfiguration`](crate::types::OriginEndpointListConfiguration).
    /// This method will fail if any of the following fields are not set:
    /// - [`arn`](crate::types::builders::OriginEndpointListConfigurationBuilder::arn)
    /// - [`channel_group_name`](crate::types::builders::OriginEndpointListConfigurationBuilder::channel_group_name)
    /// - [`channel_name`](crate::types::builders::OriginEndpointListConfigurationBuilder::channel_name)
    /// - [`origin_endpoint_name`](crate::types::builders::OriginEndpointListConfigurationBuilder::origin_endpoint_name)
    /// - [`container_type`](crate::types::builders::OriginEndpointListConfigurationBuilder::container_type)
    pub fn build(self) -> ::std::result::Result<crate::types::OriginEndpointListConfiguration, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::OriginEndpointListConfiguration {
            arn: self.arn.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "arn",
                    "arn was not specified but it is required when building OriginEndpointListConfiguration",
                )
            })?,
            channel_group_name: self.channel_group_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "channel_group_name",
                    "channel_group_name was not specified but it is required when building OriginEndpointListConfiguration",
                )
            })?,
            channel_name: self.channel_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "channel_name",
                    "channel_name was not specified but it is required when building OriginEndpointListConfiguration",
                )
            })?,
            origin_endpoint_name: self.origin_endpoint_name.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "origin_endpoint_name",
                    "origin_endpoint_name was not specified but it is required when building OriginEndpointListConfiguration",
                )
            })?,
            container_type: self.container_type.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "container_type",
                    "container_type was not specified but it is required when building OriginEndpointListConfiguration",
                )
            })?,
            description: self.description,
            created_at: self.created_at,
            modified_at: self.modified_at,
            hls_manifests: self.hls_manifests,
            low_latency_hls_manifests: self.low_latency_hls_manifests,
            dash_manifests: self.dash_manifests,
            mss_manifests: self.mss_manifests,
            force_endpoint_error_configuration: self.force_endpoint_error_configuration,
        })
    }
}
