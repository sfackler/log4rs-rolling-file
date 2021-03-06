use log4rs::file::raw::Encoder;
use serde::de::{Deserialize, Deserializer};
use serde_value::Value;
use std::collections::BTreeMap;

pub struct Config {
    pub path: String,
    pub append: Option<bool>,
    pub encoder: Option<Encoder>,
    pub policy: Policy,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Config: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for Config where



         String: _serde::de::Deserialize,
         Option<bool>: _serde::de::Deserialize,
         Option<Encoder>: _serde::de::Deserialize,
         Policy: _serde::de::Deserialize {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<Config, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, __field3, }
                    impl _serde::de::Deserialize for __Field {
                        #[inline]
                        fn deserialize<__D>(deserializer: &mut __D)
                         -> ::std::result::Result<__Field, __D::Error> where
                         __D: _serde::de::Deserializer {
                            struct __FieldVisitor<__D> {
                                phantom: ::std::marker::PhantomData<__D>,
                            }
                            impl <__D> _serde::de::Visitor for
                             __FieldVisitor<__D> where
                             __D: _serde::de::Deserializer {
                                type
                                Value
                                =
                                __Field;
                                fn visit_usize<__E>(&mut self, value: usize)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        0usize => { Ok(__Field::__field0) }
                                        1usize => { Ok(__Field::__field1) }
                                        2usize => { Ok(__Field::__field2) }
                                        3usize => { Ok(__Field::__field3) }
                                        _ => {
                                            Err(_serde::de::Error::invalid_value("expected a field"))
                                        }
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "path" => { Ok(__Field::__field0) }
                                        "append" => { Ok(__Field::__field1) }
                                        "encoder" => { Ok(__Field::__field2) }
                                        "policy" => { Ok(__Field::__field3) }
                                        _ =>
                                        Err(_serde::de::Error::unknown_field(value)),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"path" => { Ok(__Field::__field0) }
                                        b"append" => { Ok(__Field::__field1) }
                                        b"encoder" => {
                                            Ok(__Field::__field2)
                                        }
                                        b"policy" => { Ok(__Field::__field3) }
                                        _ => {
                                            let value =
                                                ::std::string::String::from_utf8_lossy(value);
                                            Err(_serde::de::Error::unknown_field(&value))
                                        }
                                    }
                                }
                            }
                            deserializer.deserialize_struct_field(__FieldVisitor::<__D>{phantom:
                                                                                            ::std::marker::PhantomData,})
                        }
                    }
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>)
                           where String: _serde::de::Deserialize,
                           Option<bool>: _serde::de::Deserialize,
                           Option<Encoder>: _serde::de::Deserialize,
                           Policy: _serde::de::Deserialize;
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> where String: _serde::de::Deserialize,
                     Option<bool>: _serde::de::Deserialize,
                     Option<Encoder>: _serde::de::Deserialize,
                     Policy: _serde::de::Deserialize {
                        type
                        Value
                        =
                        Config;
                        #[inline]
                        fn visit_seq<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Config, __V::Error> where
                         __V: _serde::de::SeqVisitor {
                            {
                                let __field0 =
                                    match try!(visitor . visit :: < String > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<bool> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(1usize));
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: <
                                               Option<Encoder> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(2usize));
                                        }
                                    };
                                let __field3 =
                                    match try!(visitor . visit :: < Policy > (
                                                )) {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(3usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(Config{path: __field0,
                                          append: __field1,
                                          encoder: __field2,
                                          policy: __field3,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Config, __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<String> = None;
                                let mut __field1: Option<Option<bool>> = None;
                                let mut __field2: Option<Option<Encoder>> =
                                    None;
                                let mut __field3: Option<Policy> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("path"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          String > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("append"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<bool> > (
                                                          )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("encoder"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<Encoder> > (
                                                          )));
                                        }
                                        __Field::__field3 => {
                                            if __field3.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("policy"));
                                            }
                                            __field3 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Policy > (  )));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field ( "path"
                                             )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field (
                                             "append" )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field (
                                             "encoder" )),
                                    };
                                let __field3 =
                                    match __field3 {
                                        Some(__field3) => __field3,
                                        None =>
                                        try!(visitor . missing_field (
                                             "policy" )),
                                    };
                                Ok(Config{path: __field0,
                                          append: __field1,
                                          encoder: __field2,
                                          policy: __field3,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["path", "append", "encoder", "policy"];
                    deserializer.deserialize_struct("Config", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
pub struct Policy {
    pub kind: String,
    pub config: Value,
}
impl Deserialize for Policy {
    fn deserialize<D>(d: &mut D) -> Result<Policy, D::Error> where
     D: Deserializer {
        let mut map =
            try!(BTreeMap :: < Value , Value > :: deserialize ( d ));
        let kind =
            match map.remove(&Value::String("kind".to_owned())) {
                Some(kind) =>
                try!(kind . deserialize_into (  ) . map_err (
                     | e | e . to_error (  ) )),
                None => "compound".to_owned(),
            };
        Ok(Policy{kind: kind, config: Value::Map(map),})
    }
}
