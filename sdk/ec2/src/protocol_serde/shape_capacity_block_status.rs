// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_capacity_block_status(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::CapacityBlockStatus, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::CapacityBlockStatus::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("capacityBlockId") /* CapacityBlockId com.amazonaws.ec2#CapacityBlockStatus$CapacityBlockId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_capacity_block_id(var_1);
            }
            ,
            s if s.matches("interconnectStatus") /* InterconnectStatus com.amazonaws.ec2#CapacityBlockStatus$InterconnectStatus */ =>  {
                let var_2 =
                    Some(
                        Result::<crate::types::CapacityBlockInterconnectStatus, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::CapacityBlockInterconnectStatus::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_interconnect_status(var_2);
            }
            ,
            s if s.matches("totalCapacity") /* TotalCapacity com.amazonaws.ec2#CapacityBlockStatus$TotalCapacity */ =>  {
                let var_3 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_total_capacity(var_3);
            }
            ,
            s if s.matches("totalAvailableCapacity") /* TotalAvailableCapacity com.amazonaws.ec2#CapacityBlockStatus$TotalAvailableCapacity */ =>  {
                let var_4 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_total_available_capacity(var_4);
            }
            ,
            s if s.matches("totalUnavailableCapacity") /* TotalUnavailableCapacity com.amazonaws.ec2#CapacityBlockStatus$TotalUnavailableCapacity */ =>  {
                let var_5 =
                    Some(
                         {
                            <i32 as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (integer: `com.amazonaws.ec2#Integer`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_total_unavailable_capacity(var_5);
            }
            ,
            s if s.matches("capacityReservationStatusSet") /* CapacityReservationStatuses com.amazonaws.ec2#CapacityBlockStatus$CapacityReservationStatuses */ =>  {
                let var_6 =
                    Some(
                        crate::protocol_serde::shape_capacity_reservation_status_set::de_capacity_reservation_status_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_capacity_reservation_statuses(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
