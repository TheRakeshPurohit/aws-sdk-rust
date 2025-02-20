// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`ListBillEstimateLineItems`](crate::operation::list_bill_estimate_line_items::builders::ListBillEstimateLineItemsFluentBuilder) operation.
    /// This operation supports pagination; See [`into_paginator()`](crate::operation::list_bill_estimate_line_items::builders::ListBillEstimateLineItemsFluentBuilder::into_paginator).
    ///
    /// - The fluent builder is configurable:
    ///   - [`bill_estimate_id(impl Into<String>)`](crate::operation::list_bill_estimate_line_items::builders::ListBillEstimateLineItemsFluentBuilder::bill_estimate_id) / [`set_bill_estimate_id(Option<String>)`](crate::operation::list_bill_estimate_line_items::builders::ListBillEstimateLineItemsFluentBuilder::set_bill_estimate_id):<br>required: **true**<br><p>The unique identifier of the bill estimate to list line items for.</p><br>
    ///   - [`filters(ListBillEstimateLineItemsFilter)`](crate::operation::list_bill_estimate_line_items::builders::ListBillEstimateLineItemsFluentBuilder::filters) / [`set_filters(Option<Vec::<ListBillEstimateLineItemsFilter>>)`](crate::operation::list_bill_estimate_line_items::builders::ListBillEstimateLineItemsFluentBuilder::set_filters):<br>required: **false**<br><p>Filters to apply to the list of line items.</p><br>
    ///   - [`next_token(impl Into<String>)`](crate::operation::list_bill_estimate_line_items::builders::ListBillEstimateLineItemsFluentBuilder::next_token) / [`set_next_token(Option<String>)`](crate::operation::list_bill_estimate_line_items::builders::ListBillEstimateLineItemsFluentBuilder::set_next_token):<br>required: **false**<br><p>A token to retrieve the next page of results.</p><br>
    ///   - [`max_results(i32)`](crate::operation::list_bill_estimate_line_items::builders::ListBillEstimateLineItemsFluentBuilder::max_results) / [`set_max_results(Option<i32>)`](crate::operation::list_bill_estimate_line_items::builders::ListBillEstimateLineItemsFluentBuilder::set_max_results):<br>required: **false**<br><p>The maximum number of results to return per page.</p><br>
    /// - On success, responds with [`ListBillEstimateLineItemsOutput`](crate::operation::list_bill_estimate_line_items::ListBillEstimateLineItemsOutput) with field(s):
    ///   - [`items(Option<Vec::<BillEstimateLineItemSummary>>)`](crate::operation::list_bill_estimate_line_items::ListBillEstimateLineItemsOutput::items): <p>The list of line items associated with the bill estimate.</p>
    ///   - [`next_token(Option<String>)`](crate::operation::list_bill_estimate_line_items::ListBillEstimateLineItemsOutput::next_token): <p>A token to retrieve the next page of results, if any.</p>
    /// - On failure, responds with [`SdkError<ListBillEstimateLineItemsError>`](crate::operation::list_bill_estimate_line_items::ListBillEstimateLineItemsError)
    pub fn list_bill_estimate_line_items(&self) -> crate::operation::list_bill_estimate_line_items::builders::ListBillEstimateLineItemsFluentBuilder {
        crate::operation::list_bill_estimate_line_items::builders::ListBillEstimateLineItemsFluentBuilder::new(self.handle.clone())
    }
}
