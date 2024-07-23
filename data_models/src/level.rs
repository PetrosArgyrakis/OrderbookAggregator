use std::fmt;

/// Represents a single <Price, Amount> pair.
#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub struct Level {
    pub price: f64,
    pub amount: f64,
}

impl Level {
    /// Constructs a new [Level].
    ///
    /// # Arguments
    ///
    /// * `price` - The [`f64`] value of the price.
    /// * `amount` - The [`f64`] value of the amount.
    pub fn new(price: f64, amount: f64) -> Self {
        Level { price, amount }
    }
}


impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(p={}, s={})", self.price, self.amount)
    }
}

impl fmt::Debug for Level {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Level")
            .field("p ", &self.price)
            .field("a ", &self.amount)
            .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;
    use crate::exchange_level::Level;

    #[test]
    fn order_eq() {
        let l1 = Level::new(1.0, 1.0);
        let l2 = Level::new(1.0, 1.0);
        assert_eq!(l1.partial_cmp(&l2).unwrap(), Ordering::Equal);
    }

    #[test]
    fn order_gt() {
        let l1 = Level::new(1.0, 2.0);
        let l2 = Level::new(1.0, 1.0);
        assert_eq!(l1.partial_cmp(&l2).unwrap(), Ordering::Greater);

        let l1 = Level::new(2.0, 1.0);
        let l2 = Level::new(1.0, 1.0);
        assert_eq!(l1.partial_cmp(&l2).unwrap(), Ordering::Greater);

        let l1 = Level::new(2.0, 2.0);
        let l2 = Level::new(1.0, 1.0);
        assert_eq!(l1.partial_cmp(&l2).unwrap(), Ordering::Greater);
    }

    #[test]
    fn order_lt() {
        let l1 = Level::new(1.0, 1.0);
        let l2 = Level::new(2.0, 1.0);
        assert_eq!(l1.partial_cmp(&l2).unwrap(), Ordering::Less);

        let l1 = Level::new(1.0, 1.0);
        let l2 = Level::new(1.0, 2.0);
        assert_eq!(l1.partial_cmp(&l2).unwrap(), Ordering::Less);

        let l1 = Level::new(1.0, 1.0);
        let l2 = Level::new(2.0, 2.0);
        assert_eq!(l1.partial_cmp(&l2).unwrap(), Ordering::Less);
    }
}