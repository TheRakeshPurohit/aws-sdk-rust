// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_byoip_cidr(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::ByoipCidr, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::ByoipCidr::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("cidr") /* Cidr com.amazonaws.ec2#ByoipCidr$Cidr */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cidr(var_1);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#ByoipCidr$Description */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_2);
            }
            ,
            s if s.matches("asnAssociationSet") /* AsnAssociations com.amazonaws.ec2#ByoipCidr$AsnAssociations */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_asn_association_set::de_asn_association_set(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_asn_associations(var_3);
            }
            ,
            s if s.matches("statusMessage") /* StatusMessage com.amazonaws.ec2#ByoipCidr$StatusMessage */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_status_message(var_4);
            }
            ,
            s if s.matches("state") /* State com.amazonaws.ec2#ByoipCidr$State */ =>  {
                let var_5 =
                    Some(
                        Result::<crate::types::ByoipCidrState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ByoipCidrState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_5);
            }
            ,
            s if s.matches("networkBorderGroup") /* NetworkBorderGroup com.amazonaws.ec2#ByoipCidr$NetworkBorderGroup */ =>  {
                let var_6 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_network_border_group(var_6);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
