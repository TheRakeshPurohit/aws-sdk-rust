// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
impl super::Client {
    /// Constructs a fluent builder for the [`UpdateScraper`](crate::operation::update_scraper::builders::UpdateScraperFluentBuilder) operation.
    ///
    /// - The fluent builder is configurable:
    ///   - [`scraper_id(impl Into<String>)`](crate::operation::update_scraper::builders::UpdateScraperFluentBuilder::scraper_id) / [`set_scraper_id(Option<String>)`](crate::operation::update_scraper::builders::UpdateScraperFluentBuilder::set_scraper_id):<br>required: **true**<br><p>The ID of the scraper to update.</p><br>
    ///   - [`alias(impl Into<String>)`](crate::operation::update_scraper::builders::UpdateScraperFluentBuilder::alias) / [`set_alias(Option<String>)`](crate::operation::update_scraper::builders::UpdateScraperFluentBuilder::set_alias):<br>required: **false**<br><p>The new alias of the scraper.</p><br>
    ///   - [`scrape_configuration(ScrapeConfiguration)`](crate::operation::update_scraper::builders::UpdateScraperFluentBuilder::scrape_configuration) / [`set_scrape_configuration(Option<ScrapeConfiguration>)`](crate::operation::update_scraper::builders::UpdateScraperFluentBuilder::set_scrape_configuration):<br>required: **false**<br><p>Contains the base-64 encoded YAML configuration for the scraper.</p><note>  <p>For more information about configuring a scraper, see <a href="https://docs.aws.amazon.com/prometheus/latest/userguide/AMP-collector-how-to.html">Using an Amazon Web Services managed collector</a> in the <i>Amazon Managed Service for Prometheus User Guide</i>.</p> </note><br>
    ///   - [`destination(Destination)`](crate::operation::update_scraper::builders::UpdateScraperFluentBuilder::destination) / [`set_destination(Option<Destination>)`](crate::operation::update_scraper::builders::UpdateScraperFluentBuilder::set_destination):<br>required: **false**<br><p>The new Amazon Managed Service for Prometheus workspace to send metrics to.</p><br>
    ///   - [`role_configuration(RoleConfiguration)`](crate::operation::update_scraper::builders::UpdateScraperFluentBuilder::role_configuration) / [`set_role_configuration(Option<RoleConfiguration>)`](crate::operation::update_scraper::builders::UpdateScraperFluentBuilder::set_role_configuration):<br>required: **false**<br><p>Use this structure to enable cross-account access, so that you can use a target account to access Prometheus metrics from source accounts.</p><br>
    ///   - [`client_token(impl Into<String>)`](crate::operation::update_scraper::builders::UpdateScraperFluentBuilder::client_token) / [`set_client_token(Option<String>)`](crate::operation::update_scraper::builders::UpdateScraperFluentBuilder::set_client_token):<br>required: **false**<br><p>A unique identifier that you can provide to ensure the idempotency of the request. Case-sensitive.</p><br>
    /// - On success, responds with [`UpdateScraperOutput`](crate::operation::update_scraper::UpdateScraperOutput) with field(s):
    ///   - [`scraper_id(String)`](crate::operation::update_scraper::UpdateScraperOutput::scraper_id): <p>The ID of the updated scraper.</p>
    ///   - [`arn(String)`](crate::operation::update_scraper::UpdateScraperOutput::arn): <p>The Amazon Resource Name (ARN) of the updated scraper.</p>
    ///   - [`status(Option<ScraperStatus>)`](crate::operation::update_scraper::UpdateScraperOutput::status): <p>A structure that displays the current status of the scraper.</p>
    ///   - [`tags(Option<HashMap::<String, String>>)`](crate::operation::update_scraper::UpdateScraperOutput::tags): <p>The list of tag keys and values that are associated with the scraper.</p>
    /// - On failure, responds with [`SdkError<UpdateScraperError>`](crate::operation::update_scraper::UpdateScraperError)
    pub fn update_scraper(&self) -> crate::operation::update_scraper::builders::UpdateScraperFluentBuilder {
        crate::operation::update_scraper::builders::UpdateScraperFluentBuilder::new(self.handle.clone())
    }
}
