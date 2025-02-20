// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`StartTelemetryEvaluation`](crate::operation::start_telemetry_evaluation::builders::StartTelemetryEvaluationFluentBuilder) operation.
    ///
    /// - The fluent builder takes no input, just [`send`](crate::operation::start_telemetry_evaluation::builders::StartTelemetryEvaluationFluentBuilder::send) it.
    /// - On success, responds with [`StartTelemetryEvaluationOutput`](crate::operation::start_telemetry_evaluation::StartTelemetryEvaluationOutput)
    /// - On failure, responds with [`SdkError<StartTelemetryEvaluationError>`](crate::operation::start_telemetry_evaluation::StartTelemetryEvaluationError)
    pub fn start_telemetry_evaluation(&self) -> crate::operation::start_telemetry_evaluation::builders::StartTelemetryEvaluationFluentBuilder {
        crate::operation::start_telemetry_evaluation::builders::StartTelemetryEvaluationFluentBuilder::new(self.handle.clone())
    }
}
