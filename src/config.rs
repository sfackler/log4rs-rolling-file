use log4rs::file::raw::Encoder;
use serde::de::{self, Deserialize, Deserializer};
use serde_value::Value;
use std::collections::BTreeMap;

pub struct Config {
    pub path: String,
    pub append: Option<bool>,
    pub encoder: Option<Encoder>,
    pub trigger: Trigger,
    pub roller: Roller,
}
impl ::serde::de::Deserialize for Config {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<Config, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field {
                __field0,
                __field1,
                __field2,
                __field3,
                __field4,
                __ignore,
            }
            impl ::serde::de::Deserialize for __Field {
                #[inline]
                fn deserialize<D>(deserializer: &mut D)
                 -> ::std::result::Result<__Field, D::Error> where
                 D: ::serde::de::Deserializer {
                    use std::marker::PhantomData;
                    struct __FieldVisitor<D> {
                        phantom: PhantomData<D>,
                    }
                    impl <__D> ::serde::de::Visitor for __FieldVisitor<__D>
                     where __D: ::serde::de::Deserializer {
                        type
                        Value
                        =
                        __Field;
                        fn visit_usize<E>(&mut self, value: usize)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                0usize => { Ok(__Field::__field0) }
                                1usize => { Ok(__Field::__field1) }
                                2usize => { Ok(__Field::__field2) }
                                3usize => { Ok(__Field::__field3) }
                                4usize => { Ok(__Field::__field4) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "path" => { Ok(__Field::__field0) }
                                "append" => { Ok(__Field::__field1) }
                                "encoder" => { Ok(__Field::__field2) }
                                "trigger" => { Ok(__Field::__field3) }
                                "roller" => { Ok(__Field::__field4) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                b"path" => { Ok(__Field::__field0) }
                                b"append" => { Ok(__Field::__field1) }
                                b"encoder" => { Ok(__Field::__field2) }
                                b"trigger" => { Ok(__Field::__field3) }
                                b"roller" => { Ok(__Field::__field4) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                    }
                    deserializer.deserialize_struct_field(__FieldVisitor::<D>{phantom:
                                                                                  PhantomData,})
                }
            }
            struct __Visitor<__D: ::serde::de::Deserializer>(::std::marker::PhantomData<__D>);
            impl <__D: 







                  ::serde::de::Deserializer> ::serde::de::Visitor for
             __Visitor<__D> {
                type
                Value
                =
                Config;
                #[inline]
                fn visit_seq<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Config, __V::Error> where
                 __V: ::serde::de::SeqVisitor {
                    {
                        let __field0 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        let __field1 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        let __field2 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        let __field3 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        let __field4 =
                            match try!(visitor . visit (  )) {
                                Some(value) => { value }
                                None => {
                                    return Err(::serde::de::Error::end_of_stream());
                                }
                            };
                        try!(visitor . end (  ));
                        Ok(Config{path: __field0,
                                  append: __field1,
                                  encoder: __field2,
                                  trigger: __field3,
                                  roller: __field4,})
                    }
                }
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Config, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        let mut __field1 = None;
                        let mut __field2 = None;
                        let mut __field3 = None;
                        let mut __field4 = None;
                        while let Some(key) = try!(visitor . visit_key (  )) {
                            match key {
                                __Field::__field0 => {
                                    __field0 =
                                        Some(try!(visitor.visit_value()));
                                }
                                __Field::__field1 => {
                                    __field1 =
                                        Some(try!(visitor.visit_value()));
                                }
                                __Field::__field2 => {
                                    __field2 =
                                        Some(try!(visitor.visit_value()));
                                }
                                __Field::__field3 => {
                                    __field3 =
                                        Some(try!(visitor.visit_value()));
                                }
                                __Field::__field4 => {
                                    __field4 =
                                        Some(try!(visitor.visit_value()));
                                }
                                _ => {
                                    try!(visitor . visit_value:: < :: serde::
                                         de:: impls:: IgnoredAny > (  ));
                                }
                            }
                        }
                        let __field0 =
                            match __field0 {
                                Some(__field0) => __field0,
                                None =>
                                match visitor.missing_field("path") {
                                    ::std::result::Result::Ok(value) => value,
                                    ::std::result::Result::Err(value) =>
                                    return ::std::result::Result::Err(value),
                                },
                            };
                        let __field1 =
                            match __field1 {
                                Some(__field1) => __field1,
                                None =>
                                match visitor.missing_field("append") {
                                    ::std::result::Result::Ok(value) => value,
                                    ::std::result::Result::Err(value) =>
                                    return ::std::result::Result::Err(value),
                                },
                            };
                        let __field2 =
                            match __field2 {
                                Some(__field2) => __field2,
                                None =>
                                match visitor.missing_field("encoder") {
                                    ::std::result::Result::Ok(value) => value,
                                    ::std::result::Result::Err(value) =>
                                    return ::std::result::Result::Err(value),
                                },
                            };
                        let __field3 =
                            match __field3 {
                                Some(__field3) => __field3,
                                None =>
                                match visitor.missing_field("trigger") {
                                    ::std::result::Result::Ok(value) => value,
                                    ::std::result::Result::Err(value) =>
                                    return ::std::result::Result::Err(value),
                                },
                            };
                        let __field4 =
                            match __field4 {
                                Some(__field4) => __field4,
                                None =>
                                match visitor.missing_field("roller") {
                                    ::std::result::Result::Ok(value) => value,
                                    ::std::result::Result::Err(value) =>
                                    return ::std::result::Result::Err(value),
                                },
                            };
                        try!(visitor . end (  ));
                        Ok(Config{path: __field0,
                                  append: __field1,
                                  encoder: __field2,
                                  trigger: __field3,
                                  roller: __field4,})
                    }
                }
            }
            const FIELDS: &'static [&'static str] =
                &["path", "append", "encoder", "trigger", "roller"];
            deserializer.deserialize_struct("Config", FIELDS,
                                            __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
pub struct Trigger {
    pub kind: String,
    pub config: Value,
}
impl Deserialize for Trigger {
    fn deserialize<D>(d: &mut D) -> Result<Trigger, D::Error> where
     D: Deserializer {
        let mut map = try!(BTreeMap:: < Value , Value > :: deserialize ( d ));
        let kind =
            match map.remove(&Value::String("kind".to_owned())) {
                Some(kind) =>
                try!(kind . deserialize_into (  ) . map_err (
                     | e | e . to_error (  ) )),
                None => return Err(de::Error::missing_field("kind")),
            };
        Ok(Trigger{kind: kind, config: Value::Map(map),})
    }
}
pub struct Roller {
    pub kind: String,
    pub config: Value,
}
impl Deserialize for Roller {
    fn deserialize<D>(d: &mut D) -> Result<Roller, D::Error> where
     D: Deserializer {
        let mut map = try!(BTreeMap:: < Value , Value > :: deserialize ( d ));
        let kind =
            match map.remove(&Value::String("kind".to_owned())) {
                Some(kind) =>
                try!(kind . deserialize_into (  ) . map_err (
                     | e | e . to_error (  ) )),
                None => return Err(de::Error::missing_field("kind")),
            };
        Ok(Roller{kind: kind, config: Value::Map(map),})
    }
}
