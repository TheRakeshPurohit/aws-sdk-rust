// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn ser_entity_filter(
    object: &mut ::aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::types::EntityFilter,
) -> ::std::result::Result<(), ::aws_smithy_types::error::operation::SerializationError> {
    {
        let mut array_1 = object.key("eventArns").start_array();
        for item_2 in &input.event_arns {
            {
                array_1.value().string(item_2.as_str());
            }
        }
        array_1.finish();
    }
    if let Some(var_3) = &input.entity_arns {
        let mut array_4 = object.key("entityArns").start_array();
        for item_5 in var_3 {
            {
                array_4.value().string(item_5.as_str());
            }
        }
        array_4.finish();
    }
    if let Some(var_6) = &input.entity_values {
        let mut array_7 = object.key("entityValues").start_array();
        for item_8 in var_6 {
            {
                array_7.value().string(item_8.as_str());
            }
        }
        array_7.finish();
    }
    if let Some(var_9) = &input.last_updated_times {
        let mut array_10 = object.key("lastUpdatedTimes").start_array();
        for item_11 in var_9 {
            {
                #[allow(unused_mut)]
                let mut object_12 = array_10.value().start_object();
                crate::protocol_serde::shape_date_time_range::ser_date_time_range(&mut object_12, item_11)?;
                object_12.finish();
            }
        }
        array_10.finish();
    }
    if let Some(var_13) = &input.tags {
        let mut array_14 = object.key("tags").start_array();
        for item_15 in var_13 {
            {
                #[allow(unused_mut)]
                let mut object_16 = array_14.value().start_object();
                for (key_17, value_18) in item_15 {
                    {
                        object_16.key(key_17.as_str()).string(value_18.as_str());
                    }
                }
                object_16.finish();
            }
        }
        array_14.finish();
    }
    if let Some(var_19) = &input.status_codes {
        let mut array_20 = object.key("statusCodes").start_array();
        for item_21 in var_19 {
            {
                array_20.value().string(item_21.as_str());
            }
        }
        array_20.finish();
    }
    Ok(())
}
