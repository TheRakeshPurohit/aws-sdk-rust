// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub use crate::types::_template::Template;

pub use crate::types::_receipt_rule::ReceiptRule;

pub use crate::types::_receipt_action::ReceiptAction;

pub use crate::types::_connect_action::ConnectAction;

pub use crate::types::_sns_action::SnsAction;

pub use crate::types::_sns_action_encoding::SnsActionEncoding;

pub use crate::types::_add_header_action::AddHeaderAction;

pub use crate::types::_stop_action::StopAction;

pub use crate::types::_stop_scope::StopScope;

pub use crate::types::_lambda_action::LambdaAction;

pub use crate::types::_invocation_type::InvocationType;

pub use crate::types::_workmail_action::WorkmailAction;

pub use crate::types::_bounce_action::BounceAction;

pub use crate::types::_s3_action::S3Action;

pub use crate::types::_tls_policy::TlsPolicy;

pub use crate::types::_tracking_options::TrackingOptions;

pub use crate::types::_event_destination::EventDestination;

pub use crate::types::_sns_destination::SnsDestination;

pub use crate::types::_cloud_watch_destination::CloudWatchDestination;

pub use crate::types::_cloud_watch_dimension_configuration::CloudWatchDimensionConfiguration;

pub use crate::types::_dimension_value_source::DimensionValueSource;

pub use crate::types::_kinesis_firehose_destination::KinesisFirehoseDestination;

pub use crate::types::_event_type::EventType;

pub use crate::types::_notification_type::NotificationType;

pub use crate::types::_behavior_on_mx_failure::BehaviorOnMxFailure;

pub use crate::types::_message_tag::MessageTag;

pub use crate::types::_destination::Destination;

pub use crate::types::_raw_message::RawMessage;

pub use crate::types::_message::Message;

pub use crate::types::_body::Body;

pub use crate::types::_content::Content;

pub use crate::types::_bulk_email_destination_status::BulkEmailDestinationStatus;

pub use crate::types::_bulk_email_status::BulkEmailStatus;

pub use crate::types::_bulk_email_destination::BulkEmailDestination;

pub use crate::types::_bounced_recipient_info::BouncedRecipientInfo;

pub use crate::types::_recipient_dsn_fields::RecipientDsnFields;

pub use crate::types::_extension_field::ExtensionField;

pub use crate::types::_dsn_action::DsnAction;

pub use crate::types::_bounce_type::BounceType;

pub use crate::types::_message_dsn::MessageDsn;

pub use crate::types::_delivery_options::DeliveryOptions;

pub use crate::types::_template_metadata::TemplateMetadata;

pub use crate::types::_receipt_rule_set_metadata::ReceiptRuleSetMetadata;

pub use crate::types::_receipt_filter::ReceiptFilter;

pub use crate::types::_receipt_ip_filter::ReceiptIpFilter;

pub use crate::types::_receipt_filter_policy::ReceiptFilterPolicy;

pub use crate::types::_identity_type::IdentityType;

pub use crate::types::_custom_verification_email_template::CustomVerificationEmailTemplate;

pub use crate::types::_configuration_set::ConfigurationSet;

pub use crate::types::_send_data_point::SendDataPoint;

pub use crate::types::_identity_verification_attributes::IdentityVerificationAttributes;

pub use crate::types::_verification_status::VerificationStatus;

pub use crate::types::_identity_notification_attributes::IdentityNotificationAttributes;

pub use crate::types::_identity_mail_from_domain_attributes::IdentityMailFromDomainAttributes;

pub use crate::types::_custom_mail_from_status::CustomMailFromStatus;

pub use crate::types::_identity_dkim_attributes::IdentityDkimAttributes;

pub use crate::types::_reputation_options::ReputationOptions;

pub use crate::types::_configuration_set_attribute::ConfigurationSetAttribute;

mod _add_header_action;

mod _behavior_on_mx_failure;

mod _body;

mod _bounce_action;

mod _bounce_type;

mod _bounced_recipient_info;

mod _bulk_email_destination;

mod _bulk_email_destination_status;

mod _bulk_email_status;

mod _cloud_watch_destination;

mod _cloud_watch_dimension_configuration;

mod _configuration_set;

mod _configuration_set_attribute;

mod _connect_action;

mod _content;

mod _custom_mail_from_status;

mod _custom_verification_email_template;

mod _delivery_options;

mod _destination;

mod _dimension_value_source;

mod _dsn_action;

mod _event_destination;

mod _event_type;

mod _extension_field;

mod _identity_dkim_attributes;

mod _identity_mail_from_domain_attributes;

mod _identity_notification_attributes;

mod _identity_type;

mod _identity_verification_attributes;

mod _invocation_type;

mod _kinesis_firehose_destination;

mod _lambda_action;

mod _message;

mod _message_dsn;

mod _message_tag;

mod _notification_type;

mod _raw_message;

mod _receipt_action;

mod _receipt_filter;

mod _receipt_filter_policy;

mod _receipt_ip_filter;

mod _receipt_rule;

mod _receipt_rule_set_metadata;

mod _recipient_dsn_fields;

mod _reputation_options;

mod _s3_action;

mod _send_data_point;

mod _sns_action;

mod _sns_action_encoding;

mod _sns_destination;

mod _stop_action;

mod _stop_scope;

mod _template;

mod _template_metadata;

mod _tls_policy;

mod _tracking_options;

mod _verification_status;

mod _workmail_action;

/// Builders
pub mod builders;

/// Error types that Amazon Simple Email Service can respond with.
pub mod error;
