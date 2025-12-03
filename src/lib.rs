pub trait NumDigits {
    fn num_digits(&self) -> Self;
}

impl NumDigits for u64 {
    fn num_digits(&self) -> Self {
        (self.ilog10() + 1) as u64
    }
}
