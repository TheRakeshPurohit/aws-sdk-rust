// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Describes the availability of capacity for a Capacity Block.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct CapacityBlockStatus {
    /// <p>The ID of the Capacity Block.</p>
    pub capacity_block_id: ::std::option::Option<::std::string::String>,
    /// <p>The status of the high-bandwidth accelerator interconnect. Possible states include:</p>
    /// <ul>
    /// <li>
    /// <p><code>ok</code> the accelerator interconnect is healthy.</p></li>
    /// <li>
    /// <p><code>impaired</code> - accelerator interconnect communication is impaired.</p></li>
    /// <li>
    /// <p><code>insufficient-data</code> - insufficient data to determine accelerator interconnect status.</p></li>
    /// </ul>
    pub interconnect_status: ::std::option::Option<crate::types::CapacityBlockInterconnectStatus>,
    /// <p>The combined amount of <code>Available</code> and <code>Unavailable</code> capacity in the Capacity Block.</p>
    pub total_capacity: ::std::option::Option<i32>,
    /// <p>The remaining capacity. Indicates the number of resources that can be launched into the Capacity Block.</p>
    pub total_available_capacity: ::std::option::Option<i32>,
    /// <p>The unavailable capacity. Indicates the instance capacity that is unavailable for use due to a system status check failure.</p>
    pub total_unavailable_capacity: ::std::option::Option<i32>,
    /// <p>The availability of capacity for the Capacity Block reservations.</p>
    pub capacity_reservation_statuses: ::std::option::Option<::std::vec::Vec<crate::types::CapacityReservationStatus>>,
}
impl CapacityBlockStatus {
    /// <p>The ID of the Capacity Block.</p>
    pub fn capacity_block_id(&self) -> ::std::option::Option<&str> {
        self.capacity_block_id.as_deref()
    }
    /// <p>The status of the high-bandwidth accelerator interconnect. Possible states include:</p>
    /// <ul>
    /// <li>
    /// <p><code>ok</code> the accelerator interconnect is healthy.</p></li>
    /// <li>
    /// <p><code>impaired</code> - accelerator interconnect communication is impaired.</p></li>
    /// <li>
    /// <p><code>insufficient-data</code> - insufficient data to determine accelerator interconnect status.</p></li>
    /// </ul>
    pub fn interconnect_status(&self) -> ::std::option::Option<&crate::types::CapacityBlockInterconnectStatus> {
        self.interconnect_status.as_ref()
    }
    /// <p>The combined amount of <code>Available</code> and <code>Unavailable</code> capacity in the Capacity Block.</p>
    pub fn total_capacity(&self) -> ::std::option::Option<i32> {
        self.total_capacity
    }
    /// <p>The remaining capacity. Indicates the number of resources that can be launched into the Capacity Block.</p>
    pub fn total_available_capacity(&self) -> ::std::option::Option<i32> {
        self.total_available_capacity
    }
    /// <p>The unavailable capacity. Indicates the instance capacity that is unavailable for use due to a system status check failure.</p>
    pub fn total_unavailable_capacity(&self) -> ::std::option::Option<i32> {
        self.total_unavailable_capacity
    }
    /// <p>The availability of capacity for the Capacity Block reservations.</p>
    ///
    /// If no value was sent for this field, a default will be set. If you want to determine if no value was sent, use `.capacity_reservation_statuses.is_none()`.
    pub fn capacity_reservation_statuses(&self) -> &[crate::types::CapacityReservationStatus] {
        self.capacity_reservation_statuses.as_deref().unwrap_or_default()
    }
}
impl CapacityBlockStatus {
    /// Creates a new builder-style object to manufacture [`CapacityBlockStatus`](crate::types::CapacityBlockStatus).
    pub fn builder() -> crate::types::builders::CapacityBlockStatusBuilder {
        crate::types::builders::CapacityBlockStatusBuilder::default()
    }
}

/// A builder for [`CapacityBlockStatus`](crate::types::CapacityBlockStatus).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct CapacityBlockStatusBuilder {
    pub(crate) capacity_block_id: ::std::option::Option<::std::string::String>,
    pub(crate) interconnect_status: ::std::option::Option<crate::types::CapacityBlockInterconnectStatus>,
    pub(crate) total_capacity: ::std::option::Option<i32>,
    pub(crate) total_available_capacity: ::std::option::Option<i32>,
    pub(crate) total_unavailable_capacity: ::std::option::Option<i32>,
    pub(crate) capacity_reservation_statuses: ::std::option::Option<::std::vec::Vec<crate::types::CapacityReservationStatus>>,
}
impl CapacityBlockStatusBuilder {
    /// <p>The ID of the Capacity Block.</p>
    pub fn capacity_block_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.capacity_block_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The ID of the Capacity Block.</p>
    pub fn set_capacity_block_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.capacity_block_id = input;
        self
    }
    /// <p>The ID of the Capacity Block.</p>
    pub fn get_capacity_block_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.capacity_block_id
    }
    /// <p>The status of the high-bandwidth accelerator interconnect. Possible states include:</p>
    /// <ul>
    /// <li>
    /// <p><code>ok</code> the accelerator interconnect is healthy.</p></li>
    /// <li>
    /// <p><code>impaired</code> - accelerator interconnect communication is impaired.</p></li>
    /// <li>
    /// <p><code>insufficient-data</code> - insufficient data to determine accelerator interconnect status.</p></li>
    /// </ul>
    pub fn interconnect_status(mut self, input: crate::types::CapacityBlockInterconnectStatus) -> Self {
        self.interconnect_status = ::std::option::Option::Some(input);
        self
    }
    /// <p>The status of the high-bandwidth accelerator interconnect. Possible states include:</p>
    /// <ul>
    /// <li>
    /// <p><code>ok</code> the accelerator interconnect is healthy.</p></li>
    /// <li>
    /// <p><code>impaired</code> - accelerator interconnect communication is impaired.</p></li>
    /// <li>
    /// <p><code>insufficient-data</code> - insufficient data to determine accelerator interconnect status.</p></li>
    /// </ul>
    pub fn set_interconnect_status(mut self, input: ::std::option::Option<crate::types::CapacityBlockInterconnectStatus>) -> Self {
        self.interconnect_status = input;
        self
    }
    /// <p>The status of the high-bandwidth accelerator interconnect. Possible states include:</p>
    /// <ul>
    /// <li>
    /// <p><code>ok</code> the accelerator interconnect is healthy.</p></li>
    /// <li>
    /// <p><code>impaired</code> - accelerator interconnect communication is impaired.</p></li>
    /// <li>
    /// <p><code>insufficient-data</code> - insufficient data to determine accelerator interconnect status.</p></li>
    /// </ul>
    pub fn get_interconnect_status(&self) -> &::std::option::Option<crate::types::CapacityBlockInterconnectStatus> {
        &self.interconnect_status
    }
    /// <p>The combined amount of <code>Available</code> and <code>Unavailable</code> capacity in the Capacity Block.</p>
    pub fn total_capacity(mut self, input: i32) -> Self {
        self.total_capacity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The combined amount of <code>Available</code> and <code>Unavailable</code> capacity in the Capacity Block.</p>
    pub fn set_total_capacity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.total_capacity = input;
        self
    }
    /// <p>The combined amount of <code>Available</code> and <code>Unavailable</code> capacity in the Capacity Block.</p>
    pub fn get_total_capacity(&self) -> &::std::option::Option<i32> {
        &self.total_capacity
    }
    /// <p>The remaining capacity. Indicates the number of resources that can be launched into the Capacity Block.</p>
    pub fn total_available_capacity(mut self, input: i32) -> Self {
        self.total_available_capacity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The remaining capacity. Indicates the number of resources that can be launched into the Capacity Block.</p>
    pub fn set_total_available_capacity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.total_available_capacity = input;
        self
    }
    /// <p>The remaining capacity. Indicates the number of resources that can be launched into the Capacity Block.</p>
    pub fn get_total_available_capacity(&self) -> &::std::option::Option<i32> {
        &self.total_available_capacity
    }
    /// <p>The unavailable capacity. Indicates the instance capacity that is unavailable for use due to a system status check failure.</p>
    pub fn total_unavailable_capacity(mut self, input: i32) -> Self {
        self.total_unavailable_capacity = ::std::option::Option::Some(input);
        self
    }
    /// <p>The unavailable capacity. Indicates the instance capacity that is unavailable for use due to a system status check failure.</p>
    pub fn set_total_unavailable_capacity(mut self, input: ::std::option::Option<i32>) -> Self {
        self.total_unavailable_capacity = input;
        self
    }
    /// <p>The unavailable capacity. Indicates the instance capacity that is unavailable for use due to a system status check failure.</p>
    pub fn get_total_unavailable_capacity(&self) -> &::std::option::Option<i32> {
        &self.total_unavailable_capacity
    }
    /// Appends an item to `capacity_reservation_statuses`.
    ///
    /// To override the contents of this collection use [`set_capacity_reservation_statuses`](Self::set_capacity_reservation_statuses).
    ///
    /// <p>The availability of capacity for the Capacity Block reservations.</p>
    pub fn capacity_reservation_statuses(mut self, input: crate::types::CapacityReservationStatus) -> Self {
        let mut v = self.capacity_reservation_statuses.unwrap_or_default();
        v.push(input);
        self.capacity_reservation_statuses = ::std::option::Option::Some(v);
        self
    }
    /// <p>The availability of capacity for the Capacity Block reservations.</p>
    pub fn set_capacity_reservation_statuses(
        mut self,
        input: ::std::option::Option<::std::vec::Vec<crate::types::CapacityReservationStatus>>,
    ) -> Self {
        self.capacity_reservation_statuses = input;
        self
    }
    /// <p>The availability of capacity for the Capacity Block reservations.</p>
    pub fn get_capacity_reservation_statuses(&self) -> &::std::option::Option<::std::vec::Vec<crate::types::CapacityReservationStatus>> {
        &self.capacity_reservation_statuses
    }
    /// Consumes the builder and constructs a [`CapacityBlockStatus`](crate::types::CapacityBlockStatus).
    pub fn build(self) -> crate::types::CapacityBlockStatus {
        crate::types::CapacityBlockStatus {
            capacity_block_id: self.capacity_block_id,
            interconnect_status: self.interconnect_status,
            total_capacity: self.total_capacity,
            total_available_capacity: self.total_available_capacity,
            total_unavailable_capacity: self.total_unavailable_capacity,
            capacity_reservation_statuses: self.capacity_reservation_statuses,
        }
    }
}
