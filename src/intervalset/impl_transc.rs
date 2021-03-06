use super::def::IntervalSet;

use crate::fp::Float;
use crate::transc::Transc;

impl<BOUND: Float> Transc for IntervalSet<BOUND> {
    type Output = Self;

    fn log(mut self) -> Self::Output {
        Self::from_intervals(self.intervals.drain(..).map(|i| i.log()).collect())
    }

    fn exp(mut self) -> Self::Output {
        Self::from_intervals(self.intervals.drain(..).map(|i| i.exp()).collect())
    }

    fn pow(self, rhs: Self) -> Self::Output {
        self.binary_op(rhs, |i, j| i.pow_multi(j))
    }

    fn sin(mut self) -> Self::Output {
        Self::from_intervals(self.intervals.drain(..).map(|i| i.sin()).collect())
    }

    fn cos(mut self) -> Self::Output {
        Self::from_intervals(self.intervals.drain(..).map(|i| i.cos()).collect())
    }

    fn tan(self) -> Self::Output {
        self.clone().sin() / self.cos()
    }

    fn sqrt(mut self) -> Self::Output {
        Self::from_intervals(self.intervals.drain(..).map(|i| i.sqrt()).collect())
    }

    fn abs(mut self) -> Self::Output {
        Self::from_intervals(self.intervals.drain(..).map(|i| i.abs()).collect())
    }

    fn signum(self) -> Self::Output {
        self.unary_op(|x| x.signum_multi())
    }
}
