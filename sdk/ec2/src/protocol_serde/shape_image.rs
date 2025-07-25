// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::needless_question_mark)]
pub fn de_image(
    decoder: &mut ::aws_smithy_xml::decode::ScopedDecoder,
) -> ::std::result::Result<crate::types::Image, ::aws_smithy_xml::decode::XmlDecodeError> {
    #[allow(unused_mut)]
    let mut builder = crate::types::Image::builder();
    while let Some(mut tag) = decoder.next_tag() {
        match tag.start_el() {
            s if s.matches("platformDetails") /* PlatformDetails com.amazonaws.ec2#Image$PlatformDetails */ =>  {
                let var_1 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_platform_details(var_1);
            }
            ,
            s if s.matches("usageOperation") /* UsageOperation com.amazonaws.ec2#Image$UsageOperation */ =>  {
                let var_2 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_usage_operation(var_2);
            }
            ,
            s if s.matches("blockDeviceMapping") /* BlockDeviceMappings com.amazonaws.ec2#Image$BlockDeviceMappings */ =>  {
                let var_3 =
                    Some(
                        crate::protocol_serde::shape_block_device_mapping_list::de_block_device_mapping_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_block_device_mappings(var_3);
            }
            ,
            s if s.matches("description") /* Description com.amazonaws.ec2#Image$Description */ =>  {
                let var_4 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_description(var_4);
            }
            ,
            s if s.matches("enaSupport") /* EnaSupport com.amazonaws.ec2#Image$EnaSupport */ =>  {
                let var_5 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_ena_support(var_5);
            }
            ,
            s if s.matches("hypervisor") /* Hypervisor com.amazonaws.ec2#Image$Hypervisor */ =>  {
                let var_6 =
                    Some(
                        Result::<crate::types::HypervisorType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::HypervisorType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_hypervisor(var_6);
            }
            ,
            s if s.matches("imageOwnerAlias") /* ImageOwnerAlias com.amazonaws.ec2#Image$ImageOwnerAlias */ =>  {
                let var_7 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_image_owner_alias(var_7);
            }
            ,
            s if s.matches("name") /* Name com.amazonaws.ec2#Image$Name */ =>  {
                let var_8 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_name(var_8);
            }
            ,
            s if s.matches("rootDeviceName") /* RootDeviceName com.amazonaws.ec2#Image$RootDeviceName */ =>  {
                let var_9 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_root_device_name(var_9);
            }
            ,
            s if s.matches("rootDeviceType") /* RootDeviceType com.amazonaws.ec2#Image$RootDeviceType */ =>  {
                let var_10 =
                    Some(
                        Result::<crate::types::DeviceType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::DeviceType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_root_device_type(var_10);
            }
            ,
            s if s.matches("sriovNetSupport") /* SriovNetSupport com.amazonaws.ec2#Image$SriovNetSupport */ =>  {
                let var_11 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_sriov_net_support(var_11);
            }
            ,
            s if s.matches("stateReason") /* StateReason com.amazonaws.ec2#Image$StateReason */ =>  {
                let var_12 =
                    Some(
                        crate::protocol_serde::shape_state_reason::de_state_reason(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_state_reason(var_12);
            }
            ,
            s if s.matches("tagSet") /* Tags com.amazonaws.ec2#Image$Tags */ =>  {
                let var_13 =
                    Some(
                        crate::protocol_serde::shape_tag_list::de_tag_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_tags(var_13);
            }
            ,
            s if s.matches("virtualizationType") /* VirtualizationType com.amazonaws.ec2#Image$VirtualizationType */ =>  {
                let var_14 =
                    Some(
                        Result::<crate::types::VirtualizationType, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::VirtualizationType::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_virtualization_type(var_14);
            }
            ,
            s if s.matches("bootMode") /* BootMode com.amazonaws.ec2#Image$BootMode */ =>  {
                let var_15 =
                    Some(
                        Result::<crate::types::BootModeValues, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::BootModeValues::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_boot_mode(var_15);
            }
            ,
            s if s.matches("tpmSupport") /* TpmSupport com.amazonaws.ec2#Image$TpmSupport */ =>  {
                let var_16 =
                    Some(
                        Result::<crate::types::TpmSupportValues, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::TpmSupportValues::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_tpm_support(var_16);
            }
            ,
            s if s.matches("deprecationTime") /* DeprecationTime com.amazonaws.ec2#Image$DeprecationTime */ =>  {
                let var_17 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_deprecation_time(var_17);
            }
            ,
            s if s.matches("imdsSupport") /* ImdsSupport com.amazonaws.ec2#Image$ImdsSupport */ =>  {
                let var_18 =
                    Some(
                        Result::<crate::types::ImdsSupportValues, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ImdsSupportValues::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_imds_support(var_18);
            }
            ,
            s if s.matches("sourceInstanceId") /* SourceInstanceId com.amazonaws.ec2#Image$SourceInstanceId */ =>  {
                let var_19 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_source_instance_id(var_19);
            }
            ,
            s if s.matches("deregistrationProtection") /* DeregistrationProtection com.amazonaws.ec2#Image$DeregistrationProtection */ =>  {
                let var_20 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_deregistration_protection(var_20);
            }
            ,
            s if s.matches("lastLaunchedTime") /* LastLaunchedTime com.amazonaws.ec2#Image$LastLaunchedTime */ =>  {
                let var_21 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_last_launched_time(var_21);
            }
            ,
            s if s.matches("imageAllowed") /* ImageAllowed com.amazonaws.ec2#Image$ImageAllowed */ =>  {
                let var_22 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_image_allowed(var_22);
            }
            ,
            s if s.matches("sourceImageId") /* SourceImageId com.amazonaws.ec2#Image$SourceImageId */ =>  {
                let var_23 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_source_image_id(var_23);
            }
            ,
            s if s.matches("sourceImageRegion") /* SourceImageRegion com.amazonaws.ec2#Image$SourceImageRegion */ =>  {
                let var_24 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_source_image_region(var_24);
            }
            ,
            s if s.matches("freeTierEligible") /* FreeTierEligible com.amazonaws.ec2#Image$FreeTierEligible */ =>  {
                let var_25 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_free_tier_eligible(var_25);
            }
            ,
            s if s.matches("imageId") /* ImageId com.amazonaws.ec2#Image$ImageId */ =>  {
                let var_26 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_image_id(var_26);
            }
            ,
            s if s.matches("imageLocation") /* ImageLocation com.amazonaws.ec2#Image$ImageLocation */ =>  {
                let var_27 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_image_location(var_27);
            }
            ,
            s if s.matches("imageState") /* State com.amazonaws.ec2#Image$State */ =>  {
                let var_28 =
                    Some(
                        Result::<crate::types::ImageState, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ImageState::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_state(var_28);
            }
            ,
            s if s.matches("imageOwnerId") /* OwnerId com.amazonaws.ec2#Image$OwnerId */ =>  {
                let var_29 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_owner_id(var_29);
            }
            ,
            s if s.matches("creationDate") /* CreationDate com.amazonaws.ec2#Image$CreationDate */ =>  {
                let var_30 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_creation_date(var_30);
            }
            ,
            s if s.matches("isPublic") /* Public com.amazonaws.ec2#Image$Public */ =>  {
                let var_31 =
                    Some(
                         {
                            <bool as ::aws_smithy_types::primitive::Parse>::parse_smithy_primitive(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                            .map_err(|_|::aws_smithy_xml::decode::XmlDecodeError::custom("expected (boolean: `com.amazonaws.ec2#Boolean`)"))
                        }
                        ?
                    )
                ;
                builder = builder.set_public(var_31);
            }
            ,
            s if s.matches("productCodes") /* ProductCodes com.amazonaws.ec2#Image$ProductCodes */ =>  {
                let var_32 =
                    Some(
                        crate::protocol_serde::shape_product_code_list::de_product_code_list(&mut tag)
                        ?
                    )
                ;
                builder = builder.set_product_codes(var_32);
            }
            ,
            s if s.matches("architecture") /* Architecture com.amazonaws.ec2#Image$Architecture */ =>  {
                let var_33 =
                    Some(
                        Result::<crate::types::ArchitectureValues, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ArchitectureValues::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_architecture(var_33);
            }
            ,
            s if s.matches("imageType") /* ImageType com.amazonaws.ec2#Image$ImageType */ =>  {
                let var_34 =
                    Some(
                        Result::<crate::types::ImageTypeValues, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::ImageTypeValues::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_image_type(var_34);
            }
            ,
            s if s.matches("kernelId") /* KernelId com.amazonaws.ec2#Image$KernelId */ =>  {
                let var_35 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_kernel_id(var_35);
            }
            ,
            s if s.matches("ramdiskId") /* RamdiskId com.amazonaws.ec2#Image$RamdiskId */ =>  {
                let var_36 =
                    Some(
                        Result::<::std::string::String, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            .into()
                        )
                        ?
                    )
                ;
                builder = builder.set_ramdisk_id(var_36);
            }
            ,
            s if s.matches("platform") /* Platform com.amazonaws.ec2#Image$Platform */ =>  {
                let var_37 =
                    Some(
                        Result::<crate::types::PlatformValues, ::aws_smithy_xml::decode::XmlDecodeError>::Ok(
                            crate::types::PlatformValues::from(
                                ::aws_smithy_xml::decode::try_data(&mut tag)?.as_ref()
                            )
                        )
                        ?
                    )
                ;
                builder = builder.set_platform(var_37);
            }
            ,
            _ => {}
        }
    }
    Ok(builder.build())
}
