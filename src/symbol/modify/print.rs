use t_funk::{function::PrintLn, typeclass::functor::Fmap};

use crate::{LiftAdt, Modify, ModifyFunction};

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Print;

impl<F> Fmap<F> for Print {
    type Fmap = Self;

    fn fmap(self, _: F) -> Self::Fmap {
        self
    }
}

impl LiftAdt for Print {
    type LiftAdt = Modify<Self>;

    fn lift_adt(self) -> Self::LiftAdt {
        Modify(self)
    }
}

impl<D> ModifyFunction<D> for Print {
    type Inputs = String;

    type Function = PrintLn;

    fn modify_function(self) -> Self::Function {
        PrintLn
    }
}

