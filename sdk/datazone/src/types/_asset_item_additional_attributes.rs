// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>The additional attributes of an inventory asset.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct AssetItemAdditionalAttributes {
    /// <p>The forms included in the additional attributes of an inventory asset.</p>
    pub forms_output: ::std::option::Option<::std::vec::Vec<crate::types::FormOutput>>,
    /// <p>The read-only forms included in the additional attributes of an inventory asset.</p>
    pub read_only_forms_output: ::std::option::Option<::std::vec::Vec<crate::types::FormOutput>>,
    /// <p>The latest time series data points forms included in the additional attributes of an asset.</p>
    pub latest_time_series_data_point_forms_output: ::std::option::Option<::std::vec::Vec<crate::types::TimeSeriesDataPointSummaryFormOutput>>,
    /// <p>List of rationales indicating why this item was matched by search.</p>
    pub match_rationale: ::std::option::Option<::std::vec::Vec<crate::types::MatchRationaleItem>>,
}
impl AssetItemAdditionalAttributes {
    /// <p>The forms included in the additional attributes of an inventory asset.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.forms_output.is_none()`.
    pub fn forms_output(&self) -> &[crate::types::FormOutput] {
        self.forms_output.as_deref().unwrap_or_default()
    }
    /// <p>The read-only forms included in the additional attributes of an inventory asset.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.read_only_forms_output.is_none()`.
    pub fn read_only_forms_output(&self) -> &[crate::types::FormOutput] {
        self.read_only_forms_output.as_deref().unwrap_or_default()
    }
    /// <p>The latest time series data points forms included in the additional attributes of an asset.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.latest_time_series_data_point_forms_output.is_none()`.
    pub fn latest_time_series_data_point_forms_output(&self) -> &[crate::types::TimeSeriesDataPointSummaryFormOutput] {
        self.latest_time_series_data_point_forms_output.as_deref().unwrap_or_default()
    }
    /// <p>List of rationales indicating why this item was matched by search.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.match_rationale.is_none()`.
    pub fn match_rationale(&self) -> &[crate::types::MatchRationaleItem] {
        self.match_rationale.as_deref().unwrap_or_default()
    }
}
impl AssetItemAdditionalAttributes {
    /// Creates a new builder-style object to manufacture [`AssetItemAdditionalAttributes`](crate::types::AssetItemAdditionalAttributes).
    pub fn builder() -> crate::types::builders::AssetItemAdditionalAttributesBuilder {
        crate::types::builders::AssetItemAdditionalAttributesBuilder::default()
    }
}

/// A builder for [`AssetItemAdditionalAttributes`](crate::types::AssetItemAdditionalAttributes).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct AssetItemAdditionalAttributesBuilder {
    pub(crate) forms_output: ::std::option::Option<::std::vec::Vec<crate::types::FormOutput>>,
    pub(crate) read_only_forms_output: ::std::option::Option<::std::vec::Vec<crate::types::FormOutput>>,
    pub(crate) latest_time_series_data_point_forms_output: ::std::option::Option<::std::vec::Vec<crate::types::TimeSeriesDataPointSummaryFormOutput>>,
    pub(crate) match_rationale: ::std::option::Option<::std::vec::Vec<crate::types::MatchRationaleItem>>,
}
impl AssetItemAdditionalAttributesBuilder {
    /// Appends an item to `forms_output`.
    ///
    /// To override the contents of this collection use [`set_forms_output`](Self::set_forms_output).
    ///
    /// <p>The forms included in the additional attributes of an inventory asset.</p>
    pub fn forms_output(mut self, input: crate::types::FormOutput) -> Self {
        let mut v = self.forms_output.unwrap_or_default();
        v.push(input);
        self.forms_output = ::std::option::Option::Some(v);
        self
    }
    /// <p>The forms included in the additional attributes of an inventory asset.</p>
    pub fn set_forms_output(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::FormOutput>>) -> Self {
        self.forms_output = input;
        self
    }
    /// <p>The forms included in the additional attributes of an inventory asset.</p>
    pub fn get_forms_output(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::FormOutput>> {
        &self.forms_output
    }
    /// Appends an item to `read_only_forms_output`.
    ///
    /// To override the contents of this collection use [`set_read_only_forms_output`](Self::set_read_only_forms_output).
    ///
    /// <p>The read-only forms included in the additional attributes of an inventory asset.</p>
    pub fn read_only_forms_output(mut self, input: crate::types::FormOutput) -> Self {
        let mut v = self.read_only_forms_output.unwrap_or_default();
        v.push(input);
        self.read_only_forms_output = ::std::option::Option::Some(v);
        self
    }
    /// <p>The read-only forms included in the additional attributes of an inventory asset.</p>
    pub fn set_read_only_forms_output(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::FormOutput>>) -> Self {
        self.read_only_forms_output = input;
        self
    }
    /// <p>The read-only forms included in the additional attributes of an inventory asset.</p>
    pub fn get_read_only_forms_output(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::FormOutput>> {
        &self.read_only_forms_output
    }
    /// Appends an item to `latest_time_series_data_point_forms_output`.
    ///
    /// To override the contents of this collection use [`set_latest_time_series_data_point_forms_output`](Self::set_latest_time_series_data_point_forms_output).
    ///
    /// <p>The latest time series data points forms included in the additional attributes of an asset.</p>
    pub fn latest_time_series_data_point_forms_output(mut self, input: crate::types::TimeSeriesDataPointSummaryFormOutput) -> Self {
        let mut v = self.latest_time_series_data_point_forms_output.unwrap_or_default();
        v.push(input);
        self.latest_time_series_data_point_forms_output = ::std::option::Option::Some(v);
        self
    }
    /// <p>The latest time series data points forms included in the additional attributes of an asset.</p>
    pub fn set_latest_time_series_data_point_forms_output(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::TimeSeriesDataPointSummaryFormOutput>>,
    ) -> Self {
        self.latest_time_series_data_point_forms_output = input;
        self
    }
    /// <p>The latest time series data points forms included in the additional attributes of an asset.</p>
    pub fn get_latest_time_series_data_point_forms_output(
        &self,
    ) -> &::std::option::Option<::std::vec::Vec<crate::types::TimeSeriesDataPointSummaryFormOutput>> {
        &self.latest_time_series_data_point_forms_output
    }
    /// Appends an item to `match_rationale`.
    ///
    /// To override the contents of this collection use [`set_match_rationale`](Self::set_match_rationale).
    ///
    /// <p>List of rationales indicating why this item was matched by search.</p>
    pub fn match_rationale(mut self, input: crate::types::MatchRationaleItem) -> Self {
        let mut v = self.match_rationale.unwrap_or_default();
        v.push(input);
        self.match_rationale = ::std::option::Option::Some(v);
        self
    }
    /// <p>List of rationales indicating why this item was matched by search.</p>
    pub fn set_match_rationale(mut self, input: ::std::option::Option<::std::vec::Vec<crate::types::MatchRationaleItem>>) -> Self {
        self.match_rationale = input;
        self
    }
    /// <p>List of rationales indicating why this item was matched by search.</p>
    pub fn get_match_rationale(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::MatchRationaleItem>> {
        &self.match_rationale
    }
    /// Consumes the builder and constructs a [`AssetItemAdditionalAttributes`](crate::types::AssetItemAdditionalAttributes).
    pub fn build(self) -> crate::types::AssetItemAdditionalAttributes {
        crate::types::AssetItemAdditionalAttributes {
            forms_output: self.forms_output,
            read_only_forms_output: self.read_only_forms_output,
            latest_time_series_data_point_forms_output: self.latest_time_series_data_point_forms_output,
            match_rationale: self.match_rationale,
        }
    }
}
