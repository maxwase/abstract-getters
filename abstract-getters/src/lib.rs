/// Defines a [Field] for a struct with a specific [type](Field::Type).
pub trait Field<Name> {
    type Type;

    /// Returns the value of a field.
    fn field(self) -> Self::Type;
}

/// A trait for [Get]ting a value from a specific [Field] with a specific [Field::Type].
pub trait Get: Sized {
    /// Returns a value of a [field](Field) called `Name`.
    fn get<Name>(self) -> <Self as Field<Name>>::Type
    where
        Self: Field<Name>,
    {
        <Self as Field<Name>>::field(self)
    }
}

/// Allow to get field from all types.
impl<T> Get for T {}

#[cfg(test)]
mod tests {
    use foo::{Bar, Baz};

    use super::*;

    #[derive(Debug, Clone)]
    struct Foo {
        foo: i32,
        bar: String,
        baz: String,
    }

    mod foo {
        use crate::Field;

        pub struct Foo;
        impl Field<Foo> for super::Foo {
            type Type = i32;
            fn field(self) -> Self::Type {
                self.foo
            }
        }
        impl<'a> Field<Foo> for &'a super::Foo {
            type Type = &'a i32;
            fn field(self) -> Self::Type {
                &self.foo
            }
        }
        impl<'a> Field<Foo> for &'a mut super::Foo {
            type Type = &'a mut i32;
            fn field(self) -> Self::Type {
                &mut self.foo
            }
        }

        pub struct Bar;
        impl Field<Bar> for super::Foo {
            type Type = String;
            fn field(self) -> Self::Type {
                self.bar
            }
        }
        impl<'a> Field<Bar> for &'a super::Foo {
            type Type = &'a String;
            fn field(self) -> Self::Type {
                &self.bar
            }
        }
        impl<'a> Field<Bar> for &'a mut super::Foo {
            type Type = &'a mut String;
            fn field(self) -> Self::Type {
                &mut self.bar
            }
        }

        pub struct Baz;
        impl Field<Baz> for super::Foo {
            type Type = String;
            fn field(self) -> Self::Type {
                self.baz
            }
        }
        impl<'a> Field<Baz> for &'a super::Foo {
            type Type = &'a String;
            fn field(self) -> Self::Type {
                &self.baz
            }
        }
        impl<'a> Field<Baz> for &'a mut super::Foo {
            type Type = &'a mut String;
            fn field(self) -> Self::Type {
                &mut self.baz
            }
        }
    }

    pub struct Foo2<'a, T> {
        number: T,
        re: &'a str,
    }

    mod foo2 {
        use super::Field;
        pub struct Number;

        impl<T> Field<Number> for super::Foo2<'_, T> {
            type Type = T;
            fn field(self) -> Self::Type {
                self.number
            }
        }

        impl<'top: 't, 't, T> Field<Number> for &'top super::Foo2<'t, T> {
            type Type = &'t T;
            fn field(self) -> Self::Type {
                &self.number
            }
        }

        impl<'top: 't, 't, T> Field<Number> for &'top mut super::Foo2<'t, T> {
            type Type = &'t mut T;
            fn field(self) -> Self::Type {
                &mut self.number
            }
        }

        pub struct Re;

        impl<'a, T> Field<Re> for super::Foo2<'a, T> {
            type Type = &'a str;
            fn field(self) -> Self::Type {
                self.re
            }
        }

        impl<'top, 't, T> Field<Re> for &'top super::Foo2<'t, T> {
            type Type = &'top &'t str;
            fn field(self) -> Self::Type {
                &self.re
            }
        }

        impl<'top, 't, T> Field<Re> for &'top mut super::Foo2<'t, T> {
            type Type = &'top mut &'t str;
            fn field(self) -> Self::Type {
                &mut self.re
            }
        }
    }

    #[test]
    fn test_foo() {
        let mut foo = Foo {
            foo: 42,
            bar: "Hello".to_string(),
            baz: "World".to_string(),
        };

        let _bar_ref: &String = (&foo).get::<Bar>();
        let _bar_mut: &mut String = (&mut foo).get::<Bar>();
        let _bar_val: String = foo.clone().get::<Bar>();

        let _baz_ref: &String = (&foo).get::<Baz>();
        let _baz_mut: &mut String = (&mut foo).get::<Baz>();
        let _baz_val: String = foo.clone().get::<Baz>();

        let _foo_ref: &i32 = (&foo).get::<foo::Foo>();
        let _foo_mut: &mut i32 = (&mut foo).get::<foo::Foo>();
        let _foo_val: i32 = foo.clone().get::<foo::Foo>();
    }

    #[test]
    fn test_foo2() {
        let mut foo2 = Foo2 {
            number: 42,
            re: "Hello",
        };

        let _re: &i32 = (&foo2).get::<foo2::Number>();
        assert_eq!(_re, &foo2.number);

        let re: &str = (&foo2).get::<foo2::Re>();
        assert_eq!(re, foo2.re);
        let _re: &mut &str = (&mut foo2).get::<foo2::Re>();
        let _re: &str = foo2.get::<foo2::Re>();
    }
}
