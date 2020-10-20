#![allow(dead_code)]
use std::collections::HashMap;
use std::ops::{Add, Div, Mul};

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
enum Currency {
    Doller,
    Franc,
}

#[derive(Debug, PartialEq)]
struct Money<T>(Vec<(Currency, T)>);

impl<T> Money<T>
where
    T: Copy + Mul<Output = T>,
{
    pub fn doller(amount: T) -> Self {
        Self(vec![(Currency::Doller, amount)])
    }
    pub fn franc(amount: T) -> Self {
        Self(vec![(Currency::Franc, amount)])
    }
    pub fn times(&self, times: T) -> Self {
        Money(self.0.iter().copied().map(|i| (i.0, i.1 * times)).collect())
    }
}

impl<T> Add for Money<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let mut lhs = self;
        let mut rhs = rhs;
        (&mut lhs.0).append(&mut rhs.0);
        lhs
    }
}

struct Bank<T> {
    rates: HashMap<(Currency, Currency), T>,
}

impl<T> Bank<T>
where
    T: Copy + Add<Output = T> + Default + Mul<Output = T> + Div<Output = T>,
{
    pub fn new() -> Self {
        Bank {
            rates: Default::default(),
        }
    }
    pub fn reduce(&self, money: Money<T>, to: Currency) -> Money<T> {
        let sum = money
            .0
            .iter()
            .copied()
            .map(|(currency, amount)| {
                if let Some(exchanged_amount) = self.exchange(amount, currency, to) {
                    return exchanged_amount;
                }
                panic!("Can't convert the amount");
            })
            .fold(T::default(), |acc, v| acc + v);
        Money(vec![(to, sum)])
    }
    fn exchange(&self, amount: T, from: Currency, to: Currency) -> Option<T> {
        if from == to {
            return Some(amount);
        }
        if let Some(rate) = self.rates.get(&(from, to)) {
            return Some(amount / *rate);
        }
        if let Some(rate) = self.rates.get(&(to, from)) {
            return Some(amount * *rate);
        }
        None
    }
    pub fn add_rate(&mut self, from: Currency, to: Currency, rate: T) {
        if self.rates.get(&(from, to)).is_none() && self.rates.get(&(to, from)).is_none() {
            self.rates.insert((from, to), rate);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiplication() {
        let five = Money::doller(5);
        assert_eq!(Money::doller(10), five.times(2));
        assert_eq!(Money::doller(15), five.times(3));
    }

    #[test]
    fn test_equality() {
        assert_eq!(Money::doller(5), Money::doller(5));
        assert_ne!(Money::doller(6), Money::doller(5));
        assert_ne!(Money::franc(5), Money::doller(5));
    }

    //    #[test]
    //    fn test_currency() {
    //        assert_eq!("USD", Money::doller(1).currency());
    //        assert_eq!("CHF", Money::franc(1).currency());
    //    }
    //
    #[test]
    fn test_simple_addition() {
        let five = Money::doller(5);
        let five2 = Money::doller(5);
        let sum = five + five2;
        let bank = Bank::new();
        let reduced = bank.reduce(sum, Currency::Doller);
        assert_eq!(Money::doller(10), reduced);
    }

    //    #[test]
    //    fn test_reduce_sum() {
    //        let sum = Sum::new(Money::Doller(3), Money::Doller(4));
    //        let bank = Bank::new();
    //        let result = bank.reduce(sum, "USD");
    //        assert_eq!(Money::Doller(7), result);
    //    }
    //
    //    #[test]
    //    fn test_reduce_money() {
    //        let bank = Bank::new();
    //        let result = bank.reduce(Money::Doller(1), "USD");
    //        assert_eq!(Money::Doller(1), result);
    //    }
    //
    #[test]
    fn test_reduce_money_diferrenct_currency() {
        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Doller, 2);
        let result = bank.reduce(Money::franc(2), Currency::Doller);
        assert_eq!(Money::doller(1), result);
        let result = bank.reduce(Money::doller(6), Currency::Franc);
        assert_eq!(Money::franc(12), result);
    }

    //    #[test]
    //    fn test_identity_rate() {
    //        assert_eq!(Bank::<u32>::new().rate("USD", "USD"), None);
    //    }
    //
    #[test]
    fn test_mixed_addition() {
        let five_bucks = Money::doller(5);
        let ten_francs = Money::franc(10);
        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Doller, 2);
        let result = bank.reduce(five_bucks + ten_francs, Currency::Doller);
        assert_eq!(Money::doller(10), result);
    }

    #[test]
    fn test_sum_plus_money() {
        let five_bucks = Money::doller(5);
        let five_bucks2 = Money::doller(5);
        let ten_francs = Money::franc(10);
        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Doller, 2);
        let sum = five_bucks + ten_francs + five_bucks2;
        let result = bank.reduce(sum, Currency::Doller);
        assert_eq!(Money::doller(15), result);
    }

    #[test]
    fn test_sum_times() {
        let five_bucks = Money::doller(5);
        let ten_francs = Money::franc(10);
        let mut bank = Bank::new();
        bank.add_rate(Currency::Franc, Currency::Doller, 2);
        let sum = (five_bucks + ten_francs).times(2);
        let result = bank.reduce(sum, Currency::Doller);
        assert_eq!(Money::doller(20), result);
    }
}
