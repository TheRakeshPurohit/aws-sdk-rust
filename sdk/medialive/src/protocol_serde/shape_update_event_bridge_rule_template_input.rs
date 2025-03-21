// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_update_event_bridge_rule_template_input_input(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::operation::update_event_bridge_rule_template::UpdateEventBridgeRuleTemplateInput,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    if let Some(var_1) = &input.description {
        object.key("description").string(var_1.as_str());
    }
    if let Some(var_2) = &input.event_targets {
        let mut array_3 = object.key("eventTargets").start_array();
        for item_4 in var_2 {
            {
                #[allow(unused_mut)]
                let mut object_5 = array_3.value().start_object();
                crate::protocol_serde::shape_event_bridge_rule_template_target::ser_event_bridge_rule_template_target(&mut object_5, item_4)?;
                object_5.finish();
            }
        }
        array_3.finish();
    }
    if let Some(var_6) = &input.event_type {
        object.key("eventType").string(var_6.as_str());
    }
    if let Some(var_7) = &input.group_identifier {
        object.key("groupIdentifier").string(var_7.as_str());
    }
    if let Some(var_8) = &input.name {
        object.key("name").string(var_8.as_str());
    }
    Ok(())
}
