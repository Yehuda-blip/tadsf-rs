// #![feature(const_trait_impl)]
// #![feature(generic_const_exprs)]

use std::marker::PhantomData;

#[derive(Debug)]
pub struct Add<const N: i32, const M: i32> {
    no_data: PhantomData<i32>
}

impl<const N: i32, const M: i32> Add<N, M> {
    pub const fn sym() -> i32{
        N + M
    }
}

#[macro_export]
macro_rules! add {
    ($N:literal, $M:literal) => {
        Add::<$N, $M>
    };

    (Add<$N:literal, $M:literal>, $K:literal) => {
        Add::<{ $N  + $M }, $K>
    };

    ($K:literal, Add<$N:literal, $M:literal>) => {
        add!(Add<$N, $M>, $K)
    };

    (Add<$N:literal, $M:literal>, Add<$K:literal, $L:literal>) => {
        add!(Add<{$N + $M}, {$K + $L}>)
    };

    (Add<$N:literal, $M:literal>, $rest:tt) => {
        add!(Add<$N, $M>, add!($rest))
    };

    ($M:literal, $rest:tt) => {
        add!(Add<$N, $M>, add!($rest))
    };

    ($var1:ident, $var2:ident) => {
        Add::<{ $var1::sym() }, {$var2::sym()}>
    }
}
