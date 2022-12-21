use crabmole_derive::constn;

pub struct Foo<T, R>(T, R);

pub struct Bar<R, T>(R, T);

constn! {
    u8 {
        pub FOO_0 = 0;
        /// The first element
        pub(crate) FOO_1 = 1;
        pub FOO_2 = 2;
    },
    Foo<u64, i64> {
        pub FOO_00 = Foo(0, 0);
        pub FOO_11 = Foo(1, 1);
        FOO_22 = Foo(2, 2);
    },
    impl Foo<u32, u64> {
        u32 {
            pub B0 = 0;
            pub B1 = 1;
            pub B2 = 2;
        },
        Bar<u32, u64> {
            pub BAR_0 = Bar(0, 0);
        },

    },
    impl<T: core::fmt::Debug, R: core::fmt::Debug> Foo<T, R> {
        Bar<u64, i64> {
            pub BAR_00 = Bar(0, 0);
            pub(crate) BAR_11 = Bar(1, 1);
            BAR_22 = Bar(2, 2);
        },
        u32 {
            pub BAR_1 = 1;
            pub BAR_2 = 2;
            pub BAR_3 = 3;
        }
    },
}

fn main() {}
