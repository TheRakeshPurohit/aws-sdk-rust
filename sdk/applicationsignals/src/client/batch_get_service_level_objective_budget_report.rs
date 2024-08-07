// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`BatchGetServiceLevelObjectiveBudgetReport`](crate::operation::batch_get_service_level_objective_budget_report::builders::BatchGetServiceLevelObjectiveBudgetReportFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`timestamp(DateTime)`](crate::operation::batch_get_service_level_objective_budget_report::builders::BatchGetServiceLevelObjectiveBudgetReportFluentBuilder::timestamp) / [`set_timestamp(Option<DateTime>)`](crate::operation::batch_get_service_level_objective_budget_report::builders::BatchGetServiceLevelObjectiveBudgetReportFluentBuilder::set_timestamp):<br>required: **true**<br><p>The date and time that you want the report to be for. It is expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p><br>
    ///   - [`slo_ids(impl Into<String>)`](crate::operation::batch_get_service_level_objective_budget_report::builders::BatchGetServiceLevelObjectiveBudgetReportFluentBuilder::slo_ids) / [`set_slo_ids(Option<Vec::<String>>)`](crate::operation::batch_get_service_level_objective_budget_report::builders::BatchGetServiceLevelObjectiveBudgetReportFluentBuilder::set_slo_ids):<br>required: **true**<br><p>An array containing the IDs of the service level objectives that you want to include in the report.</p><br>
    /// - On success, responds with [`BatchGetServiceLevelObjectiveBudgetReportOutput`](crate::operation::batch_get_service_level_objective_budget_report::BatchGetServiceLevelObjectiveBudgetReportOutput) with field(s):
    ///   - [`timestamp(DateTime)`](crate::operation::batch_get_service_level_objective_budget_report::BatchGetServiceLevelObjectiveBudgetReportOutput::timestamp): <p>The date and time that the report is for. It is expressed as the number of milliseconds since Jan 1, 1970 00:00:00 UTC.</p>
    ///   - [`reports(Vec::<ServiceLevelObjectiveBudgetReport>)`](crate::operation::batch_get_service_level_objective_budget_report::BatchGetServiceLevelObjectiveBudgetReportOutput::reports): <p>An array of structures, where each structure is one budget report.</p>
    ///   - [`errors(Vec::<ServiceLevelObjectiveBudgetReportError>)`](crate::operation::batch_get_service_level_objective_budget_report::BatchGetServiceLevelObjectiveBudgetReportOutput::errors): <p>An array of structures, where each structure includes an error indicating that one of the requests in the array was not valid.</p>
    /// - On failure, responds with [`SdkError<BatchGetServiceLevelObjectiveBudgetReportError>`](crate::operation::batch_get_service_level_objective_budget_report::BatchGetServiceLevelObjectiveBudgetReportError)
    pub fn batch_get_service_level_objective_budget_report(
        &self,
    ) -> crate::operation::batch_get_service_level_objective_budget_report::builders::BatchGetServiceLevelObjectiveBudgetReportFluentBuilder {
        crate::operation::batch_get_service_level_objective_budget_report::builders::BatchGetServiceLevelObjectiveBudgetReportFluentBuilder::new(
            self.handle.clone(),
        )
    }
}
