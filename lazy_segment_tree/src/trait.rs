trait Monoid {
    fn one() -> Self;
}

trait OperatorMonoid<X>
where
    Self: Monoid,
{
    fn act(self, other: X) -> X;
}
