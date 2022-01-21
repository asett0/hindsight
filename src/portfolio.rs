use crate::util::{ExchangeablePair, Item, Quantity};
use std::collections::HashMap;

pub type Portfolio = HashMap<Item, Quantity>;

pub fn trade(
    ExchangeablePair { tx, rx }: &ExchangeablePair,
    tx_amt: f64,
    rx_amt: f64,
    portfolio: &mut Portfolio,
) {
    let amt = portfolio.entry(tx).or_insert(0.);
    *amt -= tx_amt;

    let amt = portfolio.entry(rx).or_insert(0.);
    *amt += rx_amt;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn long_aud_bitcoin() {
        let eps = 0.000001;

        let mut portfolio = HashMap::from([("USD", 1000.)]);
        let ex = &ExchangeablePair {
            tx: "USD",
            rx: "BITCOIN",
        };
        trade(ex, 100., 1., &mut portfolio);

        assert!((portfolio.get("USD").unwrap() - 900.).abs() < eps);
        assert!((portfolio.get("BITCOIN").unwrap() - 1.).abs() < eps);
    }

    #[test]
    fn short_aud_microsoft() {
        let eps = 0.000001;

        let mut portfolio = HashMap::new();
        let ex = &ExchangeablePair {
            tx: "AUD",
            rx: "MICROSOFT",
        };
        trade(ex, 100., 1., &mut portfolio);

        assert!((portfolio.get("AUD").unwrap() + 100.).abs() < eps);
        assert!((portfolio.get("MICROSOFT").unwrap() - 1.).abs() < eps);
    }
}
