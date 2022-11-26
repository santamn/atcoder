#[allow(unused_macros)]
macro_rules! monoid {
    (
        $type:ty where  [$( $params:tt )*];
        one = $one:expr;
        mul($self:ident, $y:ident) =  $code:block
        $(;)*
    ) => {
        impl<$($params)*> std::ops::Mul for $type {
            type Output = Self;
            fn mul($self, $y: Self) -> Self { $code }
        }
        impl<$($params)*> std::ops::MulAssign for $type where Self: Clone {
            fn mul_assign(&mut $self, $y: Self) {
                *$self = (*$self).clone() * $y;
            }
        }
        impl<$($params)*> Monoid for $type {
            fn one() -> Self { $one }
        }
    };

    (
        $type:ty;
        one = $one:expr;
        mul($self:ident, $y:ident) = $code:block
        $(;)*
    ) => {
        monoid! { $type where []; one = $one; mul($self, $y) = $code; }
    };
}

pub trait Monoid: Sized + PartialEq + std::ops::Mul<Output = Self> + std::ops::MulAssign {
    fn one() -> Self;
}

#[derive(Clone, Copy, PartialEq)]
struct MyType<X>(X)
where
    X: Copy + PartialEq;

fn expand() {
    monoid! {
        MyType<X> where [ X: Copy + PartialEq + Default ];
        one = MyType(X::default());
        mul(self, other) =  { other }
    }
}
