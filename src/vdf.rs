use std::{ops::{Index, IndexMut}, fmt};

use indexmap::IndexMap;

#[derive(Clone, PartialEq, Debug)]
pub enum VDF {
    Value(String),
    Keys(IndexMap<String, VDF>),
}

impl VDF {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get<S: Into<String>>(&self, key: S) -> Option<&Self> {
        let key = key.into();
        match self {
            Self::Value(_) => None,
            Self::Keys(map) => map.get(&key),
        }
    }

    pub fn get_mut<S: Into<String>>(&mut self, key: S) -> Option<&mut Self> {
        let key = key.into();
        match self {
            Self::Value(_) => None,
            Self::Keys(map) => map.get_mut(&key),
        }
    }

    pub fn insert(&mut self, key: impl ToString, value: impl Into<Self>) {
        match self {
            Self::Value(_) => panic!("cannot insert into value"),
            Self::Keys(map) => {
                let key = key.to_string();
                let value = value.into();
                map.insert(key, value);
            },
        }
    }

    pub fn to<T: FromStr + 'static>(&self) -> Result<T, T::Err> {
        match self {
            Self::Keys(_) => panic!("cannot access value from keys"),
            Self::Value(value) => T::from_str(value),
        }
    }
}

impl Default for VDF {
    fn default() -> Self {
        Self::Keys(IndexMap::new())
    }
}

impl From<Vec<VDF>> for VDF {
    fn from(v: Vec<Self>) -> Self {
        let mut vdf = Self::new();
        for (i, value) in v.into_iter().enumerate() {
            vdf.insert(i, value);
        }
        vdf
    }
}

impl<S: ToString> Index<S> for VDF {
    type Output = VDF;

    fn index(&self, index: S) -> &Self::Output {
        let index = index.to_string();
        match self {
            VDF::Keys(map) => {
                match map.get(&index) {
                    Some(value) => value,
                    None => panic!("invalid key"),
                }
            },
            VDF::Value(_) => panic!("cannot access key of value"),
        }
    }
}

impl<S: ToString> IndexMut<S> for VDF {
    fn index_mut(&mut self, index: S) -> &mut Self::Output {
        let index = index.to_string();
        match self {
            VDF::Keys(map) => {
                match map.get_mut(&index) {
                    Some(value) => value,
                    None => panic!("invalid key"),
                }
            },
            VDF::Value(_) => panic!("cannot access key of value"),
        }
    }
}

impl<S: ToString> From<S> for VDF {
    fn from(s: S) -> Self {
        Self::Value(s.to_string())
    }
}

pub trait FromStr: Sized {
    type Err;

    fn from_str(s: &str) -> Result<Self, Self::Err>;
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for str {
    fn to_string(&self) -> String {
        self.into()
    }
}

impl ToString for &str {
    fn to_string(&self) -> String {
        (*self).into()
    }
}

impl ToString for bool {
    fn to_string(&self) -> String {
        match self {
            true => ToString::to_string("1"),
            false => ToString::to_string("0"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseBoolError;

impl fmt::Display for ParseBoolError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        "provided string was not `1` or `0`".fmt(f)
    }
}

impl FromStr for bool {
    type Err = ParseBoolError;

    #[inline]
    fn from_str(s: &str) -> Result<bool, ParseBoolError> {
        match s {
            "1" => Ok(true),
            "0" => Ok(false),
            _ => Err(ParseBoolError),
        }
    }
}

macro_rules! vdf_string {
    ($ty:ident) => {
        impl FromStr for $ty {
            type Err = <$ty as std::str::FromStr>::Err;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <$ty as std::str::FromStr>::from_str(s)
            }
        }

        impl ToString for $ty {
            fn to_string(&self) -> String {
                <$ty as std::string::ToString>::to_string(self)
            }
        }
    };
}

vdf_string!(String);
vdf_string!(char);
// floats:
vdf_string!(f32);
vdf_string!(f64);
// signed integers:
vdf_string!(i8);
vdf_string!(i16);
vdf_string!(i32);
vdf_string!(i64);
vdf_string!(i128);
vdf_string!(isize);
// unsigned integers:
vdf_string!(u8);
vdf_string!(u16);
vdf_string!(u32);
vdf_string!(u64);
vdf_string!(u128);
vdf_string!(usize);

#[macro_export(local_inner_macros)]
macro_rules! vdf {
    ($($json:tt)+) => {
        vdf_internal!($($json)+)
    };
}

#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! vdf_internal {
    // Arrays:

    // Done with trailing comma.
    (@array [$($elems:expr,)*]) => {
        vdf_internal_vec![$($elems,)*]
    };

    // Done without trailing comma.
    (@array [$($elems:expr),*]) => {
        vdf_internal_vec![$($elems),*]
    };

    // Next element is `true`.
    (@array [$($elems:expr,)*] true $($rest:tt)*) => {
        vdf_internal!(@array [$($elems,)* vdf_internal!(true)] $($rest)*)
    };

    // Next element is `false`.
    (@array [$($elems:expr,)*] false $($rest:tt)*) => {
        vdf_internal!(@array [$($elems,)* vdf_internal!(false)] $($rest)*)
    };

    // Next element is an array.
    (@array [$($elems:expr,)*] [$($array:tt)*] $($rest:tt)*) => {
        vdf_internal!(@array [$($elems,)* vdf_internal!([$($array)*])] $($rest)*)
    };

    // Next element is a map.
    (@array [$($elems:expr,)*] {$($map:tt)*} $($rest:tt)*) => {
        json_internal!(@array [$($elems,)* vdf_internal!({$($map)*})] $($rest)*)
    };

    // Next element is an expression followed by comma.
    (@array [$($elems:expr,)*] $next:expr, $($rest:tt)*) => {
        vdf_internal!(@array [$($elems,)* vdf_internal!($next),] $($rest)*)
    };

    // Last element is an expression with no trailing comma.
    (@array [$($elems:expr,)*] $last:expr) => {
        vdf_internal!(@array [$($elems,)* vdf_internal!($last)])
    };

    // Comma after the most recent element.
    (@array [$($elems:expr),*] , $($rest:tt)*) => {
        vdf_internal!(@array [$($elems,)*] $($rest)*)
    };

    // Unexpected token after most recent element.
    (@array [$($elems:expr),*] $unexpected:tt $($rest:tt)*) => {
        vdf_unexpected!($unexpected)
    };

    // Objects:

    // Done.
    (@map $map:ident () () ()) => {};

    // // Insert the current entry followed by trailing comma.
    // (@object $object:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
    //     let _ = $object.insert(($($key)+).into(), $value);
    //     json_internal!(@object $object () ($($rest)*) ($($rest)*));
    // };

    // // Current entry followed by unexpected token.
    // (@object $object:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
    //     json_unexpected!($unexpected);
    // };

    // // Insert the last entry without trailing comma.
    // (@object $object:ident [$($key:tt)+] ($value:expr)) => {
    //     let _ = $object.insert(($($key)+).into(), $value);
    // };

    // Insert the current entry followed by trailing comma.
    (@map $map:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $map.insert(($($key)+).to_string(), $value);
        vdf_internal!(@map $map () ($($rest)*) ($($rest)*));
    };

    // Insert the last entry without trailing comma.
    (@map $map:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $map.insert(($($key)+).to_string(), $value);
    };

    
    (@map $map:ident [$($key:tt)+] (: false , $($rest:tt)*) $copy:tt) => {
        vdf_internal!(@map $map [$($key)+] (vdf_internal!(false)) $($rest)*);
    };

    (@map $map:ident [$($key:tt)+] (: true , $($rest:tt)*) $copy:tt) => {
        vdf_internal!(@map $map [$($key)+] (vdf_internal!(true)) $($rest)*);
    };



    // Next value is an expression followed by comma.
    (@map $map:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        vdf_internal!(@map $map [$($key)+] (vdf_internal!($value)) , $($rest)*);
    };



    // Last value is an expression with no trailing comma.
    (@map $map:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        vdf_internal!(@map $map [$($key)+] (vdf_internal!($value)));
    };

    (@map $map:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        vdf_internal!(@map $map ($key) (: $($rest)*) (: $($rest)*));
    };

    // Munch a token into the current key.
    (@map $map:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        vdf_internal!(@map $map ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    // Base:

    (true) => {
        $crate::vdf::VDF::Value("1".into())
    };

    (false) => {
        $crate::vdf::VDF::Value("0".into())
    };

    ([]) => {
        $crate::vdf::VDF::new()
    };

    ([ $($tt:tt)+ ]) => {
        $crate::vdf::VDF::from(vdf_internal!(@array [] $($tt)+))
    };

    ({}) => {
        $crate::vdf::VDF::new()
    };

    ({ $($tt:tt)+ }) => {
        {
            let mut vdf = $crate::vdf::VDF::new();
            vdf_internal!(@map vdf () ($($tt)+) ($($tt)+));
            vdf
        }
    };

    ($other:expr) => {
        $crate::vdf::VDF::Value($other.to_string())
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! vdf_internal_vec {
    ($($content:tt)*) => {
        vec![$($content)*]
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! vdf_unexpected {
    () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! vdf_test {
    ($($content:tt)*) => {
        println!($($content)*);
    };
}