use crate::portfolio::{self, Portfolio};
use crate::util::{ExchangeablePair, Price, Quantity, UUID};

#[derive(Debug)]
pub enum OrderType {
    MarketOrder,
    LimitOrder(Quantity),
}

#[derive(Debug)]
pub struct OrderEvent {
    pub uuid: UUID,
    pub ex: ExchangeablePair,
    pub amt: Quantity,
    pub order_type: OrderType,
}

impl PartialEq for OrderEvent {
    fn eq(&self, other: &OrderEvent) -> bool {
        self.uuid == other.uuid
    }
}

fn is_fill(
    Price {
        ex: price_ex,
        value,
    }: &Price,
    OrderEvent {
        ex: order_ex,
        order_type,
        ..
    }: &OrderEvent,
) -> bool {
    match order_type {
        OrderType::MarketOrder => price_ex == order_ex,
        OrderType::LimitOrder(limit) => price_ex == order_ex && value <= limit,
    }
}

pub fn try_fill(
    price @ Price { value, .. }: &Price,
    order @ OrderEvent {
        ex: order_ex, amt, ..
    }: &OrderEvent,
    portfolio: &mut Portfolio,
) -> bool {
    if is_fill(price, order) {
        portfolio::trade(order_ex, value * amt, *amt, portfolio);
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn is_fill_true_market() {
        let order = &OrderEvent {
            uuid: 1,
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::MarketOrder,
        };
        let price = &Price {
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };
        assert!(is_fill(price, order));
    }

    #[test]
    fn is_fill_false_market() {
        let order = &OrderEvent {
            uuid: 1,
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::MarketOrder,
        };
        let price = &Price {
            ex: ExchangeablePair {
                tx: "USD",
                rx: "AMAZON",
            },
            value: 1.,
        };
        assert!(!is_fill(price, order));
    }

    #[test]
    fn is_fill_true_limit() {
        let order = &OrderEvent {
            uuid: 1,
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::LimitOrder(2.),
        };
        let price = &Price {
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };
        assert!(is_fill(price, order));
    }

    #[test]
    fn is_fill_false_limit() {
        let order = &OrderEvent {
            uuid: 1,
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::LimitOrder(0.5),
        };
        let price = &Price {
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };
        assert!(!is_fill(price, order));
    }

    #[test]
    fn try_fill_true_market() {
        let eps = 0.000001;

        let order = &OrderEvent {
            uuid: 1,
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::MarketOrder,
        };
        let price = &Price {
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };
        let mut portfolio = Portfolio::new();

        assert!(try_fill(price, order, &mut portfolio));
        assert!((portfolio.get("AUD").unwrap() + 1.).abs() < eps);
        assert!((portfolio.get("MICROSOFT").unwrap() - 1.).abs() < eps);
    }

    #[test]
    fn try_fill_false_market() {
        let order = &OrderEvent {
            uuid: 1,
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::MarketOrder,
        };
        let price = &Price {
            ex: ExchangeablePair {
                tx: "USD",
                rx: "AMAZON",
            },
            value: 1.,
        };
        let mut portfolio = Portfolio::new();

        assert!(!try_fill(price, order, &mut portfolio));
        assert!(portfolio.is_empty());
    }

    #[test]
    fn try_fill_true_limit() {
        let eps = 0.000001;

        let order = &OrderEvent {
            uuid: 1,
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::LimitOrder(2.),
        };
        let price = &Price {
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };
        let mut portfolio = Portfolio::new();

        assert!(try_fill(price, order, &mut portfolio));
        assert!((portfolio.get("AUD").unwrap() + 1.).abs() < eps);
        assert!((portfolio.get("MICROSOFT").unwrap() - 1.).abs() < eps);
    }

    #[test]
    fn try_fill_false_limit() {
        let order = &OrderEvent {
            uuid: 1,
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::LimitOrder(0.5),
        };
        let price = &Price {
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };
        let mut portfolio = Portfolio::new();

        assert!(!try_fill(price, order, &mut portfolio));
        assert!(portfolio.is_empty());
    }
}
