use serde::de::{Deserialize, Deserializer};

pub struct Config {
    pub limit: u64,
}
impl ::serde::de::Deserialize for Config {
    fn deserialize<__D>(deserializer: &mut __D)
     -> ::std::result::Result<Config, __D::Error> where
     __D: ::serde::de::Deserializer {
        {
            #[allow(non_camel_case_types)]
            enum __Field { __field0, __ignore, }
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
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_str<E>(&mut self, value: &str)
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                "limit" => { Ok(__Field::__field0) }
                                _ => Ok(__Field::__ignore),
                            }
                        }
                        fn visit_bytes<E>(&mut self, value: &[u8])
                         -> ::std::result::Result<__Field, E> where
                         E: ::serde::de::Error {
                            match value {
                                b"limit" => { Ok(__Field::__field0) }
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
                  // FIXME
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
                        try!(visitor . end (  ));
                        Ok(Config{limit: __field0,})
                    }
                }
                #[inline]
                fn visit_map<__V>(&mut self, mut visitor: __V)
                 -> ::std::result::Result<Config, __V::Error> where
                 __V: ::serde::de::MapVisitor {
                    {
                        let mut __field0 = None;
                        while let Some(key) = try!(visitor . visit_key (  )) {
                            match key {
                                __Field::__field0 => {
                                    __field0 =
                                        Some(try!({
    struct __SerdeDeserializeWithStruct {
        value: u64,
    }
    impl ::serde::de::Deserialize for __SerdeDeserializeWithStruct {
        fn deserialize<D>(deserializer: &mut D) -> Result<Self, D::Error>
         where D: ::serde::de::Deserializer {
            let value = try!(deserialize_limit ( deserializer ));
            Ok(__SerdeDeserializeWithStruct{value: value,})
        }
    }
    let value: __SerdeDeserializeWithStruct =
        try!(visitor . visit_value (  ));
    Ok(value.value)
}));
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
                                match visitor.missing_field("limit") {
                                    ::std::result::Result::Ok(value) => value,
                                    ::std::result::Result::Err(value) =>
                                    return ::std::result::Result::Err(value),
                                },
                            };
                        try!(visitor . end (  ));
                        Ok(Config{limit: __field0,})
                    }
                }
            }
            const FIELDS: &'static [&'static str] = &["limit"];
            deserializer.deserialize_struct("Config", FIELDS,
                                            __Visitor::<__D>(::std::marker::PhantomData))
        }
    }
}
fn deserialize_limit<D>(d: &mut D) -> Result<u64, D::Error> where
 D: Deserializer {
    u64::deserialize(d)
}
