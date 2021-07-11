#[macro_export]
macro_rules! extract {
    ($var:path, ($( $item:tt ),*), $enum:expr) => {
        match $enum {
            $var($( $item ),*) => Some(($( $item ),*)),
            _ => None,
        }
    };
}

macro_rules! gen_extract_shorthand {
    ($name:ident ($( $item:ident ),*)) => {
        #[allow(unused_macros)]
        #[macro_export]
        macro_rules! $name {
            ($var:path, $enum:expr) => {
                extract!($var, ($( $item ),*), $enum)
            };
        }
    };

    ($( $name:ident ($( $item:ident ),*) );*;) => {
        $( gen_extract_shorthand!($name ($( $item ),*)); )*
    };
}
gen_extract_shorthand! {
    extract_monuple(a); extract_1tuple(a);
    extract_couple(a, b); extract_2tuple(a, b);
    extract_triple(a, b, c); extract_3tuple(a, b, c);
    extract_quadruple(a, b, c, d); extract_4tuple(a, b, c, d);
}

#[macro_export]
macro_rules! extract_variant_method {
    ($method:ident (&self) { $var:path as ($( $item_id:ident ),*): ($( $item_ty:ty ),*) }) => {
        #[allow(dead_code, unused_parens)]
        pub fn $method(&self) -> Option<($( $item_ty ),*)> {
            extract!($var, ($( $item_id ),*), self)
        }
    };

    ($method:ident (self) { $var:path as ($( $item_id:ident ),*): ($( $item_ty:ty ),*) }) => {
        #[allow(dead_code, unused_parens)]
        pub fn $method(self) -> Option<($( $item_ty ),*)> {
            extract!($var, ($( $item_id ),*), self)
        }
    };
}

#[cfg(test)]
mod tests {
    enum Enum {
        A(usize),
        B(usize),
        C(usize, usize),
    }

    impl Enum {
        extract_variant_method!(as_a(&self) { Self::A as (a): (&usize) });
        extract_variant_method!(to_a(self) { Self::A as (a): (usize) });
        extract_variant_method!(as_b(&self) { Self::B as (a): (&usize) });
        extract_variant_method!(to_b(self) { Self::B as (a): (usize) });
        extract_variant_method!(as_c(&self) { Self::C as (a, b): (&usize, &usize) });
        extract_variant_method!(to_c(self) { Self::C as (a, b): (usize, usize) });
    }

    #[test]
    fn extract_macro() {
        assert_eq!(extract!(Enum::A, (a), Enum::A(0)), Some(0),);
        assert_eq!(extract!(Enum::A, (a), Enum::B(0)), None,);

        assert_eq!(extract!(Enum::C, (a, b), Enum::C(0, 0)), Some((0, 0)),);
        assert_eq!(extract!(Enum::C, (a, b), Enum::A(0)), None,);
    }

    #[test]
    fn extract_macro_shorthand() {
        assert_eq!(extract_monuple!(Enum::A, Enum::A(0)), Some(0),);
        assert_eq!(extract_monuple!(Enum::A, Enum::B(0)), None,);

        assert_eq!(extract_couple!(Enum::C, Enum::C(0, 0)), Some((0, 0)),);
        assert_eq!(extract_couple!(Enum::C, Enum::A(0)), None,);
    }

    #[test]
    fn extract_method() {
        assert_eq!(Enum::A(0).as_a(), Some(&0),);
        assert_eq!(Enum::B(0).as_a(), None,);
    }
}
