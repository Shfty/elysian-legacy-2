use t_funk::{
    macros::impl_adt,
    typeclass::{
        foldable::{Foldl, FoldlT},
        monoid::{Mconcat, Mempty, MemptyT},
        semigroup::MappendF,
    },
};

use crate::{Combine, Sequence, Unit};

impl_adt! {
    impl<A, B, C> Mconcat for Unit<A> | Sequence<A, B> | Combine<A, B, C>
    where
        Self: Mempty + Foldl<MappendF, <Self as Mempty>::Mempty>,
    {
        type Mconcat = FoldlT<Self, MappendF, MemptyT<Self>>;

        fn mconcat(self) -> Self::Mconcat {
            self.foldl(MappendF::default(), Self::mempty())
        }
    }
}
