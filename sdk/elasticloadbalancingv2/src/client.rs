// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[derive(Debug)]
pub(crate) struct Handle {
    pub(crate) conf: crate::Config,
    #[allow(dead_code)] // unused when a service does not provide any operations
    pub(crate) runtime_plugins: ::aws_smithy_runtime_api::client::runtime_plugin::RuntimePlugins,
}

/// Client for Elastic Load Balancing
///
/// Client for invoking operations on Elastic Load Balancing. Each operation on Elastic Load Balancing is a method on this
/// this struct. `.send()` MUST be invoked on the generated operations to dispatch the request to the service.
/// ## Constructing a `Client`
///
/// A [`Config`] is required to construct a client. For most use cases, the [`aws-config`]
/// crate should be used to automatically resolve this config using
/// [`aws_config::load_from_env()`], since this will resolve an [`SdkConfig`] which can be shared
/// across multiple different AWS SDK clients. This config resolution process can be customized
/// by calling [`aws_config::from_env()`] instead, which returns a [`ConfigLoader`] that uses
/// the [builder pattern] to customize the default config.
///
/// In the simplest case, creating a client looks as follows:
/// ```rust,no_run
/// # async fn wrapper() {
/// let config = aws_config::load_from_env().await;
/// let client = aws_sdk_elasticloadbalancingv2::Client::new(&config);
/// # }
/// ```
///
/// Occasionally, SDKs may have additional service-specific values that can be set on the [`Config`] that
/// is absent from [`SdkConfig`], or slightly different settings for a specific client may be desired.
/// The [`Builder`](crate::config::Builder) struct implements `From<&SdkConfig>`, so setting these specific settings can be
/// done as follows:
///
/// ```rust,no_run
/// # async fn wrapper() {
/// let sdk_config = ::aws_config::load_from_env().await;
/// let config = aws_sdk_elasticloadbalancingv2::config::Builder::from(&sdk_config)
/// # /*
///     .some_service_specific_setting("value")
/// # */
///     .build();
/// # }
/// ```
///
/// See the [`aws-config` docs] and [`Config`] for more information on customizing configuration.
///
/// _Note:_ Client construction is expensive due to connection thread pool initialization, and should
/// be done once at application start-up.
///
/// [`Config`]: crate::Config
/// [`ConfigLoader`]: https://docs.rs/aws-config/*/aws_config/struct.ConfigLoader.html
/// [`SdkConfig`]: https://docs.rs/aws-config/*/aws_config/struct.SdkConfig.html
/// [`aws-config` docs]: https://docs.rs/aws-config/*
/// [`aws-config`]: https://crates.io/crates/aws-config
/// [`aws_config::from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.from_env.html
/// [`aws_config::load_from_env()`]: https://docs.rs/aws-config/*/aws_config/fn.load_from_env.html
/// [builder pattern]: https://rust-lang.github.io/api-guidelines/type-safety.html#builders-enable-construction-of-complex-values-c-builder
/// # Using the `Client`
///
/// A client has a function for every operation that can be performed by the service.
/// For example, the [`AddListenerCertificates`](crate::operation::add_listener_certificates) operation has
/// a [`Client::add_listener_certificates`], function which returns a builder for that operation.
/// The fluent builder ultimately has a `send()` function that returns an async future that
/// returns a result, as illustrated below:
///
/// ```rust,ignore
/// let result = client.add_listener_certificates()
///     .listener_arn("example")
///     .send()
///     .await;
/// ```
///
/// The underlying HTTP requests that get made by this can be modified with the `customize_operation`
/// function on the fluent builder. See the [`customize`](crate::client::customize) module for more
/// information.
/// # Waiters
///
/// This client provides `wait_until` methods behind the [`Waiters`](crate::client::Waiters) trait.
/// To use them, simply import the trait, and then call one of the `wait_until` methods. This will
/// return a waiter fluent builder that takes various parameters, which are documented on the builder
/// type. Once parameters have been provided, the `wait` method can be called to initiate waiting.
///
/// For example, if there was a `wait_until_thing` method, it could look like:
/// ```rust,ignore
/// let result = client.wait_until_thing()
///     .thing_id("someId")
///     .wait(Duration::from_secs(120))
///     .await;
/// ```
#[derive(::std::clone::Clone, ::std::fmt::Debug)]
pub struct Client {
    handle: ::std::sync::Arc<Handle>,
}

impl Client {
    /// Creates a new client from the service [`Config`](crate::Config).
    ///
    /// # Panics
    ///
    /// This method will panic in the following cases:
    ///
    /// - Retries or timeouts are enabled without a `sleep_impl` configured.
    /// - Identity caching is enabled without a `sleep_impl` and `time_source` configured.
    /// - No `behavior_version` is provided.
    ///
    /// The panic message for each of these will have instructions on how to resolve them.
    #[track_caller]
    pub fn from_conf(conf: crate::Config) -> Self {
        let handle = Handle {
            conf: conf.clone(),
            runtime_plugins: crate::config::base_client_runtime_plugins(conf),
        };
        if let Err(err) = Self::validate_config(&handle) {
            panic!("Invalid client configuration: {err}");
        }
        Self {
            handle: ::std::sync::Arc::new(handle),
        }
    }

    /// Returns the client's configuration.
    pub fn config(&self) -> &crate::Config {
        &self.handle.conf
    }

    fn validate_config(handle: &Handle) -> ::std::result::Result<(), ::aws_smithy_runtime_api::box_error::BoxError> {
        let mut cfg = ::aws_smithy_types::config_bag::ConfigBag::base();
        handle
            .runtime_plugins
            .apply_client_configuration(&mut cfg)?
            .validate_base_client_config(&cfg)?;
        Ok(())
    }
}

///
/// Waiter functions for the client.
///
/// Import this trait to get `wait_until` methods on the client.
///
pub trait Waiters {
    /// Wait for `load_balancer_available`
    fn wait_until_load_balancer_available(&self) -> crate::waiters::load_balancer_available::LoadBalancerAvailableFluentBuilder;
    /// Wait for `load_balancer_exists`
    fn wait_until_load_balancer_exists(&self) -> crate::waiters::load_balancer_exists::LoadBalancerExistsFluentBuilder;
    /// Wait for `load_balancers_deleted`
    fn wait_until_load_balancers_deleted(&self) -> crate::waiters::load_balancers_deleted::LoadBalancersDeletedFluentBuilder;
    /// Wait for `target_deregistered`
    fn wait_until_target_deregistered(&self) -> crate::waiters::target_deregistered::TargetDeregisteredFluentBuilder;
    /// Wait for `target_in_service`
    fn wait_until_target_in_service(&self) -> crate::waiters::target_in_service::TargetInServiceFluentBuilder;
}
impl Waiters for Client {
    fn wait_until_load_balancer_available(&self) -> crate::waiters::load_balancer_available::LoadBalancerAvailableFluentBuilder {
        crate::waiters::load_balancer_available::LoadBalancerAvailableFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_load_balancer_exists(&self) -> crate::waiters::load_balancer_exists::LoadBalancerExistsFluentBuilder {
        crate::waiters::load_balancer_exists::LoadBalancerExistsFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_load_balancers_deleted(&self) -> crate::waiters::load_balancers_deleted::LoadBalancersDeletedFluentBuilder {
        crate::waiters::load_balancers_deleted::LoadBalancersDeletedFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_target_deregistered(&self) -> crate::waiters::target_deregistered::TargetDeregisteredFluentBuilder {
        crate::waiters::target_deregistered::TargetDeregisteredFluentBuilder::new(self.handle.clone())
    }
    fn wait_until_target_in_service(&self) -> crate::waiters::target_in_service::TargetInServiceFluentBuilder {
        crate::waiters::target_in_service::TargetInServiceFluentBuilder::new(self.handle.clone())
    }
}

impl Client {
    /// Creates a new client from an [SDK Config](::aws_types::sdk_config::SdkConfig).
    ///
    /// # Panics
    ///
    /// - This method will panic if the `sdk_config` is missing an async sleep implementation. If you experience this panic, set
    ///   the `sleep_impl` on the Config passed into this function to fix it.
    /// - This method will panic if the `sdk_config` is missing an HTTP connector. If you experience this panic, set the
    ///   `http_connector` on the Config passed into this function to fix it.
    /// - This method will panic if no `BehaviorVersion` is provided. If you experience this panic, set `behavior_version` on the Config or enable the `behavior-version-latest` Cargo feature.
    #[track_caller]
    pub fn new(sdk_config: &::aws_types::sdk_config::SdkConfig) -> Self {
        Self::from_conf(sdk_config.into())
    }
}

mod add_listener_certificates;

mod add_tags;

mod add_trust_store_revocations;

mod create_listener;

mod create_load_balancer;

mod create_rule;

mod create_target_group;

mod create_trust_store;

/// Operation customization and supporting types.
///
/// The underlying HTTP requests made during an operation can be customized
/// by calling the `customize()` method on the builder returned from a client
/// operation call. For example, this can be used to add an additional HTTP header:
///
/// ```ignore
/// # async fn wrapper() -> ::std::result::Result<(), aws_sdk_elasticloadbalancingv2::Error> {
/// # let client: aws_sdk_elasticloadbalancingv2::Client = unimplemented!();
/// use ::http::header::{HeaderName, HeaderValue};
///
/// let result = client.add_listener_certificates()
///     .customize()
///     .mutate_request(|req| {
///         // Add `x-example-header` with value
///         req.headers_mut()
///             .insert(
///                 HeaderName::from_static("x-example-header"),
///                 HeaderValue::from_static("1"),
///             );
///     })
///     .send()
///     .await;
/// # }
/// ```
pub mod customize;

mod delete_listener;

mod delete_load_balancer;

mod delete_rule;

mod delete_shared_trust_store_association;

mod delete_target_group;

mod delete_trust_store;

mod deregister_targets;

mod describe_account_limits;

mod describe_capacity_reservation;

mod describe_listener_attributes;

mod describe_listener_certificates;

mod describe_listeners;

mod describe_load_balancer_attributes;

mod describe_load_balancers;

mod describe_rules;

mod describe_ssl_policies;

mod describe_tags;

mod describe_target_group_attributes;

mod describe_target_groups;

mod describe_target_health;

mod describe_trust_store_associations;

mod describe_trust_store_revocations;

mod describe_trust_stores;

mod get_resource_policy;

mod get_trust_store_ca_certificates_bundle;

mod get_trust_store_revocation_content;

mod modify_capacity_reservation;

mod modify_ip_pools;

mod modify_listener;

mod modify_listener_attributes;

mod modify_load_balancer_attributes;

mod modify_rule;

mod modify_target_group;

mod modify_target_group_attributes;

mod modify_trust_store;

mod register_targets;

mod remove_listener_certificates;

mod remove_tags;

mod remove_trust_store_revocations;

mod set_ip_address_type;

mod set_rule_priorities;

mod set_security_groups;

mod set_subnets;
