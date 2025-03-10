// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::operation::delete_db_cluster::_delete_db_cluster_output::DeleteDbClusterOutputBuilder;

pub use crate::operation::delete_db_cluster::_delete_db_cluster_input::DeleteDbClusterInputBuilder;

impl crate::operation::delete_db_cluster::builders::DeleteDbClusterInputBuilder {
    /// Sends a request with this input using the given client.
    pub async fn send_with(
        self,
        client: &crate::Client,
    ) -> ::std::result::Result<
        crate::operation::delete_db_cluster::DeleteDbClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_db_cluster::DeleteDBClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let mut fluent_builder = client.delete_db_cluster();
        fluent_builder.inner = self;
        fluent_builder.send().await
    }
}
/// Fluent builder constructing a request to `DeleteDBCluster`.
///
/// <p>The DeleteDBCluster action deletes a previously provisioned DB cluster. When you delete a DB cluster, all automated backups for that DB cluster are deleted and can't be recovered. Manual DB cluster snapshots of the specified DB cluster are not deleted.</p>
/// <p>If you're deleting a Multi-AZ DB cluster with read replicas, all cluster members are terminated and read replicas are promoted to standalone instances.</p>
/// <p>For more information on Amazon Aurora, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/AuroraUserGuide/CHAP_AuroraOverview.html"> What is Amazon Aurora?</a> in the <i>Amazon Aurora User Guide</i>.</p>
/// <p>For more information on Multi-AZ DB clusters, see <a href="https://docs.aws.amazon.com/AmazonRDS/latest/UserGuide/multi-az-db-clusters-concepts.html"> Multi-AZ DB cluster deployments</a> in the <i>Amazon RDS User Guide</i>.</p>
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct DeleteDBClusterFluentBuilder {
    handle: ::std::sync::Arc<crate::client::Handle>,
    inner: crate::operation::delete_db_cluster::builders::DeleteDbClusterInputBuilder,
    config_override: ::std::option::Option<crate::config::Builder>,
}
impl
    crate::client::customize::internal::CustomizableSend<
        crate::operation::delete_db_cluster::DeleteDbClusterOutput,
        crate::operation::delete_db_cluster::DeleteDBClusterError,
    > for DeleteDBClusterFluentBuilder
{
    fn send(
        self,
        config_override: crate::config::Builder,
    ) -> crate::client::customize::internal::BoxFuture<
        crate::client::customize::internal::SendResult<
            crate::operation::delete_db_cluster::DeleteDbClusterOutput,
            crate::operation::delete_db_cluster::DeleteDBClusterError,
        >,
    > {
        ::std::boxed::Box::pin(async move { self.config_override(config_override).send().await })
    }
}
impl DeleteDBClusterFluentBuilder {
    /// Creates a new `DeleteDBClusterFluentBuilder`.
    pub(crate) fn new(handle: ::std::sync::Arc<crate::client::Handle>) -> Self {
        Self {
            handle,
            inner: ::std::default::Default::default(),
            config_override: ::std::option::Option::None,
        }
    }
    /// Access the DeleteDBCluster as a reference.
    pub fn as_input(&self) -> &crate::operation::delete_db_cluster::builders::DeleteDbClusterInputBuilder {
        &self.inner
    }
    /// Sends the request and returns the response.
    ///
    /// If an error occurs, an `SdkError` will be returned with additional details that
    /// can be matched against.
    ///
    /// By default, any retryable failures will be retried twice. Retry behavior
    /// is configurable with the [RetryConfig](aws_smithy_types::retry::RetryConfig), which can be
    /// set when configuring the client.
    pub async fn send(
        self,
    ) -> ::std::result::Result<
        crate::operation::delete_db_cluster::DeleteDbClusterOutput,
        ::aws_smithy_runtime_api::client::result::SdkError<
            crate::operation::delete_db_cluster::DeleteDBClusterError,
            ::aws_smithy_runtime_api::client::orchestrator::HttpResponse,
        >,
    > {
        let input = self
            .inner
            .build()
            .map_err(::aws_smithy_runtime_api::client::result::SdkError::construction_failure)?;
        let runtime_plugins = crate::operation::delete_db_cluster::DeleteDBCluster::operation_runtime_plugins(
            self.handle.runtime_plugins.clone(),
            &self.handle.conf,
            self.config_override,
        );
        crate::operation::delete_db_cluster::DeleteDBCluster::orchestrate(&runtime_plugins, input).await
    }

    /// Consumes this builder, creating a customizable operation that can be modified before being sent.
    pub fn customize(
        self,
    ) -> crate::client::customize::CustomizableOperation<
        crate::operation::delete_db_cluster::DeleteDbClusterOutput,
        crate::operation::delete_db_cluster::DeleteDBClusterError,
        Self,
    > {
        crate::client::customize::CustomizableOperation::new(self)
    }
    pub(crate) fn config_override(mut self, config_override: impl ::std::convert::Into<crate::config::Builder>) -> Self {
        self.set_config_override(::std::option::Option::Some(config_override.into()));
        self
    }

    pub(crate) fn set_config_override(&mut self, config_override: ::std::option::Option<crate::config::Builder>) -> &mut Self {
        self.config_override = config_override;
        self
    }
    /// <p>The DB cluster identifier for the DB cluster to be deleted. This parameter isn't case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must match an existing DBClusterIdentifier.</p></li>
    /// </ul>
    pub fn db_cluster_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.db_cluster_identifier(input.into());
        self
    }
    /// <p>The DB cluster identifier for the DB cluster to be deleted. This parameter isn't case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must match an existing DBClusterIdentifier.</p></li>
    /// </ul>
    pub fn set_db_cluster_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_db_cluster_identifier(input);
        self
    }
    /// <p>The DB cluster identifier for the DB cluster to be deleted. This parameter isn't case-sensitive.</p>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must match an existing DBClusterIdentifier.</p></li>
    /// </ul>
    pub fn get_db_cluster_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_db_cluster_identifier()
    }
    /// <p>Specifies whether to skip the creation of a final DB cluster snapshot before RDS deletes the DB cluster. If you set this value to <code>true</code>, RDS doesn't create a final DB cluster snapshot. If you set this value to <code>false</code> or don't specify it, RDS creates a DB cluster snapshot before it deletes the DB cluster. By default, this parameter is disabled, so RDS creates a final DB cluster snapshot.</p><note>
    /// <p>If <code>SkipFinalSnapshot</code> is disabled, you must specify a value for the <code>FinalDBSnapshotIdentifier</code> parameter.</p>
    /// </note>
    pub fn skip_final_snapshot(mut self, input: bool) -> Self {
        self.inner = self.inner.skip_final_snapshot(input);
        self
    }
    /// <p>Specifies whether to skip the creation of a final DB cluster snapshot before RDS deletes the DB cluster. If you set this value to <code>true</code>, RDS doesn't create a final DB cluster snapshot. If you set this value to <code>false</code> or don't specify it, RDS creates a DB cluster snapshot before it deletes the DB cluster. By default, this parameter is disabled, so RDS creates a final DB cluster snapshot.</p><note>
    /// <p>If <code>SkipFinalSnapshot</code> is disabled, you must specify a value for the <code>FinalDBSnapshotIdentifier</code> parameter.</p>
    /// </note>
    pub fn set_skip_final_snapshot(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_skip_final_snapshot(input);
        self
    }
    /// <p>Specifies whether to skip the creation of a final DB cluster snapshot before RDS deletes the DB cluster. If you set this value to <code>true</code>, RDS doesn't create a final DB cluster snapshot. If you set this value to <code>false</code> or don't specify it, RDS creates a DB cluster snapshot before it deletes the DB cluster. By default, this parameter is disabled, so RDS creates a final DB cluster snapshot.</p><note>
    /// <p>If <code>SkipFinalSnapshot</code> is disabled, you must specify a value for the <code>FinalDBSnapshotIdentifier</code> parameter.</p>
    /// </note>
    pub fn get_skip_final_snapshot(&self) -> &::std::option::Option<bool> {
        self.inner.get_skip_final_snapshot()
    }
    /// <p>The DB cluster snapshot identifier of the new DB cluster snapshot created when <code>SkipFinalSnapshot</code> is disabled.</p><note>
    /// <p>If you specify this parameter and also skip the creation of a final DB cluster snapshot with the <code>SkipFinalShapshot</code> parameter, the request results in an error.</p>
    /// </note>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must be 1 to 255 letters, numbers, or hyphens.</p></li>
    /// <li>
    /// <p>First character must be a letter</p></li>
    /// <li>
    /// <p>Can't end with a hyphen or contain two consecutive hyphens</p></li>
    /// </ul>
    pub fn final_db_snapshot_identifier(mut self, input: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.inner = self.inner.final_db_snapshot_identifier(input.into());
        self
    }
    /// <p>The DB cluster snapshot identifier of the new DB cluster snapshot created when <code>SkipFinalSnapshot</code> is disabled.</p><note>
    /// <p>If you specify this parameter and also skip the creation of a final DB cluster snapshot with the <code>SkipFinalShapshot</code> parameter, the request results in an error.</p>
    /// </note>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must be 1 to 255 letters, numbers, or hyphens.</p></li>
    /// <li>
    /// <p>First character must be a letter</p></li>
    /// <li>
    /// <p>Can't end with a hyphen or contain two consecutive hyphens</p></li>
    /// </ul>
    pub fn set_final_db_snapshot_identifier(mut self, input: ::std::option::Option<::std::string::String>) -> Self {
        self.inner = self.inner.set_final_db_snapshot_identifier(input);
        self
    }
    /// <p>The DB cluster snapshot identifier of the new DB cluster snapshot created when <code>SkipFinalSnapshot</code> is disabled.</p><note>
    /// <p>If you specify this parameter and also skip the creation of a final DB cluster snapshot with the <code>SkipFinalShapshot</code> parameter, the request results in an error.</p>
    /// </note>
    /// <p>Constraints:</p>
    /// <ul>
    /// <li>
    /// <p>Must be 1 to 255 letters, numbers, or hyphens.</p></li>
    /// <li>
    /// <p>First character must be a letter</p></li>
    /// <li>
    /// <p>Can't end with a hyphen or contain two consecutive hyphens</p></li>
    /// </ul>
    pub fn get_final_db_snapshot_identifier(&self) -> &::std::option::Option<::std::string::String> {
        self.inner.get_final_db_snapshot_identifier()
    }
    /// <p>Specifies whether to remove automated backups immediately after the DB cluster is deleted. This parameter isn't case-sensitive. The default is to remove automated backups immediately after the DB cluster is deleted, unless the Amazon Web Services Backup policy specifies a point-in-time restore rule.</p>
    pub fn delete_automated_backups(mut self, input: bool) -> Self {
        self.inner = self.inner.delete_automated_backups(input);
        self
    }
    /// <p>Specifies whether to remove automated backups immediately after the DB cluster is deleted. This parameter isn't case-sensitive. The default is to remove automated backups immediately after the DB cluster is deleted, unless the Amazon Web Services Backup policy specifies a point-in-time restore rule.</p>
    pub fn set_delete_automated_backups(mut self, input: ::std::option::Option<bool>) -> Self {
        self.inner = self.inner.set_delete_automated_backups(input);
        self
    }
    /// <p>Specifies whether to remove automated backups immediately after the DB cluster is deleted. This parameter isn't case-sensitive. The default is to remove automated backups immediately after the DB cluster is deleted, unless the Amazon Web Services Backup policy specifies a point-in-time restore rule.</p>
    pub fn get_delete_automated_backups(&self) -> &::std::option::Option<bool> {
        self.inner.get_delete_automated_backups()
    }
}
