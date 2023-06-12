use std::marker::PhantomData;

use t_funk::{
    closure::{CallF, Closure, Compose, ComposeLT, OutputT},
    collection::set::{DropF, LiftContext, LiftContextT},
    macros::{functions, types},
    typeclass::arrow::{Fanout, FanoutT},
};

use crate::{
    interpreter::register_machine::evaluate_function::FunctionT,
    interpreter::register_machine::evaluate_function::MovesT, AdtEnd, Combine, EvaluateFunction,
    LiftEvaluates, Modify, NotAdtEnd, Run, Shape, Then, LiftEvaluatesT,
};

use super::evaluate_function::InputsT;

#[functions]
#[types]
pub trait LiftEvaluate<D> {
    type LiftEvaluate;

    fn lift_evaluate(self) -> Self::LiftEvaluate;
}

impl<A, D> LiftEvaluate<D> for Shape<A>
where
    D: LiftEvaluates<A>,
{
    type LiftEvaluate = LiftEvaluatesT<D, A>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        D::lift_evaluates(self.0)
    }
}

impl<A, D> LiftEvaluate<D> for Modify<A>
where
    A: EvaluateFunction<D>,
    FunctionT<A, D>: LiftContext<InputsT<A, D>>,
    LiftContextT<FunctionT<A, D>, InputsT<A, D>>: Fanout<DropF<MovesT<A, D>>>,
{
    type LiftEvaluate = ComposeLT<
        FanoutT<LiftContextT<FunctionT<A, D>, InputsT<A, D>>, DropF<MovesT<A, D>>>,
        CallF,
    >;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        self.0
            .evaluate_function()
            .lift_context()
            .fanout(DropF::<MovesT<A, D>>::default())
            .compose_l(CallF)
    }
}

impl<T, D> LiftEvaluate<D> for Run<T>
where
    T: LiftEvaluate<D>,
{
    type LiftEvaluate = LiftEvaluateT<T, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        self.0.lift_evaluate()
    }
}

impl<A, B, D> LiftEvaluate<D> for Then<A, B>
where
    A: LiftEvaluate<D>,
    B: LiftEvaluate<D>,
    LiftEvaluateT<A, D>: Compose<LiftEvaluateT<B, D>>,
    B: NotAdtEnd,
{
    type LiftEvaluate = ComposeLT<LiftEvaluateT<A, D>, LiftEvaluateT<B, D>>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        self.0.lift_evaluate().compose_l(self.1.lift_evaluate())
    }
}

impl<A, D> LiftEvaluate<D> for Then<A, AdtEnd>
where
    A: LiftEvaluate<D>,
{
    type LiftEvaluate = LiftEvaluateT<A, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        self.0.lift_evaluate()
    }
}

impl<A, B, F, D> LiftEvaluate<D> for Combine<A, B, F> {
    type LiftEvaluate = LiftEvaluateCombine<A, B, F, D>;

    fn lift_evaluate(self) -> Self::LiftEvaluate {
        LiftEvaluateCombine(self.0, self.1, self.2, PhantomData)
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LiftEvaluateCombine<A, B, F, D>(pub A, pub B, pub F, pub PhantomData<D>);

impl<A, B, F, D, C> Closure<C> for LiftEvaluateCombine<A, B, F, D>
where
    A: Clone + LiftEvaluate<D>,
    B: Clone + LiftEvaluate<D>,
    F: Closure<(A, B, C, LiftEvaluateT<A, D>, LiftEvaluateT<B, D>)>,
{
    type Output = OutputT<F, (A, B, C, LiftEvaluateT<A, D>, LiftEvaluateT<B, D>)>;

    fn call(self, input: C) -> Self::Output {
        self.2.call((
            self.0.clone(),
            self.1.clone(),
            input,
            LiftEvaluate::<D>::lift_evaluate(self.0),
            LiftEvaluate::<D>::lift_evaluate(self.1),
        ))
    }
}
