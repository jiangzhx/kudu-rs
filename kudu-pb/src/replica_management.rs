// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::CodedOutputStream;
use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct ReplicaManagementInfoPB {
    // message fields
    replacement_scheme: ::std::option::Option<ReplicaManagementInfoPB_ReplacementScheme>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReplicaManagementInfoPB {}

impl ReplicaManagementInfoPB {
    pub fn new() -> ReplicaManagementInfoPB {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReplicaManagementInfoPB {
        static mut instance: ::protobuf::lazy::Lazy<ReplicaManagementInfoPB> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReplicaManagementInfoPB,
        };
        unsafe {
            instance.get(|| {
                ReplicaManagementInfoPB {
                    replacement_scheme: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kudu.consensus.ReplicaManagementInfoPB.ReplacementScheme replacement_scheme = 1;

    pub fn clear_replacement_scheme(&mut self) {
        self.replacement_scheme = ::std::option::Option::None;
    }

    pub fn has_replacement_scheme(&self) -> bool {
        self.replacement_scheme.is_some()
    }

    // Param is passed by value, moved
    pub fn set_replacement_scheme(&mut self, v: ReplicaManagementInfoPB_ReplacementScheme) {
        self.replacement_scheme = ::std::option::Option::Some(v);
    }

    pub fn get_replacement_scheme(&self) -> ReplicaManagementInfoPB_ReplacementScheme {
        self.replacement_scheme.unwrap_or(ReplicaManagementInfoPB_ReplacementScheme::UNKNOWN)
    }
}

impl ::protobuf::Message for ReplicaManagementInfoPB {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.replacement_scheme = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in self.replacement_scheme.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, mut w: &mut ::std::io::Write) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.replacement_scheme {
            try!(w.write_enum(1, v.value()));
        };
        try!(w.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReplicaManagementInfoPB>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReplicaManagementInfoPB {
    fn new() -> ReplicaManagementInfoPB {
        ReplicaManagementInfoPB::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReplicaManagementInfoPB>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "replacement_scheme",
                    ReplicaManagementInfoPB::has_replacement_scheme,
                    ReplicaManagementInfoPB::get_replacement_scheme,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReplicaManagementInfoPB>(
                    "ReplicaManagementInfoPB",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReplicaManagementInfoPB {
    fn clear(&mut self) {
        self.clear_replacement_scheme();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReplicaManagementInfoPB {
    fn eq(&self, other: &ReplicaManagementInfoPB) -> bool {
        self.replacement_scheme == other.replacement_scheme &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReplicaManagementInfoPB {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ReplicaManagementInfoPB_ReplacementScheme {
    UNKNOWN = 999,
    EVICT_FIRST = 0,
    PREPARE_REPLACEMENT_BEFORE_EVICTION = 1,
}

impl ::protobuf::ProtobufEnum for ReplicaManagementInfoPB_ReplacementScheme {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ReplicaManagementInfoPB_ReplacementScheme> {
        match value {
            999 => ::std::option::Option::Some(ReplicaManagementInfoPB_ReplacementScheme::UNKNOWN),
            0 => ::std::option::Option::Some(ReplicaManagementInfoPB_ReplacementScheme::EVICT_FIRST),
            1 => ::std::option::Option::Some(ReplicaManagementInfoPB_ReplacementScheme::PREPARE_REPLACEMENT_BEFORE_EVICTION),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ReplicaManagementInfoPB_ReplacementScheme] = &[
            ReplicaManagementInfoPB_ReplacementScheme::UNKNOWN,
            ReplicaManagementInfoPB_ReplacementScheme::EVICT_FIRST,
            ReplicaManagementInfoPB_ReplacementScheme::PREPARE_REPLACEMENT_BEFORE_EVICTION,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ReplicaManagementInfoPB_ReplacementScheme>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ReplicaManagementInfoPB_ReplacementScheme", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ReplicaManagementInfoPB_ReplacementScheme {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x27, 0x6b, 0x75, 0x64, 0x75, 0x2f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73,
    0x2f, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x5f, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x6d,
    0x65, 0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0e, 0x6b, 0x75, 0x64, 0x75, 0x2e,
    0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x22, 0xe0, 0x01, 0x0a, 0x17, 0x52, 0x65,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x49,
    0x6e, 0x66, 0x6f, 0x50, 0x42, 0x12, 0x68, 0x0a, 0x12, 0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x39, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73,
    0x75, 0x73, 0x2e, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x4d, 0x61, 0x6e, 0x61, 0x67, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x50, 0x42, 0x2e, 0x52, 0x65, 0x70, 0x6c, 0x61,
    0x63, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x52, 0x11, 0x72, 0x65,
    0x70, 0x6c, 0x61, 0x63, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x22,
    0x5b, 0x0a, 0x11, 0x52, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x53, 0x63,
    0x68, 0x65, 0x6d, 0x65, 0x12, 0x0c, 0x0a, 0x07, 0x55, 0x4e, 0x4b, 0x4e, 0x4f, 0x57, 0x4e, 0x10,
    0xe7, 0x07, 0x12, 0x0f, 0x0a, 0x0b, 0x45, 0x56, 0x49, 0x43, 0x54, 0x5f, 0x46, 0x49, 0x52, 0x53,
    0x54, 0x10, 0x00, 0x12, 0x27, 0x0a, 0x23, 0x50, 0x52, 0x45, 0x50, 0x41, 0x52, 0x45, 0x5f, 0x52,
    0x45, 0x50, 0x4c, 0x41, 0x43, 0x45, 0x4d, 0x45, 0x4e, 0x54, 0x5f, 0x42, 0x45, 0x46, 0x4f, 0x52,
    0x45, 0x5f, 0x45, 0x56, 0x49, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x10, 0x01, 0x42, 0x1b, 0x0a, 0x19,
    0x6f, 0x72, 0x67, 0x2e, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2e, 0x6b, 0x75, 0x64, 0x75, 0x2e,
    0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x73, 0x75, 0x73, 0x4a, 0xb2, 0x0c, 0x0a, 0x06, 0x12, 0x04,
    0x11, 0x00, 0x28, 0x01, 0x0a, 0x8c, 0x06, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x11, 0x00, 0x12, 0x32,
    0x81, 0x06, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65, 0x20, 0x53, 0x6f, 0x66, 0x74, 0x77, 0x61,
    0x72, 0x65, 0x20, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x28, 0x41,
    0x53, 0x46, 0x29, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x6f, 0x6e, 0x65, 0x0a, 0x20, 0x6f,
    0x72, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74,
    0x6f, 0x72, 0x20, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x61, 0x67, 0x72, 0x65, 0x65,
    0x6d, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x20, 0x20, 0x53, 0x65, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x4e, 0x4f, 0x54, 0x49, 0x43, 0x45, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x0a, 0x20, 0x64, 0x69, 0x73,
    0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68,
    0x69, 0x73, 0x20, 0x77, 0x6f, 0x72, 0x6b, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x64, 0x64, 0x69,
    0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x0a, 0x20, 0x72, 0x65, 0x67, 0x61, 0x72, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x63, 0x6f,
    0x70, 0x79, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x6f, 0x77, 0x6e, 0x65, 0x72, 0x73, 0x68, 0x69,
    0x70, 0x2e, 0x20, 0x20, 0x54, 0x68, 0x65, 0x20, 0x41, 0x53, 0x46, 0x20, 0x6c, 0x69, 0x63, 0x65,
    0x6e, 0x73, 0x65, 0x73, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x0a, 0x20,
    0x74, 0x6f, 0x20, 0x79, 0x6f, 0x75, 0x20, 0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x41, 0x70, 0x61, 0x63, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2c,
    0x20, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x32, 0x2e, 0x30, 0x20, 0x28, 0x74, 0x68,
    0x65, 0x0a, 0x20, 0x22, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x22, 0x29, 0x3b, 0x20, 0x79,
    0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x75, 0x73, 0x65, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x65, 0x78, 0x63, 0x65, 0x70, 0x74, 0x20,
    0x69, 0x6e, 0x20, 0x63, 0x6f, 0x6d, 0x70, 0x6c, 0x69, 0x61, 0x6e, 0x63, 0x65, 0x0a, 0x20, 0x77,
    0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x2e,
    0x20, 0x20, 0x59, 0x6f, 0x75, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x6f, 0x62, 0x74, 0x61, 0x69, 0x6e,
    0x20, 0x61, 0x20, 0x63, 0x6f, 0x70, 0x79, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c,
    0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x61, 0x74, 0x0a, 0x0a, 0x20, 0x20, 0x20, 0x68, 0x74,
    0x74, 0x70, 0x3a, 0x2f, 0x2f, 0x77, 0x77, 0x77, 0x2e, 0x61, 0x70, 0x61, 0x63, 0x68, 0x65, 0x2e,
    0x6f, 0x72, 0x67, 0x2f, 0x6c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x73, 0x2f, 0x4c, 0x49, 0x43,
    0x45, 0x4e, 0x53, 0x45, 0x2d, 0x32, 0x2e, 0x30, 0x0a, 0x0a, 0x20, 0x55, 0x6e, 0x6c, 0x65, 0x73,
    0x73, 0x20, 0x72, 0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x20, 0x62, 0x79, 0x20, 0x61, 0x70,
    0x70, 0x6c, 0x69, 0x63, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x6c, 0x61, 0x77, 0x20, 0x6f, 0x72, 0x20,
    0x61, 0x67, 0x72, 0x65, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x20, 0x77, 0x72, 0x69,
    0x74, 0x69, 0x6e, 0x67, 0x2c, 0x0a, 0x20, 0x73, 0x6f, 0x66, 0x74, 0x77, 0x61, 0x72, 0x65, 0x20,
    0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x75, 0x6e, 0x64, 0x65,
    0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x69, 0x73,
    0x20, 0x64, 0x69, 0x73, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x6e, 0x20,
    0x61, 0x6e, 0x0a, 0x20, 0x22, 0x41, 0x53, 0x20, 0x49, 0x53, 0x22, 0x20, 0x42, 0x41, 0x53, 0x49,
    0x53, 0x2c, 0x20, 0x57, 0x49, 0x54, 0x48, 0x4f, 0x55, 0x54, 0x20, 0x57, 0x41, 0x52, 0x52, 0x41,
    0x4e, 0x54, 0x49, 0x45, 0x53, 0x20, 0x4f, 0x52, 0x20, 0x43, 0x4f, 0x4e, 0x44, 0x49, 0x54, 0x49,
    0x4f, 0x4e, 0x53, 0x20, 0x4f, 0x46, 0x20, 0x41, 0x4e, 0x59, 0x0a, 0x20, 0x4b, 0x49, 0x4e, 0x44,
    0x2c, 0x20, 0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x65, 0x78, 0x70, 0x72, 0x65, 0x73, 0x73,
    0x20, 0x6f, 0x72, 0x20, 0x69, 0x6d, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x2e, 0x20, 0x20, 0x53, 0x65,
    0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73, 0x65, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x74, 0x68, 0x65, 0x0a, 0x20, 0x73, 0x70, 0x65, 0x63, 0x69, 0x66, 0x69, 0x63, 0x20,
    0x6c, 0x61, 0x6e, 0x67, 0x75, 0x61, 0x67, 0x65, 0x20, 0x67, 0x6f, 0x76, 0x65, 0x72, 0x6e, 0x69,
    0x6e, 0x67, 0x20, 0x70, 0x65, 0x72, 0x6d, 0x69, 0x73, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x0a, 0x20,
    0x75, 0x6e, 0x64, 0x65, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x69, 0x63, 0x65, 0x6e, 0x73,
    0x65, 0x2e, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x12, 0x08, 0x16, 0x0a, 0x08, 0x0a,
    0x01, 0x08, 0x12, 0x03, 0x14, 0x00, 0x32, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x14, 0x00, 0x32, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x14,
    0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x14, 0x07,
    0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x07,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12, 0x03, 0x14, 0x16, 0x31, 0x0a,
    0x4a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x17, 0x00, 0x28, 0x01, 0x1a, 0x3e, 0x20, 0x43, 0x6f,
    0x6d, 0x6d, 0x75, 0x6e, 0x69, 0x63, 0x61, 0x74, 0x65, 0x73, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69,
    0x63, 0x61, 0x20, 0x6d, 0x61, 0x6e, 0x61, 0x67, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6e,
    0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x62, 0x65, 0x74, 0x77, 0x65, 0x65,
    0x6e, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x17, 0x08, 0x1f, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x00, 0x04, 0x00, 0x12,
    0x04, 0x19, 0x02, 0x23, 0x03, 0x1a, 0x1e, 0x20, 0x52, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x20,
    0x72, 0x65, 0x70, 0x6c, 0x61, 0x63, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x63, 0x68, 0x65,
    0x6d, 0x65, 0x73, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x19, 0x07, 0x18, 0x0a, 0x0d, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x1a,
    0x04, 0x12, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a,
    0x04, 0x0b, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1a,
    0x0e, 0x11, 0x0a, 0x6e, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x04,
    0x14, 0x1a, 0x5f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x72,
    0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x65, 0x76, 0x69, 0x63, 0x74, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61,
    0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x0a, 0x20, 0x76, 0x6f, 0x74, 0x65, 0x72, 0x20,
    0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x69, 0x73, 0x20, 0x61, 0x64, 0x64, 0x65, 0x64,
    0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e,
    0x04, 0x0f, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1e,
    0x12, 0x13, 0x0a, 0x9d, 0x01, 0x0a, 0x06, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x22,
    0x04, 0x2c, 0x1a, 0x8d, 0x01, 0x20, 0x41, 0x64, 0x64, 0x20, 0x61, 0x20, 0x6e, 0x65, 0x77, 0x20,
    0x6e, 0x6f, 0x6e, 0x2d, 0x76, 0x6f, 0x74, 0x65, 0x72, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63,
    0x61, 0x2c, 0x20, 0x70, 0x72, 0x6f, 0x6d, 0x6f, 0x74, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x65, 0x70, 0x6c, 0x69, 0x63, 0x61, 0x20, 0x74, 0x6f, 0x20, 0x76, 0x6f, 0x74, 0x65, 0x72, 0x20,
    0x6f, 0x6e, 0x63, 0x65, 0x20, 0x69, 0x74, 0x0a, 0x20, 0x63, 0x61, 0x75, 0x67, 0x68, 0x74, 0x20,
    0x75, 0x70, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x61, 0x66, 0x74,
    0x65, 0x72, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x65, 0x76, 0x69, 0x63, 0x74, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x20, 0x72, 0x65, 0x70, 0x6c, 0x69, 0x63, 0x61,
    0x2e, 0x0a, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x22,
    0x04, 0x27, 0x0a, 0x0e, 0x0a, 0x07, 0x04, 0x00, 0x04, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x22,
    0x2a, 0x2b, 0x0a, 0x72, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x27, 0x02, 0x34, 0x1a,
    0x65, 0x20, 0x55, 0x73, 0x69, 0x6e, 0x67, 0x20, 0x27, 0x6f, 0x70, 0x74, 0x69, 0x6f, 0x6e, 0x61,
    0x6c, 0x27, 0x20, 0x69, 0x6e, 0x73, 0x74, 0x65, 0x61, 0x64, 0x20, 0x6f, 0x66, 0x20, 0x27, 0x72,
    0x65, 0x71, 0x75, 0x69, 0x72, 0x65, 0x64, 0x27, 0x20, 0x62, 0x65, 0x63, 0x61, 0x75, 0x73, 0x65,
    0x20, 0x61, 0x74, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x20, 0x70, 0x6f, 0x69, 0x6e, 0x74, 0x20, 0x77,
    0x65, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x64, 0x65, 0x63, 0x69, 0x64, 0x65, 0x0a, 0x20, 0x74, 0x6f,
    0x20, 0x6f, 0x62, 0x73, 0x6f, 0x6c, 0x65, 0x74, 0x65, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x66,
    0x69, 0x65, 0x6c, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x27, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x27,
    0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x27, 0x1d, 0x2f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x27, 0x32, 0x33,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
