pub trait MinMaxExt: Copy + Sized {
    fn mmin(&self, other: &Self) -> Self;

    fn mmax(&self, other: &Self) -> Self;

    fn mmin_op(&self, other: Option<Self>) -> Self {
        if let Some(other) = other {
            self.mmin(&other)
        } else {
            *self
        }
    }

    fn mmin_op2(a: Option<Self>, b: Option<Self>) -> Option<Self> {
        if let Some(a) = a {
            Some(a.mmin_op(b))
        } else {
            b
        }
    }

    fn mmax_op(&self, other: Option<Self>) -> Self {
        if let Some(other) = other {
            self.mmax(&other)
        } else {
            *self
        }
    }

    fn mmax_op2(a: Option<Self>, b: Option<Self>) -> Option<Self> {
        if let Some(a) = a {
            Some(a.mmax_op(b))
        } else {
            b
        }
    }
}

impl MinMaxExt for f32 {
    fn mmin(&self, other: &Self) -> Self {
        self.min(*other)
    }

    fn mmax(&self, other: &Self) -> Self {
        self.max(*other)
    }
}
