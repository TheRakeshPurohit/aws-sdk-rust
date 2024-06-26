// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateAnalysisTemplate`](crate::operation::update_analysis_template::builders::UpdateAnalysisTemplateFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`membership_identifier(impl Into<String>)`](crate::operation::update_analysis_template::builders::UpdateAnalysisTemplateFluentBuilder::membership_identifier) / [`set_membership_identifier(Option<String>)`](crate::operation::update_analysis_template::builders::UpdateAnalysisTemplateFluentBuilder::set_membership_identifier):<br>required: **true**<br><p>The identifier for a membership resource.</p><br>
    ///   - [`analysis_template_identifier(impl Into<String>)`](crate::operation::update_analysis_template::builders::UpdateAnalysisTemplateFluentBuilder::analysis_template_identifier) / [`set_analysis_template_identifier(Option<String>)`](crate::operation::update_analysis_template::builders::UpdateAnalysisTemplateFluentBuilder::set_analysis_template_identifier):<br>required: **true**<br><p>The identifier for the analysis template resource.</p><br>
    ///   - [`description(impl Into<String>)`](crate::operation::update_analysis_template::builders::UpdateAnalysisTemplateFluentBuilder::description) / [`set_description(Option<String>)`](crate::operation::update_analysis_template::builders::UpdateAnalysisTemplateFluentBuilder::set_description):<br>required: **false**<br><p>A new description for the analysis template.</p><br>
    /// - On success, responds with [`UpdateAnalysisTemplateOutput`](crate::operation::update_analysis_template::UpdateAnalysisTemplateOutput) with field(s):
    ///   - [`analysis_template(Option<AnalysisTemplate>)`](crate::operation::update_analysis_template::UpdateAnalysisTemplateOutput::analysis_template): <p>The analysis template.</p>
    /// - On failure, responds with [`SdkError<UpdateAnalysisTemplateError>`](crate::operation::update_analysis_template::UpdateAnalysisTemplateError)
    pub fn update_analysis_template(&self) -> crate::operation::update_analysis_template::builders::UpdateAnalysisTemplateFluentBuilder {
        crate::operation::update_analysis_template::builders::UpdateAnalysisTemplateFluentBuilder::new(self.handle.clone())
    }
}
