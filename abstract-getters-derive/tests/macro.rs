use core::fmt;
use std::fmt::Display;

use abstract_getters::{Field, Get};
use abstract_getters_derive::Getters;

#[derive(Debug, Clone, Getters)]
struct Foo3<'a, T> {
    t: T,
    re: &'a String,
}

#[test]
fn test_derive() {
    let mut foo3 = Foo3 {
        t: 42,
        re: &"Hello".to_string(),
    };
    let _foo3_ref: &i32 = (&foo3).get::<foo_3::t>();
    let _foo3_mut: &mut i32 = (&mut foo3).get::<foo_3::t>();
    let _foo3_val: i32 = foo3.clone().get::<foo_3::t>();

    let _foo3_re_ref: &&String = (&foo3).get::<foo_3::re>();
    let _foo3_re_mut: &mut &String = (&mut foo3).get::<foo_3::re>();
    let _foo3_re_val: &String = foo3.get::<foo_3::re>();
}

fn abstract_get_field<'a, 'b, T, Name>(val: &'b Foo3<'a, T>) -> String
where
    &'b Foo3<'a, T>: Field<Name>,
    <&'b Foo3<'a, T> as Field<Name>>::Type: Display,
{
    val.get::<Name>().to_string()
}

#[test]
fn test_abstract_get_genetic() {
    let foo3 = Foo3 {
        t: 42,
        re: &"Hello".to_string(),
    };

    let field = abstract_get_field::<_, foo_3::t>(&foo3);
    assert_eq!(field, foo3.t.to_string());

    let re = abstract_get_field::<_, foo_3::re>(&foo3);
    assert_eq!(re, foo3.re.to_string());
}
#[derive(Debug, Clone, Getters)]
struct Simple {
    a: i32,
    b: String,
}

#[derive(Debug, Clone, Getters)]
struct Simple2 {
    a: i32,
    b: String,
}

fn field_to_string<'s, 't, Name, S, T>(from: &'s S) -> String
where
    &'s S: Get + Field<Name, Type = &'t T>,
    T: Display + 't,
{
    let got = from.get::<Name>();
    got.to_string()
}

fn require_i32<Name, S: Get + Field<Name, Type = i32>>(from: S) {
    let got = from.get::<Name>();
    assert_eq!(std::any::type_name_of_val(&got), "i32");
}

#[test]
fn test_simple() {
    let simple = Simple {
        a: 42,
        b: "Hello".to_string(),
    };

    let a = (&simple).get::<simple::a>();
    let b = (&simple).get::<simple::b>();

    let a_string = field_to_string::<simple::a, _, _>(&simple);
    assert_eq!(a_string, simple.a.to_string());
    assert_eq!(a_string, a.to_string());

    let b_string = field_to_string::<simple::b, _, _>(&simple);
    assert_eq!(b_string, simple.b);
    assert_eq!(b_string, *b);

    require_i32::<simple::a, _>(simple);
}

#[test]
fn test_simple2() {
    let simple = Simple2 {
        a: 42,
        b: "Hello".to_string(),
    };

    let a = (&simple).get::<simple_2::a>();
    let b = (&simple).get::<simple_2::b>();

    let a_string = field_to_string::<simple_2::a, _, _>(&simple);
    assert_eq!(a_string, simple.a.to_string());
    assert_eq!(a_string, a.to_string());

    let b_string = field_to_string::<simple_2::b, _, _>(&simple);
    assert_eq!(b_string, simple.b);
    assert_eq!(b_string, *b);

    require_i32::<simple_2::a, _>(simple);
}

#[derive(Debug, Clone, Getters)]
struct Tuple(i32, String);

impl fmt::Display for Tuple {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("Tuple").finish()
    }
}

#[test]
fn test_tuple() {
    let tuple = Tuple(42, "Hello".to_string());

    let a = (&tuple).get::<tuple::_0>();
    let b = (&tuple).get::<tuple::_1>();

    let a_string = field_to_string::<tuple::_0, _, _>(&tuple);
    assert_eq!(a_string, tuple.0.to_string());
    assert_eq!(a_string, a.to_string());

    let b_string = field_to_string::<tuple::_1, _, _>(&tuple);
    assert_eq!(b_string, tuple.1);
    assert_eq!(b_string, *b);
}

#[derive(Getters)]
enum DataEnum {
    InnerStruct(Tuple),
    Inline {
        a: i32,
        b: String,
    },
    Tuple(i32, String),
    #[allow(unused)]
    Unit,
    Single(i32),
}

impl DataEnum {
    fn inline(self) -> (i32, String) {
        match self {
            DataEnum::Inline { a, b } => (a, b),
            _ => panic!("Not an inline variant"),
        }
    }

    fn try_into_single(self) -> Option<i32> {
        if let Self::Single(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

fn field_to_string_opt<'s, 't, Name, S, T>(from: &'s S) -> Option<String>
where
    &'s S: Get + Field<Name, Type = Option<&'t T>>,
    T: Display + 't,
{
    let got = from.get::<Name>();
    got.map(ToString::to_string)
}

fn require_i32_opt<Name, S: Get + Field<Name, Type = Option<i32>>>(from: S) {
    let got = from.get::<Name>();
    assert_eq!(
        std::any::type_name_of_val(&got),
        "core::option::Option<i32>"
    );
}

#[test]
fn test_data_enum() {
    let data = DataEnum::InnerStruct(Tuple(42, "Hello".to_string()));

    let _a: Option<&Tuple> = (&data).get::<data_enum::inner_struct::_0>();

    let a_string = field_to_string_opt::<data_enum::inline::a, _, _>(&data);
    assert_eq!(a_string, None);

    let data = DataEnum::Inline {
        a: 42,
        b: "Hello".to_string(),
    };
    let a_string = field_to_string_opt::<data_enum::inline::a, _, _>(&data);
    assert_eq!(a_string.unwrap(), data.inline().0.to_string());

    require_i32_opt::<data_enum::single::_0, _>(DataEnum::Single(0));

    let data = DataEnum::Single(0);
    let a_string = field_to_string_opt::<data_enum::single::_0, _, _>(&data);
    assert_eq!(
        a_string.unwrap(),
        data.try_into_single().unwrap().to_string()
    );

    let mut data = DataEnum::Tuple(0, "Hello".to_string());
    let _tuple: Option<&i32> = (&data).get::<data_enum::tuple::_0>();
    let _tuple: Option<&mut String> = (&mut data).get::<data_enum::tuple::_1>();
}

#[derive(Getters)]
struct InnerStruct(Tuple);

#[test]
fn test_inner_struct() {
    let inner = InnerStruct(Tuple(42, "Hello".to_string()));

    let _a: &Tuple = (&inner).get::<inner_struct::_0>();

    let tuple_ref = (&inner).get::<inner_struct::_0>();
    assert_eq!(tuple_ref.to_string(), inner.0.to_string());
    let a_string = tuple_ref.get::<tuple::_0>().to_string();
    assert_eq!(a_string, inner.0.0.to_string());
}
