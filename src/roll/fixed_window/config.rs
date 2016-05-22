pub struct Config {
    pub pattern: String,
    pub base: Option<u32>,
    pub limit: u32,
}
#[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
const _IMPL_DESERIALIZE_FOR_Config: () =
    {
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::de::Deserialize for Config {
            fn deserialize<__D>(deserializer: &mut __D)
             -> ::std::result::Result<Config, __D::Error> where
             __D: _serde::de::Deserializer {
                {
                    #[allow(non_camel_case_types)]
                    enum __Field { __field0, __field1, __field2, }
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
                                        _ => {
                                            Err(_serde::de::Error::invalid_value("expected a field"))
                                        }
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "pattern" => { Ok(__Field::__field0) }
                                        "base" => { Ok(__Field::__field1) }
                                        "limit" => { Ok(__Field::__field2) }
                                        _ =>
                                        Err(_serde::de::Error::unknown_field(value)),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"pattern" => {
                                            Ok(__Field::__field0)
                                        }
                                        b"base" => { Ok(__Field::__field1) }
                                        b"limit" => { Ok(__Field::__field2) }
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
                    struct __Visitor<__D: _serde::de::Deserializer>(::std::marker::PhantomData<__D>);
                    impl <__D: _serde::de::Deserializer> _serde::de::Visitor
                     for __Visitor<__D> {
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
                                            return Err(_serde::de::Error::end_of_stream());
                                        }
                                    };
                                let __field1 =
                                    match try!(visitor . visit :: <
                                               Option<u32> > (  )) {
                                        Some(value) => { value }
                                        None => {
                                            return Err(_serde::de::Error::end_of_stream());
                                        }
                                    };
                                let __field2 =
                                    match try!(visitor . visit :: < u32 > (
                                               )) {
                                        Some(value) => { value }
                                        None => {
                                            return Err(_serde::de::Error::end_of_stream());
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(Config{pattern: __field0,
                                          base: __field1,
                                          limit: __field2,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Config, __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<String> = None;
                                let mut __field1: Option<Option<u32>> = None;
                                let mut __field2: Option<u32> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("pattern"));
                                            }
                                            __field0 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          String > (  )));
                                        }
                                        __Field::__field1 => {
                                            if __field1.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("base"));
                                            }
                                            __field1 =
                                                Some(try!(visitor .
                                                          visit_value :: <
                                                          Option<u32> > (
                                                          )));
                                        }
                                        __Field::__field2 => {
                                            if __field2.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("limit"));
                                            }
                                            __field2 =
                                                Some(try!(visitor .
                                                          visit_value :: < u32
                                                          > (  )));
                                        }
                                    }
                                }
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        try!(visitor . missing_field (
                                             "pattern" )),
                                    };
                                let __field1 =
                                    match __field1 {
                                        Some(__field1) => __field1,
                                        None =>
                                        try!(visitor . missing_field ( "base"
                                             )),
                                    };
                                let __field2 =
                                    match __field2 {
                                        Some(__field2) => __field2,
                                        None =>
                                        try!(visitor . missing_field ( "limit"
                                             )),
                                    };
                                try!(visitor . end (  ));
                                Ok(Config{pattern: __field0,
                                          base: __field1,
                                          limit: __field2,})
                            }
                        }
                    }
                    const FIELDS: &'static [&'static str] =
                        &["pattern", "base", "limit"];
                    deserializer.deserialize_struct("Config", FIELDS,
                                                    __Visitor::<__D>(::std::marker::PhantomData))
                }
            }
        }
    };
