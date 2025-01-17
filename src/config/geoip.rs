// This file is generated by rust-protobuf 2.26.1. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `src/config/geoip.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_26_1;

#[derive(PartialEq,Clone,Default)]
pub struct CIDR {
    // message fields
    pub ip: ::bytes::Bytes,
    pub prefix: u32,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a CIDR {
    fn default() -> &'a CIDR {
        <CIDR as ::protobuf::Message>::default_instance()
    }
}

impl CIDR {
    pub fn new() -> CIDR {
        ::std::default::Default::default()
    }

    // bytes ip = 1;


    pub fn get_ip(&self) -> &[u8] {
        &self.ip
    }
    pub fn clear_ip(&mut self) {
        self.ip.clear();
    }

    // Param is passed by value, moved
    pub fn set_ip(&mut self, v: ::bytes::Bytes) {
        self.ip = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ip(&mut self) -> &mut ::bytes::Bytes {
        &mut self.ip
    }

    // Take field
    pub fn take_ip(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.ip, ::bytes::Bytes::new())
    }

    // uint32 prefix = 2;


    pub fn get_prefix(&self) -> u32 {
        self.prefix
    }
    pub fn clear_prefix(&mut self) {
        self.prefix = 0;
    }

    // Param is passed by value, moved
    pub fn set_prefix(&mut self, v: u32) {
        self.prefix = v;
    }
}

impl ::protobuf::Message for CIDR {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.ip)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.prefix = tmp;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.ip.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.ip);
        }
        if self.prefix != 0 {
            my_size += ::protobuf::rt::value_size(2, self.prefix, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.ip.is_empty() {
            os.write_bytes(1, &self.ip)?;
        }
        if self.prefix != 0 {
            os.write_uint32(2, self.prefix)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> CIDR {
        CIDR::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                "ip",
                |m: &CIDR| { &m.ip },
                |m: &mut CIDR| { &mut m.ip },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                "prefix",
                |m: &CIDR| { &m.prefix },
                |m: &mut CIDR| { &mut m.prefix },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<CIDR>(
                "CIDR",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static CIDR {
        static instance: ::protobuf::rt::LazyV2<CIDR> = ::protobuf::rt::LazyV2::INIT;
        instance.get(CIDR::new)
    }
}

impl ::protobuf::Clear for CIDR {
    fn clear(&mut self) {
        self.ip.clear();
        self.prefix = 0;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CIDR {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CIDR {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GeoIP {
    // message fields
    pub country_code: ::std::string::String,
    pub cidr: ::protobuf::RepeatedField<CIDR>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GeoIP {
    fn default() -> &'a GeoIP {
        <GeoIP as ::protobuf::Message>::default_instance()
    }
}

impl GeoIP {
    pub fn new() -> GeoIP {
        ::std::default::Default::default()
    }

    // string country_code = 1;


    pub fn get_country_code(&self) -> &str {
        &self.country_code
    }
    pub fn clear_country_code(&mut self) {
        self.country_code.clear();
    }

    // Param is passed by value, moved
    pub fn set_country_code(&mut self, v: ::std::string::String) {
        self.country_code = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_country_code(&mut self) -> &mut ::std::string::String {
        &mut self.country_code
    }

    // Take field
    pub fn take_country_code(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.country_code, ::std::string::String::new())
    }

    // repeated .CIDR cidr = 2;


    pub fn get_cidr(&self) -> &[CIDR] {
        &self.cidr
    }
    pub fn clear_cidr(&mut self) {
        self.cidr.clear();
    }

    // Param is passed by value, moved
    pub fn set_cidr(&mut self, v: ::protobuf::RepeatedField<CIDR>) {
        self.cidr = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cidr(&mut self) -> &mut ::protobuf::RepeatedField<CIDR> {
        &mut self.cidr
    }

    // Take field
    pub fn take_cidr(&mut self) -> ::protobuf::RepeatedField<CIDR> {
        ::std::mem::replace(&mut self.cidr, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for GeoIP {
    fn is_initialized(&self) -> bool {
        for v in &self.cidr {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.country_code)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cidr)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.country_code.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.country_code);
        }
        for value in &self.cidr {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.country_code.is_empty() {
            os.write_string(1, &self.country_code)?;
        }
        for v in &self.cidr {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GeoIP {
        GeoIP::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                "country_code",
                |m: &GeoIP| { &m.country_code },
                |m: &mut GeoIP| { &mut m.country_code },
            ));
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CIDR>>(
                "cidr",
                |m: &GeoIP| { &m.cidr },
                |m: &mut GeoIP| { &mut m.cidr },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GeoIP>(
                "GeoIP",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GeoIP {
        static instance: ::protobuf::rt::LazyV2<GeoIP> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GeoIP::new)
    }
}

impl ::protobuf::Clear for GeoIP {
    fn clear(&mut self) {
        self.country_code.clear();
        self.cidr.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GeoIP {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GeoIP {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GeoIPList {
    // message fields
    pub entry: ::protobuf::RepeatedField<GeoIP>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a GeoIPList {
    fn default() -> &'a GeoIPList {
        <GeoIPList as ::protobuf::Message>::default_instance()
    }
}

impl GeoIPList {
    pub fn new() -> GeoIPList {
        ::std::default::Default::default()
    }

    // repeated .GeoIP entry = 1;


    pub fn get_entry(&self) -> &[GeoIP] {
        &self.entry
    }
    pub fn clear_entry(&mut self) {
        self.entry.clear();
    }

    // Param is passed by value, moved
    pub fn set_entry(&mut self, v: ::protobuf::RepeatedField<GeoIP>) {
        self.entry = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entry(&mut self) -> &mut ::protobuf::RepeatedField<GeoIP> {
        &mut self.entry
    }

    // Take field
    pub fn take_entry(&mut self) -> ::protobuf::RepeatedField<GeoIP> {
        ::std::mem::replace(&mut self.entry, ::protobuf::RepeatedField::new())
    }
}

impl ::protobuf::Message for GeoIPList {
    fn is_initialized(&self) -> bool {
        for v in &self.entry {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entry)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.entry {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entry {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> GeoIPList {
        GeoIPList::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GeoIP>>(
                "entry",
                |m: &GeoIPList| { &m.entry },
                |m: &mut GeoIPList| { &mut m.entry },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<GeoIPList>(
                "GeoIPList",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static GeoIPList {
        static instance: ::protobuf::rt::LazyV2<GeoIPList> = ::protobuf::rt::LazyV2::INIT;
        instance.get(GeoIPList::new)
    }
}

impl ::protobuf::Clear for GeoIPList {
    fn clear(&mut self) {
        self.entry.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GeoIPList {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GeoIPList {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x16src/config/geoip.proto\".\n\x04CIDR\x12\x0e\n\x02ip\x18\x01\x20\
    \x01(\x0cR\x02ip\x12\x16\n\x06prefix\x18\x02\x20\x01(\rR\x06prefix\"E\n\
    \x05GeoIP\x12!\n\x0ccountry_code\x18\x01\x20\x01(\tR\x0bcountryCode\x12\
    \x19\n\x04cidr\x18\x02\x20\x03(\x0b2\x05.CIDRR\x04cidr\")\n\tGeoIPList\
    \x12\x1c\n\x05entry\x18\x01\x20\x03(\x0b2\x06.GeoIPR\x05entryb\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
