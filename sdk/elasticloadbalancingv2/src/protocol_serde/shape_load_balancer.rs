// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_load_balancer(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::LoadBalancer, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::LoadBalancer::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("LoadBalancerArn") /* LoadBalancerArn com.amazonaws.elasticloadbalancingv2#LoadBalancer$LoadBalancerArn */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_load_balancer_arn(var_1);
            }
            ,
            s if s.matches("DNSName") /* DNSName com.amazonaws.elasticloadbalancingv2#LoadBalancer$DNSName */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_dns_name(var_2);
            }
            ,
            s if s.matches("CanonicalHostedZoneId") /* CanonicalHostedZoneId com.amazonaws.elasticloadbalancingv2#LoadBalancer$CanonicalHostedZoneId */ =>  {
                let var_3 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_canonical_hosted_zone_id(var_3);
            }
            ,
            s if s.matches("CreatedTime") /* CreatedTime com.amazonaws.elasticloadbalancingv2#LoadBalancer$CreatedTime */ =>  {
                let var_4 =
                    Some(
                        ::aws_smithy_types::DateTime::from_str(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            , ::aws_smithy_types::date_time::Format::DateTimeWithOffset
                        )
                        .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (timestamp: `com.amazonaws.elasticloadbalancingv2#CreatedTime`)"))
                        ?
                    )
                ;
                builder = builder.set_created_time(var_4);
            }
            ,
            s if s.matches("LoadBalancerName") /* LoadBalancerName com.amazonaws.elasticloadbalancingv2#LoadBalancer$LoadBalancerName */ =>  {
                let var_5 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_load_balancer_name(var_5);
            }
            ,
            s if s.matches("Scheme") /* Scheme com.amazonaws.elasticloadbalancingv2#LoadBalancer$Scheme */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::types::LoadBalancerSchemeEnum, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::LoadBalancerSchemeEnum::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_scheme(var_6);
            }
            ,
            s if s.matches("VpcId") /* VpcId com.amazonaws.elasticloadbalancingv2#LoadBalancer$VpcId */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_vpc_id(var_7);
            }
            ,
            s if s.matches("State") /* State com.amazonaws.elasticloadbalancingv2#LoadBalancer$State */ =>  {
                let var_8 =
                    Some(
                        crate::protocol_serde::shape_load_balancer_state::de_load_balancer_state(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_state(var_8);
            }
            ,
            s if s.matches("Type") /* Type com.amazonaws.elasticloadbalancingv2#LoadBalancer$Type */ =>  {
                let var_9 =
                    Some(
                        Result::<crate::types::LoadBalancerTypeEnum, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::LoadBalancerTypeEnum::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_type(var_9);
            }
            ,
            s if s.matches("AvailabilityZones") /* AvailabilityZones com.amazonaws.elasticloadbalancingv2#LoadBalancer$AvailabilityZones */ =>  {
                let var_10 =
                    Some(
                        crate::protocol_serde::shape_availability_zones::de_availability_zones(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_availability_zones(var_10);
            }
            ,
            s if s.matches("SecurityGroups") /* SecurityGroups com.amazonaws.elasticloadbalancingv2#LoadBalancer$SecurityGroups */ =>  {
                let var_11 =
                    Some(
                        crate::protocol_serde::shape_security_groups::de_security_groups(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_security_groups(var_11);
            }
            ,
            s if s.matches("IpAddressType") /* IpAddressType com.amazonaws.elasticloadbalancingv2#LoadBalancer$IpAddressType */ =>  {
                let var_12 =
                    Some(
                        Result::<crate::types::IpAddressType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::IpAddressType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_ip_address_type(var_12);
            }
            ,
            s if s.matches("CustomerOwnedIpv4Pool") /* CustomerOwnedIpv4Pool com.amazonaws.elasticloadbalancingv2#LoadBalancer$CustomerOwnedIpv4Pool */ =>  {
                let var_13 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_customer_owned_ipv4_pool(var_13);
            }
            ,
            s if s.matches("EnforceSecurityGroupInboundRulesOnPrivateLinkTraffic") /* EnforceSecurityGroupInboundRulesOnPrivateLinkTraffic com.amazonaws.elasticloadbalancingv2#LoadBalancer$EnforceSecurityGroupInboundRulesOnPrivateLinkTraffic */ =>  {
                let var_14 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_enforce_security_group_inbound_rules_on_private_link_traffic(var_14);
            }
            ,
            s if s.matches("EnablePrefixForIpv6SourceNat") /* EnablePrefixForIpv6SourceNat com.amazonaws.elasticloadbalancingv2#LoadBalancer$EnablePrefixForIpv6SourceNat */ =>  {
                let var_15 =
                    Some(
                        Result::<crate::types::EnablePrefixForIpv6SourceNatEnum, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::EnablePrefixForIpv6SourceNatEnum::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_enable_prefix_for_ipv6_source_nat(var_15);
            }
            ,
            s if s.matches("IpamPools") /* IpamPools com.amazonaws.elasticloadbalancingv2#LoadBalancer$IpamPools */ =>  {
                let var_16 =
                    Some(
                        crate::protocol_serde::shape_ipam_pools::de_ipam_pools(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_ipam_pools(var_16);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
