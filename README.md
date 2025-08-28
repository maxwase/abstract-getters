Abstract getters
=============
[![crates](https://img.shields.io/crates/v/abstract-getters?style=for-the-badge)](https://crates.io/crates/abstract-getters/)
[![doc](https://img.shields.io/docsrs/abstract-getters?style=for-the-badge)](https://docs.rs/abstract-getters/latest/)

Abstract what, how and from where one wants to get a value using 2 traits!

Forget about `get_ref`, `get_mut`, `into_inner` and others, one `get` to rule them all!

## How it works

Under the hood, the `abstract_getters_derive`'s macro generates 3 implementations of **the same** trait: for T, &T, and &mut T.
It also creates a module with the StructName converted into snake_case. The module contains generated ZST structs with the same
name as the struct's fields. You can later use a qualified path like `struct_name::foo` to get the field. Whether the field is returned as a reference, mutable reference, or an owned value depends on the provided value to the getter; types do **not** need to be adjusted manually!

## Examples

In the example below you can see how the type is determined by the ownership that the `get` method is given.

```rust
use abstract_getters::Get;
use abstract_getters_derive::Getters;

#[derive(Getters)]
struct ConcreteStruct {
    a: i32,
    b: String,
    c: (usize, &'static str),
}

fn main() {
    let mut concrete = ConcreteStruct {
        a: 24,
        b: "Hello".to_string(),
        c: (7, "World"),
    };

    {
        let a/* : &i32 */ = (&concrete).get::<concrete_struct::a>();
        let b/* : &String */ = (&concrete).get::<concrete_struct::b>();
        let c/* : &(usize, &str) */ = (&concrete).get::<concrete_struct::c>();
    }

    {
        let a/* : &mut i32 */ = (&mut concrete).get::<concrete_struct::a>();
        let b/* : &mut String */ = (&mut concrete).get::<concrete_struct::b>();
        let c/* : &mut (usize, &str) */ = (&mut concrete).get::<concrete_struct::c>();
    }
    {
        let a/* : i32 */ = concrete.get::<concrete_struct::a>();
        // The value was moved
        // let b/* : String */ = concrete.clone().get::<concrete_struct::b>();
        // let c/* : (usize, &str) */ = concrete.clone().get::<concrete_struct::c>();
    }
}
```

It's possible to require one or more fields from ***a*** value.
It can be useful when designing an interface, which accepts a large struct with a lot of fields,
when in reality it would only need a couple. Using these getters, you can design refactor-proof APIs!
It's also possible to give users opportunities to define their own types instead of requiring them
to implement yet another trait that *you* would need to maintain. It's always easier to define a
module/struct alias rather than have duplicated fields!

```rust
use abstract_getters::{Field, Get};

struct LargeStruct {
    // The function need only a
    a: i32,
    // and b
    b: String,

    // To be refactored
    c: f64,
    d: bool,
    // 100500 more fields
}


/// For a `V`alue, that has a Namei32 [Field] with a [type](Field::Type) [i32]...
/// For a `V`alue, that has a NameString [Field] with a [type](Field::Type) [String]...
fn require_i32_and_string<'v, Namei32, NameString, V>(from: &'v V)
where
    &'v V: Get + Field<Namei32, Type = i32> + Field<NameString, Type = String>,
{
    let got_i32 = from.get::<Namei32>();
    assert_eq!(std::any::type_name_of_val(&got_i32), "i32");

    let got_string = from.get::<NameString>();
    assert_eq!(std::any::type_name_of_val(&got_string), "String");
}
```


It is also possible to derive getters for enums, no more `as_`, `try_` and `into_`!

```rust
use abstract_getters::Get;

#[derive(Getters)]
enum DataEnum {
    Inline {
        a: i32,
        b: String,
    },
    Tuple(i32, String),
    #[allow(unused)]
    Unit,
    Single(i32),
}

fn test_data_enum() {
    let mut data = DataEnum::Tuple(42, "Hello".to_string());

    let wrong_variant/* : Option<&i32> */ = (&data).get::<data_enum::inline::a>();
    assert_eq!(wrong_variant, None);
    let int/* : Option<&mut i32> */ = (&mut data).get::<data_enum::tuple::_0>();
    assert_eq!(*int.unwrap(), 42);
}
```