pub trait MinMaxExt: Copy + Sized {
    fn mmin(&self, other: &Self) -> Self;

    fn mmax(&self, other: &Self) -> Self;

    #[inline(always)]
    #[must_use]
    fn mmin_op(&self, other: Option<Self>) -> Self {
        if let Some(other) = other {
            self.mmin(&other)
        } else {
            *self
        }
    }

    #[inline(always)]
    #[must_use]
    fn mmin_op2(a: Option<Self>, b: Option<Self>) -> Option<Self> {
        if let Some(a) = a {
            Some(a.mmin_op(b))
        } else {
            b
        }
    }

    #[inline(always)]
    #[must_use]
    fn mmax_op(&self, other: Option<Self>) -> Self {
        if let Some(other) = other {
            self.mmax(&other)
        } else {
            *self
        }
    }

    #[inline(always)]
    #[must_use]
    fn mmax_op2(a: Option<Self>, b: Option<Self>) -> Option<Self> {
        if let Some(a) = a {
            Some(a.mmax_op(b))
        } else {
            b
        }
    }
}

impl MinMaxExt for f32 {
    #[inline(always)]
    #[must_use]
    fn mmin(&self, other: &Self) -> Self {
        self.min(*other)
    }

    #[inline(always)]
    #[must_use]
    fn mmax(&self, other: &Self) -> Self {
        self.max(*other)
    }
}
