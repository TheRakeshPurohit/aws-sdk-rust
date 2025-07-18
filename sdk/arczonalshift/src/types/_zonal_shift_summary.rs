// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// <p>Lists information about zonal shifts in Amazon Application Recovery Controller, including zonal shifts that you start yourself and zonal shifts that ARC starts on your behalf for practice runs with zonal autoshift.</p>
/// <p>Zonal shifts are temporary, including customer-initiated zonal shifts and the zonal autoshift practice run zonal shifts that ARC starts weekly, on your behalf. A zonal shift that a customer starts can be active for up to three days (72 hours). A practice run zonal shift has a 30 minute duration.</p>
#[non_exhaustive]
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::fmt::Debug)]
pub struct ZonalShiftSummary {
    /// <p>The identifier of a zonal shift.</p>
    pub zonal_shift_id: ::std::string::String,
    /// <p>The identifier for the resource to include in a zonal shift. The identifier is the Amazon Resource Name (ARN) for the resource.</p>
    /// <p>Amazon Application Recovery Controller currently supports enabling the following resources for zonal shift and zonal autoshift:</p>
    /// <ul>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.ec2-auto-scaling-groups.html">Amazon EC2 Auto Scaling groups</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.eks.html">Amazon Elastic Kubernetes Service</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.app-load-balancers.html">Application Load Balancers</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.network-load-balancers.html">Network Load Balancers</a></p></li>
    /// </ul>
    pub resource_identifier: ::std::string::String,
    /// <p>The Availability Zone (for example, <code>use1-az1</code>) that traffic is moved away from for a resource when you start a zonal shift. Until the zonal shift expires or you cancel it, traffic for the resource is instead moved to other Availability Zones in the Amazon Web Services Region.</p>
    pub away_from: ::std::string::String,
    /// <p>The expiry time (expiration time) for a customer-initiated zonal shift. A zonal shift is temporary and must be set to expire when you start the zonal shift. You can initially set a zonal shift to expire in a maximum of three days (72 hours). However, you can update a zonal shift to set a new expiration at any time.</p>
    /// <p>When you start a zonal shift, you specify how long you want it to be active, which ARC converts to an expiry time (expiration time). You can cancel a zonal shift when you're ready to restore traffic to the Availability Zone, or just wait for it to expire. Or you can update the zonal shift to specify another length of time to expire in.</p>
    pub expiry_time: ::aws_smithy_types::DateTime,
    /// <p>The time (UTC) when the zonal shift starts.</p>
    pub start_time: ::aws_smithy_types::DateTime,
    /// <p>A status for a zonal shift.</p>
    /// <p>The <code>Status</code> for a zonal shift can have one of the following values:</p>
    /// <ul>
    /// <li>
    /// <p><b>ACTIVE:</b> The zonal shift has been started and is active.</p></li>
    /// <li>
    /// <p><b>EXPIRED:</b> The zonal shift has expired (the expiry time was exceeded).</p></li>
    /// <li>
    /// <p><b>CANCELED:</b> The zonal shift was canceled.</p></li>
    /// </ul>
    pub status: crate::types::ZonalShiftStatus,
    /// <p>A comment that you enter about the zonal shift. Only the latest comment is retained; no comment history is maintained. That is, a new comment overwrites any existing comment string.</p>
    pub comment: ::std::string::String,
    /// <p>Defines the zonal shift type.</p>
    pub shift_type: ::std::option::Option<crate::types::ShiftType>,
    /// <p>The outcome, or end state, of a practice run. The following values can be returned:</p>
    /// <ul>
    /// <li>
    /// <p><b>PENDING:</b> Outcome value when the practice run is in progress.</p></li>
    /// <li>
    /// <p><b>SUCCEEDED:</b> Outcome value when the outcome alarm specified for the practice run configuration does not go into an <code>ALARM</code> state during the practice run, and the practice run was not interrupted before it completed.</p></li>
    /// <li>
    /// <p><b>INTERRUPTED:</b> Outcome value when the practice run did not run for the expected 30 minutes or there was another problem with the practice run that created an inconclusive outcome.</p></li>
    /// <li>
    /// <p><b>FAILED:</b> Outcome value when the outcome alarm specified for the practice run configuration goes into an <code>ALARM</code> state during the practice run, and the practice run was not interrupted before it completed.</p></li>
    /// <li>
    /// <p><b>CAPACITY_CHECK_FAILED:</b> The check for balanced capacity across Availability Zones for your load balancing and Auto Scaling group resources failed.</p></li>
    /// </ul>
    /// <p>For more information about practice run outcomes, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-autoshift.configure.html"> Considerations when you configure zonal autoshift</a> in the Amazon Application Recovery Controller Developer Guide.</p>
    pub practice_run_outcome: ::std::option::Option<crate::types::PracticeRunOutcome>,
}
impl ZonalShiftSummary {
    /// <p>The identifier of a zonal shift.</p>
    pub fn zonal_shift_id(&self) -> &str {
        use std::ops::Deref;
        self.zonal_shift_id.deref()
    }
    /// <p>The identifier for the resource to include in a zonal shift. The identifier is the Amazon Resource Name (ARN) for the resource.</p>
    /// <p>Amazon Application Recovery Controller currently supports enabling the following resources for zonal shift and zonal autoshift:</p>
    /// <ul>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.ec2-auto-scaling-groups.html">Amazon EC2 Auto Scaling groups</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.eks.html">Amazon Elastic Kubernetes Service</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.app-load-balancers.html">Application Load Balancers</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.network-load-balancers.html">Network Load Balancers</a></p></li>
    /// </ul>
    pub fn resource_identifier(&self) -> &str {
        use std::ops::Deref;
        self.resource_identifier.deref()
    }
    /// <p>The Availability Zone (for example, <code>use1-az1</code>) that traffic is moved away from for a resource when you start a zonal shift. Until the zonal shift expires or you cancel it, traffic for the resource is instead moved to other Availability Zones in the Amazon Web Services Region.</p>
    pub fn away_from(&self) -> &str {
        use std::ops::Deref;
        self.away_from.deref()
    }
    /// <p>The expiry time (expiration time) for a customer-initiated zonal shift. A zonal shift is temporary and must be set to expire when you start the zonal shift. You can initially set a zonal shift to expire in a maximum of three days (72 hours). However, you can update a zonal shift to set a new expiration at any time.</p>
    /// <p>When you start a zonal shift, you specify how long you want it to be active, which ARC converts to an expiry time (expiration time). You can cancel a zonal shift when you're ready to restore traffic to the Availability Zone, or just wait for it to expire. Or you can update the zonal shift to specify another length of time to expire in.</p>
    pub fn expiry_time(&self) -> &::aws_smithy_types::DateTime {
        &self.expiry_time
    }
    /// <p>The time (UTC) when the zonal shift starts.</p>
    pub fn start_time(&self) -> &::aws_smithy_types::DateTime {
        &self.start_time
    }
    /// <p>A status for a zonal shift.</p>
    /// <p>The <code>Status</code> for a zonal shift can have one of the following values:</p>
    /// <ul>
    /// <li>
    /// <p><b>ACTIVE:</b> The zonal shift has been started and is active.</p></li>
    /// <li>
    /// <p><b>EXPIRED:</b> The zonal shift has expired (the expiry time was exceeded).</p></li>
    /// <li>
    /// <p><b>CANCELED:</b> The zonal shift was canceled.</p></li>
    /// </ul>
    pub fn status(&self) -> &crate::types::ZonalShiftStatus {
        &self.status
    }
    /// <p>A comment that you enter about the zonal shift. Only the latest comment is retained; no comment history is maintained. That is, a new comment overwrites any existing comment string.</p>
    pub fn comment(&self) -> &str {
        use std::ops::Deref;
        self.comment.deref()
    }
    /// <p>Defines the zonal shift type.</p>
    pub fn shift_type(&self) -> ::std::option::Option<&crate::types::ShiftType> {
        self.shift_type.as_ref()
    }
    /// <p>The outcome, or end state, of a practice run. The following values can be returned:</p>
    /// <ul>
    /// <li>
    /// <p><b>PENDING:</b> Outcome value when the practice run is in progress.</p></li>
    /// <li>
    /// <p><b>SUCCEEDED:</b> Outcome value when the outcome alarm specified for the practice run configuration does not go into an <code>ALARM</code> state during the practice run, and the practice run was not interrupted before it completed.</p></li>
    /// <li>
    /// <p><b>INTERRUPTED:</b> Outcome value when the practice run did not run for the expected 30 minutes or there was another problem with the practice run that created an inconclusive outcome.</p></li>
    /// <li>
    /// <p><b>FAILED:</b> Outcome value when the outcome alarm specified for the practice run configuration goes into an <code>ALARM</code> state during the practice run, and the practice run was not interrupted before it completed.</p></li>
    /// <li>
    /// <p><b>CAPACITY_CHECK_FAILED:</b> The check for balanced capacity across Availability Zones for your load balancing and Auto Scaling group resources failed.</p></li>
    /// </ul>
    /// <p>For more information about practice run outcomes, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-autoshift.configure.html"> Considerations when you configure zonal autoshift</a> in the Amazon Application Recovery Controller Developer Guide.</p>
    pub fn practice_run_outcome(&self) -> ::std::option::Option<&crate::types::PracticeRunOutcome> {
        self.practice_run_outcome.as_ref()
    }
}
impl ZonalShiftSummary {
    /// Creates a new builder-style object to manufacture [`ZonalShiftSummary`](crate::types::ZonalShiftSummary).
    pub fn builder() -> crate::types::builders::ZonalShiftSummaryBuilder {
        crate::types::builders::ZonalShiftSummaryBuilder::default()
    }
}

/// A builder for [`ZonalShiftSummary`](crate::types::ZonalShiftSummary).
#[derive(::std::clone::Clone, ::std::cmp::PartialEq, ::std::default::Default, ::std::fmt::Debug)]
#[non_exhaustive]
pub struct ZonalShiftSummaryBuilder {
    pub(crate) zonal_shift_id: ::std::option::Option<::std::string::String>,
    pub(crate) resource_identifier: ::std::option::Option<::std::string::String>,
    pub(crate) away_from: ::std::option::Option<::std::string::String>,
    pub(crate) expiry_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) start_time: ::std::option::Option<::aws_smithy_types::DateTime>,
    pub(crate) status: ::std::option::Option<crate::types::ZonalShiftStatus>,
    pub(crate) comment: ::std::option::Option<::std::string::String>,
    pub(crate) shift_type: ::std::option::Option<crate::types::ShiftType>,
    pub(crate) practice_run_outcome: ::std::option::Option<crate::types::PracticeRunOutcome>,
}
impl ZonalShiftSummaryBuilder {
    /// <p>The identifier of a zonal shift.</p>
    /// This field is required.
    pub fn zonal_shift_id(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.zonal_shift_id = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier of a zonal shift.</p>
    pub fn set_zonal_shift_id(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.zonal_shift_id = input;
        self
    }
    /// <p>The identifier of a zonal shift.</p>
    pub fn get_zonal_shift_id(&self) -> &::std::option::Option<::std::string::String> {
        &self.zonal_shift_id
    }
    /// <p>The identifier for the resource to include in a zonal shift. The identifier is the Amazon Resource Name (ARN) for the resource.</p>
    /// <p>Amazon Application Recovery Controller currently supports enabling the following resources for zonal shift and zonal autoshift:</p>
    /// <ul>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.ec2-auto-scaling-groups.html">Amazon EC2 Auto Scaling groups</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.eks.html">Amazon Elastic Kubernetes Service</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.app-load-balancers.html">Application Load Balancers</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.network-load-balancers.html">Network Load Balancers</a></p></li>
    /// </ul>
    /// This field is required.
    pub fn resource_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.resource_identifier = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The identifier for the resource to include in a zonal shift. The identifier is the Amazon Resource Name (ARN) for the resource.</p>
    /// <p>Amazon Application Recovery Controller currently supports enabling the following resources for zonal shift and zonal autoshift:</p>
    /// <ul>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.ec2-auto-scaling-groups.html">Amazon EC2 Auto Scaling groups</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.eks.html">Amazon Elastic Kubernetes Service</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.app-load-balancers.html">Application Load Balancers</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.network-load-balancers.html">Network Load Balancers</a></p></li>
    /// </ul>
    pub fn set_resource_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.resource_identifier = input;
        self
    }
    /// <p>The identifier for the resource to include in a zonal shift. The identifier is the Amazon Resource Name (ARN) for the resource.</p>
    /// <p>Amazon Application Recovery Controller currently supports enabling the following resources for zonal shift and zonal autoshift:</p>
    /// <ul>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.ec2-auto-scaling-groups.html">Amazon EC2 Auto Scaling groups</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.eks.html">Amazon Elastic Kubernetes Service</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.app-load-balancers.html">Application Load Balancers</a></p></li>
    /// <li>
    /// <p><a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-shift.resource-types.network-load-balancers.html">Network Load Balancers</a></p></li>
    /// </ul>
    pub fn get_resource_identifier(&self) -> &::std::option::Option<::std::string::String> {
        &self.resource_identifier
    }
    /// <p>The Availability Zone (for example, <code>use1-az1</code>) that traffic is moved away from for a resource when you start a zonal shift. Until the zonal shift expires or you cancel it, traffic for the resource is instead moved to other Availability Zones in the Amazon Web Services Region.</p>
    /// This field is required.
    pub fn away_from(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.away_from = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>The Availability Zone (for example, <code>use1-az1</code>) that traffic is moved away from for a resource when you start a zonal shift. Until the zonal shift expires or you cancel it, traffic for the resource is instead moved to other Availability Zones in the Amazon Web Services Region.</p>
    pub fn set_away_from(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.away_from = input;
        self
    }
    /// <p>The Availability Zone (for example, <code>use1-az1</code>) that traffic is moved away from for a resource when you start a zonal shift. Until the zonal shift expires or you cancel it, traffic for the resource is instead moved to other Availability Zones in the Amazon Web Services Region.</p>
    pub fn get_away_from(&self) -> &::std::option::Option<::std::string::String> {
        &self.away_from
    }
    /// <p>The expiry time (expiration time) for a customer-initiated zonal shift. A zonal shift is temporary and must be set to expire when you start the zonal shift. You can initially set a zonal shift to expire in a maximum of three days (72 hours). However, you can update a zonal shift to set a new expiration at any time.</p>
    /// <p>When you start a zonal shift, you specify how long you want it to be active, which ARC converts to an expiry time (expiration time). You can cancel a zonal shift when you're ready to restore traffic to the Availability Zone, or just wait for it to expire. Or you can update the zonal shift to specify another length of time to expire in.</p>
    /// This field is required.
    pub fn expiry_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.expiry_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The expiry time (expiration time) for a customer-initiated zonal shift. A zonal shift is temporary and must be set to expire when you start the zonal shift. You can initially set a zonal shift to expire in a maximum of three days (72 hours). However, you can update a zonal shift to set a new expiration at any time.</p>
    /// <p>When you start a zonal shift, you specify how long you want it to be active, which ARC converts to an expiry time (expiration time). You can cancel a zonal shift when you're ready to restore traffic to the Availability Zone, or just wait for it to expire. Or you can update the zonal shift to specify another length of time to expire in.</p>
    pub fn set_expiry_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.expiry_time = input;
        self
    }
    /// <p>The expiry time (expiration time) for a customer-initiated zonal shift. A zonal shift is temporary and must be set to expire when you start the zonal shift. You can initially set a zonal shift to expire in a maximum of three days (72 hours). However, you can update a zonal shift to set a new expiration at any time.</p>
    /// <p>When you start a zonal shift, you specify how long you want it to be active, which ARC converts to an expiry time (expiration time). You can cancel a zonal shift when you're ready to restore traffic to the Availability Zone, or just wait for it to expire. Or you can update the zonal shift to specify another length of time to expire in.</p>
    pub fn get_expiry_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.expiry_time
    }
    /// <p>The time (UTC) when the zonal shift starts.</p>
    /// This field is required.
    pub fn start_time(mut self, input: ::aws_smithy_types::DateTime) -> Self {
        self.start_time = ::std::option::Option::Some(input);
        self
    }
    /// <p>The time (UTC) when the zonal shift starts.</p>
    pub fn set_start_time(mut self, input: ::std::option::Option<::aws_smithy_types::DateTime>) -> Self {
        self.start_time = input;
        self
    }
    /// <p>The time (UTC) when the zonal shift starts.</p>
    pub fn get_start_time(&self) -> &::std::option::Option<::aws_smithy_types::DateTime> {
        &self.start_time
    }
    /// <p>A status for a zonal shift.</p>
    /// <p>The <code>Status</code> for a zonal shift can have one of the following values:</p>
    /// <ul>
    /// <li>
    /// <p><b>ACTIVE:</b> The zonal shift has been started and is active.</p></li>
    /// <li>
    /// <p><b>EXPIRED:</b> The zonal shift has expired (the expiry time was exceeded).</p></li>
    /// <li>
    /// <p><b>CANCELED:</b> The zonal shift was canceled.</p></li>
    /// </ul>
    /// This field is required.
    pub fn status(mut self, input: crate::types::ZonalShiftStatus) -> Self {
        self.status = ::std::option::Option::Some(input);
        self
    }
    /// <p>A status for a zonal shift.</p>
    /// <p>The <code>Status</code> for a zonal shift can have one of the following values:</p>
    /// <ul>
    /// <li>
    /// <p><b>ACTIVE:</b> The zonal shift has been started and is active.</p></li>
    /// <li>
    /// <p><b>EXPIRED:</b> The zonal shift has expired (the expiry time was exceeded).</p></li>
    /// <li>
    /// <p><b>CANCELED:</b> The zonal shift was canceled.</p></li>
    /// </ul>
    pub fn set_status(mut self, input: ::std::option::Option<crate::types::ZonalShiftStatus>) -> Self {
        self.status = input;
        self
    }
    /// <p>A status for a zonal shift.</p>
    /// <p>The <code>Status</code> for a zonal shift can have one of the following values:</p>
    /// <ul>
    /// <li>
    /// <p><b>ACTIVE:</b> The zonal shift has been started and is active.</p></li>
    /// <li>
    /// <p><b>EXPIRED:</b> The zonal shift has expired (the expiry time was exceeded).</p></li>
    /// <li>
    /// <p><b>CANCELED:</b> The zonal shift was canceled.</p></li>
    /// </ul>
    pub fn get_status(&self) -> &::std::option::Option<crate::types::ZonalShiftStatus> {
        &self.status
    }
    /// <p>A comment that you enter about the zonal shift. Only the latest comment is retained; no comment history is maintained. That is, a new comment overwrites any existing comment string.</p>
    /// This field is required.
    pub fn comment(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.comment = ::std::option::Option::Some(input.into());
        self
    }
    /// <p>A comment that you enter about the zonal shift. Only the latest comment is retained; no comment history is maintained. That is, a new comment overwrites any existing comment string.</p>
    pub fn set_comment(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.comment = input;
        self
    }
    /// <p>A comment that you enter about the zonal shift. Only the latest comment is retained; no comment history is maintained. That is, a new comment overwrites any existing comment string.</p>
    pub fn get_comment(&self) -> &::std::option::Option<::std::string::String> {
        &self.comment
    }
    /// <p>Defines the zonal shift type.</p>
    pub fn shift_type(mut self, input: crate::types::ShiftType) -> Self {
        self.shift_type = ::std::option::Option::Some(input);
        self
    }
    /// <p>Defines the zonal shift type.</p>
    pub fn set_shift_type(mut self, input: ::std::option::Option<crate::types::ShiftType>) -> Self {
        self.shift_type = input;
        self
    }
    /// <p>Defines the zonal shift type.</p>
    pub fn get_shift_type(&self) -> &::std::option::Option<crate::types::ShiftType> {
        &self.shift_type
    }
    /// <p>The outcome, or end state, of a practice run. The following values can be returned:</p>
    /// <ul>
    /// <li>
    /// <p><b>PENDING:</b> Outcome value when the practice run is in progress.</p></li>
    /// <li>
    /// <p><b>SUCCEEDED:</b> Outcome value when the outcome alarm specified for the practice run configuration does not go into an <code>ALARM</code> state during the practice run, and the practice run was not interrupted before it completed.</p></li>
    /// <li>
    /// <p><b>INTERRUPTED:</b> Outcome value when the practice run did not run for the expected 30 minutes or there was another problem with the practice run that created an inconclusive outcome.</p></li>
    /// <li>
    /// <p><b>FAILED:</b> Outcome value when the outcome alarm specified for the practice run configuration goes into an <code>ALARM</code> state during the practice run, and the practice run was not interrupted before it completed.</p></li>
    /// <li>
    /// <p><b>CAPACITY_CHECK_FAILED:</b> The check for balanced capacity across Availability Zones for your load balancing and Auto Scaling group resources failed.</p></li>
    /// </ul>
    /// <p>For more information about practice run outcomes, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-autoshift.configure.html"> Considerations when you configure zonal autoshift</a> in the Amazon Application Recovery Controller Developer Guide.</p>
    pub fn practice_run_outcome(mut self, input: crate::types::PracticeRunOutcome) -> Self {
        self.practice_run_outcome = ::std::option::Option::Some(input);
        self
    }
    /// <p>The outcome, or end state, of a practice run. The following values can be returned:</p>
    /// <ul>
    /// <li>
    /// <p><b>PENDING:</b> Outcome value when the practice run is in progress.</p></li>
    /// <li>
    /// <p><b>SUCCEEDED:</b> Outcome value when the outcome alarm specified for the practice run configuration does not go into an <code>ALARM</code> state during the practice run, and the practice run was not interrupted before it completed.</p></li>
    /// <li>
    /// <p><b>INTERRUPTED:</b> Outcome value when the practice run did not run for the expected 30 minutes or there was another problem with the practice run that created an inconclusive outcome.</p></li>
    /// <li>
    /// <p><b>FAILED:</b> Outcome value when the outcome alarm specified for the practice run configuration goes into an <code>ALARM</code> state during the practice run, and the practice run was not interrupted before it completed.</p></li>
    /// <li>
    /// <p><b>CAPACITY_CHECK_FAILED:</b> The check for balanced capacity across Availability Zones for your load balancing and Auto Scaling group resources failed.</p></li>
    /// </ul>
    /// <p>For more information about practice run outcomes, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-autoshift.configure.html"> Considerations when you configure zonal autoshift</a> in the Amazon Application Recovery Controller Developer Guide.</p>
    pub fn set_practice_run_outcome(mut self, input: ::std::option::Option<crate::types::PracticeRunOutcome>) -> Self {
        self.practice_run_outcome = input;
        self
    }
    /// <p>The outcome, or end state, of a practice run. The following values can be returned:</p>
    /// <ul>
    /// <li>
    /// <p><b>PENDING:</b> Outcome value when the practice run is in progress.</p></li>
    /// <li>
    /// <p><b>SUCCEEDED:</b> Outcome value when the outcome alarm specified for the practice run configuration does not go into an <code>ALARM</code> state during the practice run, and the practice run was not interrupted before it completed.</p></li>
    /// <li>
    /// <p><b>INTERRUPTED:</b> Outcome value when the practice run did not run for the expected 30 minutes or there was another problem with the practice run that created an inconclusive outcome.</p></li>
    /// <li>
    /// <p><b>FAILED:</b> Outcome value when the outcome alarm specified for the practice run configuration goes into an <code>ALARM</code> state during the practice run, and the practice run was not interrupted before it completed.</p></li>
    /// <li>
    /// <p><b>CAPACITY_CHECK_FAILED:</b> The check for balanced capacity across Availability Zones for your load balancing and Auto Scaling group resources failed.</p></li>
    /// </ul>
    /// <p>For more information about practice run outcomes, see <a href="https://docs.aws.amazon.com/r53recovery/latest/dg/arc-zonal-autoshift.configure.html"> Considerations when you configure zonal autoshift</a> in the Amazon Application Recovery Controller Developer Guide.</p>
    pub fn get_practice_run_outcome(&self) -> &::std::option::Option<crate::types::PracticeRunOutcome> {
        &self.practice_run_outcome
    }
    /// Consumes the builder and constructs a [`ZonalShiftSummary`](crate::types::ZonalShiftSummary).
    /// This method will fail if any of the following fields are not set:
    /// - [`zonal_shift_id`](crate::types::builders::ZonalShiftSummaryBuilder::zonal_shift_id)
    /// - [`resource_identifier`](crate::types::builders::ZonalShiftSummaryBuilder::resource_identifier)
    /// - [`away_from`](crate::types::builders::ZonalShiftSummaryBuilder::away_from)
    /// - [`expiry_time`](crate::types::builders::ZonalShiftSummaryBuilder::expiry_time)
    /// - [`start_time`](crate::types::builders::ZonalShiftSummaryBuilder::start_time)
    /// - [`status`](crate::types::builders::ZonalShiftSummaryBuilder::status)
    /// - [`comment`](crate::types::builders::ZonalShiftSummaryBuilder::comment)
    pub fn build(self) -> ::std::result::Result<crate::types::ZonalShiftSummary, ::aws_smithy_types::error::operation::BuildError> {
        ::std::result::Result::Ok(crate::types::ZonalShiftSummary {
            zonal_shift_id: self.zonal_shift_id.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "zonal_shift_id",
                    "zonal_shift_id was not specified but it is required when building ZonalShiftSummary",
                )
            })?,
            resource_identifier: self.resource_identifier.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "resource_identifier",
                    "resource_identifier was not specified but it is required when building ZonalShiftSummary",
                )
            })?,
            away_from: self.away_from.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "away_from",
                    "away_from was not specified but it is required when building ZonalShiftSummary",
                )
            })?,
            expiry_time: self.expiry_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "expiry_time",
                    "expiry_time was not specified but it is required when building ZonalShiftSummary",
                )
            })?,
            start_time: self.start_time.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "start_time",
                    "start_time was not specified but it is required when building ZonalShiftSummary",
                )
            })?,
            status: self.status.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "status",
                    "status was not specified but it is required when building ZonalShiftSummary",
                )
            })?,
            comment: self.comment.ok_or_else(|| {
                ::aws_smithy_types::error::operation::BuildError::missing_field(
                    "comment",
                    "comment was not specified but it is required when building ZonalShiftSummary",
                )
            })?,
            shift_type: self.shift_type,
            practice_run_outcome: self.practice_run_outcome,
        })
    }
}
