/// Evaluation context containing Position, Distance and Color
use t_funk::collection::set::{Get, Insert, Remove};

use crate::{Color, Distance, PosDist, Position};

#[derive(Debug, Default, Copy, Clone)]
pub struct PosDistColor<P, D, C> {
    pub pos_dist: PosDist<P, D>,
    pub color: C,
}

impl<P, D, C> Get<Position<P>> for PosDistColor<Position<P>, D, C> {
    fn get(self) -> Position<P> {
        self.pos_dist.get()
    }
}

impl<P, D, C> Get<Distance<D>> for PosDistColor<P, Distance<D>, C> {
    fn get(self) -> Distance<D> {
        self.pos_dist.get()
    }
}

impl<P, D, C> Get<Color<C>> for PosDistColor<P, D, Color<C>> {
    fn get(self) -> Color<C> {
        self.color
    }
}

impl<PA, PB, D, C> Insert<Position<PB>> for PosDistColor<PA, D, C> {
    type Insert = PosDistColor<Position<PB>, D, C>;

    fn insert(self, t: Position<PB>) -> Self::Insert {
        let PosDistColor { pos_dist, color } = self;
        PosDistColor {
            pos_dist: pos_dist.insert(t),
            color,
        }
    }
}

impl<P, DA, DB, C> Insert<Distance<DB>> for PosDistColor<P, DA, C> {
    type Insert = PosDistColor<P, Distance<DB>, C>;

    fn insert(self, t: Distance<DB>) -> Self::Insert {
        let PosDistColor { pos_dist, color } = self;
        PosDistColor {
            pos_dist: pos_dist.insert(t),
            color,
        }
    }
}

impl<P, D, CA, CB> Insert<Color<CB>> for PosDistColor<P, D, CA> {
    type Insert = PosDistColor<P, D, Color<CB>>;

    fn insert(self, color: Color<CB>) -> Self::Insert {
        let PosDistColor { pos_dist, .. } = self;
        PosDistColor { color, pos_dist }
    }
}

impl<P, D, C> Remove<Position<P>> for PosDistColor<Position<P>, D, C> {
    type Remove = PosDistColor<(), D, C>;

    fn remove(self) -> (Self::Remove, Position<P>) {
        let PosDistColor { pos_dist, color } = self;
        let (pos_dist, p) = pos_dist.remove();
        (PosDistColor { pos_dist, color }, p)
    }
}

impl<P, D, C> Remove<Distance<D>> for PosDistColor<P, Distance<D>, C> {
    type Remove = PosDistColor<P, (), C>;

    fn remove(self) -> (Self::Remove, Distance<D>) {
        let PosDistColor { pos_dist, color } = self;
        let (pos_dist, d) = pos_dist.remove();
        (PosDistColor { pos_dist, color }, d)
    }
}

impl<P, D, C> Remove<Color<C>> for PosDistColor<P, D, Color<C>> {
    type Remove = PosDistColor<P, D, ()>;

    fn remove(self) -> (Self::Remove, Color<C>) {
        let PosDistColor { pos_dist, color } = self;

        (
            PosDistColor {
                pos_dist,
                color: (),
            },
            color,
        )
    }
}