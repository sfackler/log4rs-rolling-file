use serde::de::{Deserializer, Visitor, Error};
use std::ascii::AsciiExt;

pub struct Config {
    pub limit: u64,
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
                    enum __Field { __field0, }
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
                                        _ => {
                                            Err(_serde::de::Error::invalid_value("expected a field"))
                                        }
                                    }
                                }
                                fn visit_str<__E>(&mut self, value: &str)
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        "limit" => { Ok(__Field::__field0) }
                                        _ =>
                                        Err(_serde::de::Error::unknown_field(value)),
                                    }
                                }
                                fn visit_bytes<__E>(&mut self, value: &[u8])
                                 -> ::std::result::Result<__Field, __E> where
                                 __E: _serde::de::Error {
                                    match value {
                                        b"limit" => { Ok(__Field::__field0) }
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
                    impl <__D: 










                          _serde::de::Deserializer> _serde::de::Visitor for
                     __Visitor<__D> {
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
                                    match {
                                              struct __SerdeDeserializeWithStruct {
                                                  value: u64,
                                                  phantom: ::std::marker::PhantomData<Config>,
                                              }
                                              impl _serde::de::Deserialize for
                                               __SerdeDeserializeWithStruct {
                                                  fn deserialize<__D>(__d:
                                                                          &mut __D)
                                                   ->
                                                       ::std::result::Result<Self,
                                                                             __D::Error>
                                                   where
                                                   __D: _serde::de::Deserializer {
                                                      let value =
                                                          try!(deserialize_limit
                                                               ( __d ));
                                                      Ok(__SerdeDeserializeWithStruct{value:
                                                                                          value,
                                                                                      phantom:
                                                                                          ::std::marker::PhantomData,})
                                                  }
                                              }
                                              try!(visitor . visit :: <
                                                   __SerdeDeserializeWithStruct
                                                   > (
                                                   )).map(|wrap| wrap.value)
                                          } {
                                        Some(value) => { value }
                                        None => {
                                            try!(visitor . end (  ));
                                            return Err(_serde::de::Error::invalid_length(0usize));
                                        }
                                    };
                                try!(visitor . end (  ));
                                Ok(Config{limit: __field0,})
                            }
                        }
                        #[inline]
                        fn visit_map<__V>(&mut self, mut visitor: __V)
                         -> ::std::result::Result<Config, __V::Error> where
                         __V: _serde::de::MapVisitor {
                            {
                                let mut __field0: Option<u64> = None;
                                while let Some(key) =
                                          try!(visitor . visit_key :: <
                                               __Field > (  )) {
                                    match key {
                                        __Field::__field0 => {
                                            if __field0.is_some() {
                                                return Err(<__V::Error as
                                                               _serde::de::Error>::duplicate_field("limit"));
                                            }
                                            __field0 =
                                                Some(({
                                                          struct __SerdeDeserializeWithStruct {
                                                              value: u64,
                                                              phantom: ::std::marker::PhantomData<Config>,
                                                          }
                                                          impl _serde::de::Deserialize
                                                           for
                                                           __SerdeDeserializeWithStruct
                                                           {
                                                              fn deserialize<__D>(__d:
                                                                                      &mut __D)
                                                               ->
                                                                   ::std::result::Result<Self,
                                                                                         __D::Error>
                                                               where
                                                               __D: _serde::de::Deserializer {
                                                                  let value =
                                                                      try!(deserialize_limit
                                                                           (
                                                                           __d
                                                                           ));
                                                                  Ok(__SerdeDeserializeWithStruct{value:
                                                                                                      value,
                                                                                                  phantom:
                                                                                                      ::std::marker::PhantomData,})
                                                              }
                                                          }
                                                          try!(visitor .
                                                               visit_value ::
                                                               <
                                                               __SerdeDeserializeWithStruct
                                                               > (  )).value
                                                      }));
                                        }
                                    }
                                }
                                try!(visitor . end (  ));
                                let __field0 =
                                    match __field0 {
                                        Some(__field0) => __field0,
                                        None =>
                                        return Err(<__V::Error as
                                                       _serde::de::Error>::missing_field("limit")),
                                    };
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
    };
fn deserialize_limit<D>(d: &mut D) -> Result<u64, D::Error> where
 D: Deserializer {
    struct V;
    impl Visitor for V {
        type
        Value
        =
        u64;
        fn visit_u64<E>(&mut self, v: u64) -> Result<u64, E> where E: Error {
            Ok(v)
        }
        fn visit_i64<E>(&mut self, v: i64) -> Result<u64, E> where E: Error {
            if v < 0 { return Err(E::invalid_value("must be non-negative")); }
            Ok(v as u64)
        }
        fn visit_str<E>(&mut self, v: &str) -> Result<u64, E> where E: Error {
            let (number, unit) =
                match v.find(|c: char| !c.is_digit(10)) {
                    Some(n) => (v[..n].trim(), Some(v[n..].trim())),
                    None => (v.trim(), None),
                };
            let number =
                match number.parse::<u64>() {
                    Ok(n) => n,
                    Err(e) => return Err(E::invalid_value(&e.to_string())),
                };
            let unit =
                match unit { Some(u) => u, None => return Ok(number), };
            let number =
                if unit.eq_ignore_ascii_case("b") {
                    Some(number)
                } else if unit.eq_ignore_ascii_case("kb") ||
                              unit.eq_ignore_ascii_case("kib") {
                    number.checked_mul(1024)
                } else if unit.eq_ignore_ascii_case("mb") ||
                              unit.eq_ignore_ascii_case("mib") {
                    number.checked_mul(1024 * 1024)
                } else if unit.eq_ignore_ascii_case("gb") ||
                              unit.eq_ignore_ascii_case("gib") {
                    number.checked_mul(1024 * 1024 * 1024)
                } else if unit.eq_ignore_ascii_case("tb") ||
                              unit.eq_ignore_ascii_case("tib") {
                    number.checked_mul(1024 * 1024 * 1024 * 1024)
                } else {
                    return Err(E::invalid_value(&format!("invalid unit `{}`" ,
                                                         unit)));
                };
            match number {
                Some(n) => Ok(n),
                None => Err(E::invalid_value("value overflowed")),
            }
        }
    }
    d.deserialize(V)
}
