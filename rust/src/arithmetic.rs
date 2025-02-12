use std::ops::Mul;

pub trait HasE9: Copy + Mul<Self, Output = Self> {
    const E9: Self;

    #[must_use]
    fn e9(self) -> Self {
        self * Self::E9
    }
}

macro_rules! has_e9 {
    ($t:ty, $ten:expr) => {
        impl HasE9 for $t {
            const E9: $t = $ten.pow(9);
        }
    };
}

has_e9!(u64, 10u64);
