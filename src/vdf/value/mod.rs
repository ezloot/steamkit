use indexmap::IndexMap;
use std::fmt;
use std::ops::{Index, IndexMut};

#[derive(Clone, PartialEq, Debug)]
pub enum Value {
    String(String),
    Map(IndexMap<String, Value>),
}

impl Value {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, key: impl ToString) -> Option<&Self> {
        match self {
            Self::String(_) => None,
            Self::Map(map) => {
                let key = key.to_string();
                map.get(&key)
            }
        }
    }

    pub fn get_mut(&mut self, key: impl ToString) -> Option<&mut Self> {
        match self {
            Self::String(_) => None,
            Self::Map(map) => {
                let key = key.to_string();
                map.get_mut(&key)
            }
        }
    }

    pub fn insert(&mut self, key: impl ToString, value: impl Into<Self>) -> bool {
        match self {
            Self::String(_) => false,
            Self::Map(map) => {
                let key = key.to_string();
                let value = value.into();
                map.insert(key, value);
                true
            }
        }
    }

    pub fn to<T: FromStr + 'static>(&self) -> Result<T, T::Err> {
        match self {
            Self::String(value) => T::from_str(value),
            Self::Map(_) => panic!("cannot access value from keys"),
        }
    }

    pub fn set(&mut self, value: &str) -> bool {
        match self {
            Self::String(s) => {
                s.clear();
                s.push_str(value);
                true
            }
            Self::Map(_) => false,
        }
    }
}

impl Default for Value {
    fn default() -> Self {
        Self::Map(IndexMap::new())
    }
}

impl From<Vec<Value>> for Value {
    fn from(v: Vec<Self>) -> Self {
        let mut value = Self::new();
        for (i, entry) in v.into_iter().enumerate() {
            value.insert(i, entry);
        }
        value
    }
}

impl<S: ToString> Index<S> for Value {
    type Output = Value;

    fn index(&self, index: S) -> &Self::Output {
        let index = index.to_string();
        match self {
            Self::Map(map) => match map.get(&index) {
                Some(value) => value,
                None => panic!("invalid key"),
            },
            Self::String(_) => panic!("cannot access key of value"),
        }
    }
}

impl<S: ToString> IndexMut<S> for Value {
    fn index_mut(&mut self, index: S) -> &mut Self::Output {
        let index = index.to_string();
        match self {
            Self::Map(map) => match map.get_mut(&index) {
                Some(value) => value,
                None => panic!("invalid key"),
            },
            Self::String(_) => panic!("cannot access key of value"),
        }
    }
}

impl<S: ToString> From<S> for Value {
    fn from(s: S) -> Self {
        Self::String(s.to_string())
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
    ($ty:ident, $fmt:ident) => {
        impl FromStr for $ty {
            type Err = <$ty as std::str::FromStr>::Err;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                <$ty as std::str::FromStr>::from_str(s)
            }
        }

        impl ToString for $ty {
            fn to_string(&self) -> String {
                let mut buffer = $fmt::Buffer::new();
                buffer.format(*self).to_owned()
            }
        }
    };
}

vdf_string!(String);
vdf_string!(char);
// floats:
vdf_string!(f32, ryu);
vdf_string!(f64, ryu);
// signed integers:
vdf_string!(i8, itoa);
vdf_string!(i16, itoa);
vdf_string!(i32, itoa);
vdf_string!(i64, itoa);
vdf_string!(i128, itoa);
vdf_string!(isize, itoa);
// unsigned integers:
vdf_string!(u8, itoa);
vdf_string!(u16, itoa);
vdf_string!(u32, itoa);
vdf_string!(u64, itoa);
vdf_string!(u128, itoa);
vdf_string!(usize, itoa);

#[macro_export(local_inner_macros)]
macro_rules! vdf {
    ($($json:tt)+) => {
    vdf_internal!($($json)+)
    };
}

// Replicates serde_json macro from https://docs.serde.rs/src/serde_json/macros.rs.html
#[macro_export(local_inner_macros)]
#[doc(hidden)]
macro_rules! vdf_internal {
    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an array [...]. Produces a vec![...]
    // of the elements.
    //
    // Must be invoked as: vdf_internal!(@array [] $($tt)*)
    //////////////////////////////////////////////////////////////////////////

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
        vdf_internal!(@array [$($elems,)* vdf_internal!({$($map)*})] $($rest)*)
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

    //////////////////////////////////////////////////////////////////////////
    // TT muncher for parsing the inside of an object {...}. Each entry is
    // inserted into the given map variable.
    //
    // Must be invoked as: vdf_internal!(@map $map () ($($tt)*) ($($tt)*))
    //
    // We require two copies of the input tokens so that we can match on one
    // copy and trigger errors on the other copy.
    //////////////////////////////////////////////////////////////////////////

    // Done.
    (@map $map:ident () () ()) => {};

    // Insert the current entry followed by trailing comma.
    (@map $map:ident [$($key:tt)+] ($value:expr) , $($rest:tt)*) => {
        let _ = $map.insert(($($key)+).to_string(), $value);
        vdf_internal!(@map $map () ($($rest)*) ($($rest)*));
    };

    // Current entry followed by unexpected token.
    (@map $map:ident [$($key:tt)+] ($value:expr) $unexpected:tt $($rest:tt)*) => {
        vdf_unexpected!($unexpected);
    };

    // Insert the last entry without trailing comma.
    (@map $map:ident [$($key:tt)+] ($value:expr)) => {
        let _ = $map.insert(($($key)+).to_string(), $value);
    };

    // Next value is `true`.
    (@map $map:ident ($($key:tt)+) (: true $($rest:tt)*) $copy:tt) => {
        vdf_internal!(@map $map [$($key)+] (vdf_internal!(true)) $($rest)*);
    };

    // Next value is `false`.
    (@map $map:ident ($($key:tt)+) (: false $($rest:tt)*) $copy:tt) => {
        vdf_internal!(@map $map [$($key)+] (vdf_internal!(false)) $($rest)*);
    };

    // Next value is an array.
    (@map $map:ident ($($key:tt)+) (: [$($array:tt)*] $($rest:tt)*) $copy:tt) => {
        vdf_internal!(@map $map [$($key)+] (vdf_internal!([$($array)*])) $($rest)*);
    };

    // Next value is a map.
    (@map $map:ident ($($key:tt)+) (: {$($map_inner:tt)*} $($rest:tt)*) $copy:tt) => {
        vdf_internal!(@map $map [$($key)+] (vdf_internal!({$($map_inner)*})) $($rest)*);
    };

    // Next value is an expression followed by comma.
    (@map $map:ident ($($key:tt)+) (: $value:expr , $($rest:tt)*) $copy:tt) => {
        vdf_internal!(@map $map [$($key)+] (vdf_internal!($value)) , $($rest)*);
    };

    // Last value is an expression with no trailing comma.
    (@map $map:ident ($($key:tt)+) (: $value:expr) $copy:tt) => {
        vdf_internal!(@map $map [$($key)+] (vdf_internal!($value)));
    };

    // Missing value for last entry. Trigger a reasonable error message.
    (@map $map:ident ($($key:tt)+) (:) $copy:tt) => {
        // "unexpected end of macro invocation"
        vdf_internal!();
    };

    // Missing colon and value for last entry. Trigger a reasonable error
    // message.
    (@map $map:ident ($($key:tt)+) () $copy:tt) => {
        // "unexpected end of macro invocation"
        vdf_internal!();
    };

    // Misplaced colon. Trigger a reasonable error message.
    (@map $map:ident () (: $($rest:tt)*) ($colon:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `:`".
        vdf_unexpected!($colon);
    };

    // Found a comma inside a key. Trigger a reasonable error message.
    (@map $map:ident ($($key:tt)*) (, $($rest:tt)*) ($comma:tt $($copy:tt)*)) => {
        // Takes no arguments so "no rules expected the token `,`".
        vdf_unexpected!($comma);
    };

    // Key is fully parenthesized. This avoids clippy double_parens false
    // positives because the parenthesization may be necessary here.
    (@map $map:ident () (($key:expr) : $($rest:tt)*) $copy:tt) => {
        vdf_internal!(@map $map ($key) (: $($rest)*) (: $($rest)*));
    };

    // Refuse to absorb colon token into key expression.
    (@map $map:ident ($($key:tt)*) (: $($unexpected:tt)+) $copy:tt) => {
        vdf_expect_expr_comma!($($unexpected)+);
    };

    // Munch a token into the current key.
    (@map $map:ident ($($key:tt)*) ($tt:tt $($rest:tt)*) $copy:tt) => {
        vdf_internal!(@map $map ($($key)* $tt) ($($rest)*) ($($rest)*));
    };

    //////////////////////////////////////////////////////////////////////////
    // The main implementation.
    //
    // Must be invoked as: vdf_internal!($($json)+)
    //////////////////////////////////////////////////////////////////////////

    (true) => {
        $crate::vdf::Value::String("1".into())
    };

    (false) => {
        $crate::vdf::Value::String("0".into())
    };

    ([]) => {
        $crate::vdf::Value::new()
    };

    ([ $($tt:tt)+ ]) => {
        $crate::vdf::Value::from(vdf_internal!(@array [] $($tt)+))
    };

    ({}) => {
        $crate::vdf::Value::new()
    };

    ({ $($tt:tt)+ }) => {
        {
            let mut value = $crate::vdf::Value::new();
            vdf_internal!(@map value () ($($tt)+) ($($tt)+));
            value
        }
    };

    ($other:expr) => {
        $crate::vdf::Value::String($other.to_string())
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
macro_rules! Value_unexpected {
    () => {};
}

#[macro_export]
#[doc(hidden)]
macro_rules! Value_expect_expr_comma {
    ($e:expr , $($tt:tt)*) => {};
}
