// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_node_snapshot(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::NodeSnapshot, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::NodeSnapshot::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("CacheClusterId") /* CacheClusterId com.amazonaws.elasticache#NodeSnapshot$CacheClusterId */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cache_cluster_id(var_1);
            }
            ,
            s if s.matches("NodeGroupId") /* NodeGroupId com.amazonaws.elasticache#NodeSnapshot$NodeGroupId */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_node_group_id(var_2);
            }
            ,
            s if s.matches("CacheNodeId") /* CacheNodeId com.amazonaws.elasticache#NodeSnapshot$CacheNodeId */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cache_node_id(var_3);
            }
            ,
            s if s.matches("NodeGroupConfiguration") /* NodeGroupConfiguration com.amazonaws.elasticache#NodeSnapshot$NodeGroupConfiguration */ =>  {
                let var_4 =
                    Some(
                        crate::protocol_serde::shape_node_group_configuration::de_node_group_configuration(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_node_group_configuration(var_4);
            }
            ,
            s if s.matches("CacheSize") /* CacheSize com.amazonaws.elasticache#NodeSnapshot$CacheSize */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_cache_size(var_5);
            }
            ,
            s if s.matches("CacheNodeCreateTime") /* CacheNodeCreateTime com.amazonaws.elasticache#NodeSnapshot$CacheNodeCreateTime */ =>  {
                let var_6 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.elasticache#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_cache_node_create_time(var_6);
            }
            ,
            s if s.matches("SnapshotCreateTime") /* SnapshotCreateTime com.amazonaws.elasticache#NodeSnapshot$SnapshotCreateTime */ =>  {
                let var_7 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.elasticache#TStamp`)"))
                        ?
                    )
                ;
                builder = builder.set_snapshot_create_time(var_7);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
