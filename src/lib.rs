use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
};

type Item = &'static str;
type Quantity = f64;
type Portfolio = HashMap<Item, Quantity>;
type MarketEvent = HashSet<Price>;
type UUID = u32;

#[derive(PartialEq, Eq, Hash, Debug)]
struct ExchangablePair {
    tx: Item,
    rx: Item,
}

struct Price {
    ex: ExchangablePair,
    value: Quantity,
}

impl PartialEq for Price {
    fn eq(&self, other: &Price) -> bool {
        self.ex == other.ex
    }
}

impl Eq for Price {}

impl Hash for Price {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.ex.hash(state)
    }
}

#[derive(Debug)]
struct Order {
    uuid: UUID,
    ex: ExchangablePair,
    amt: Quantity,
    order_type: OrderType,
}

impl PartialEq for Order {
    fn eq(&self, other: &Order) -> bool {
        self.uuid == other.uuid
    }
}

#[derive(Debug)]
enum OrderType {
    MarketOrder,
    LimitOrder(Quantity),
}

fn trade(
    ExchangablePair { tx, rx }: &ExchangablePair,
    tx_amt: f64,
    rx_amt: f64,
    portfolio: &mut Portfolio,
) {
    let amt = portfolio.entry(tx).or_insert(0.);
    *amt -= tx_amt;

    let amt = portfolio.entry(rx).or_insert(0.);
    *amt += rx_amt;
}

fn is_fill(
    Price {
        ex: price_ex,
        value,
    }: &Price,
    Order {
        ex: order_ex,
        order_type,
        ..
    }: &Order,
) -> bool {
    match order_type {
        OrderType::MarketOrder => price_ex == order_ex,
        OrderType::LimitOrder(limit) => price_ex == order_ex && value <= limit,
    }
}

fn try_fill(
    price @ Price {
        ex: price_ex,
        value,
    }: &Price,
    order @ Order {
        ex: order_ex, amt, ..
    }: &Order,
    portfolio: &mut Portfolio,
) -> bool {
    if is_fill(price, order) {
        trade(order_ex, value * amt, *amt, portfolio);
        true
    } else {
        false
    }
}

fn market_event_step(
    market_event: MarketEvent,
    orders: &mut Vec<Order>,
    portfolio: &mut Portfolio,
) {
    for price in market_event.iter() {
        orders.retain(|order| !try_fill(price, order, portfolio));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trade_aud_bitcoin() {
        let eps = 0.000001;

        let mut portfolio = HashMap::from([("USD", 1000.)]);
        let ex = &ExchangablePair {
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
        let ex = &ExchangablePair {
            tx: "AUD",
            rx: "MICROSOFT",
        };
        trade(ex, 100., 1., &mut portfolio);

        assert!((portfolio.get("AUD").unwrap() + 100.).abs() < eps);
        assert!((portfolio.get("MICROSOFT").unwrap() - 1.).abs() < eps);
    }

    #[test]
    fn is_fill_true_market() {
        let order = &Order {
            uuid: 1,
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::MarketOrder,
        };
        let price = &Price {
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };
        assert!(is_fill(price, order));
    }

    #[test]
    fn is_fill_false_market() {
        let order = &Order {
            uuid: 1,
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::MarketOrder,
        };
        let price = &Price {
            ex: ExchangablePair {
                tx: "USD",
                rx: "AMAZON",
            },
            value: 1.,
        };
        assert!(!is_fill(price, order));
    }

    #[test]
    fn is_fill_true_limit() {
        let order = &Order {
            uuid: 1,
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::LimitOrder(2.),
        };
        let price = &Price {
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };
        assert!(is_fill(price, order));
    }

    #[test]
    fn is_fill_false_limit() {
        let order = &Order {
            uuid: 1,
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::LimitOrder(0.5),
        };
        let price = &Price {
            ex: ExchangablePair {
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

        let order = &Order {
            uuid: 1,
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::MarketOrder,
        };
        let price = &Price {
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };
        let mut portfolio = HashMap::new();

        assert!(try_fill(price, order, &mut portfolio));
        assert!((portfolio.get("AUD").unwrap() + 1.).abs() < eps);
        assert!((portfolio.get("MICROSOFT").unwrap() - 1.).abs() < eps);
    }

    #[test]
    fn try_fill_false_market() {
        let order = &Order {
            uuid: 1,
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::MarketOrder,
        };
        let price = &Price {
            ex: ExchangablePair {
                tx: "USD",
                rx: "AMAZON",
            },
            value: 1.,
        };
        let mut portfolio = HashMap::new();

        assert!(!try_fill(price, order, &mut portfolio));
        assert!(portfolio.is_empty());
    }

    #[test]
    fn try_fill_true_limit() {
        let eps = 0.000001;

        let order = &Order {
            uuid: 1,
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::LimitOrder(2.),
        };
        let price = &Price {
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };
        let mut portfolio = HashMap::new();

        assert!(try_fill(price, order, &mut portfolio));
        assert!((portfolio.get("AUD").unwrap() + 1.).abs() < eps);
        assert!((portfolio.get("MICROSOFT").unwrap() - 1.).abs() < eps);
    }

    #[test]
    fn try_fill_false_limit() {
        let order = &Order {
            uuid: 1,
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::LimitOrder(0.5),
        };
        let price = &Price {
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };
        let mut portfolio = HashMap::new();

        assert!(!try_fill(price, order, &mut portfolio));
        assert!(portfolio.is_empty());
    }

    #[test]
    fn market_event_step_market_executed() {
        let eps = 0.000001;
        let price = Price {
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };

        let order = Order {
            uuid: 1,
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::MarketOrder,
        };

        let market_event = HashSet::from([price]);
        let mut orders = vec![order];
        let mut portfolio = HashMap::new();

        market_event_step(market_event, &mut orders, &mut portfolio);

        assert!(orders.is_empty());
        assert!((portfolio.get("AUD").unwrap() + 1.).abs() < eps);
        assert!((portfolio.get("MICROSOFT").unwrap() - 1.).abs() < eps);
    }

    #[test]
    fn market_event_step_market_not_executed() {
        let price = Price {
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };

        let order = Order {
            uuid: 1,
            ex: ExchangablePair {
                tx: "USD",
                rx: "AMAZON",
            },
            amt: 1.,
            order_type: OrderType::MarketOrder,
        };

        let market_event = HashSet::from([price]);
        let mut orders = vec![order];
        let mut portfolio = HashMap::new();

        market_event_step(market_event, &mut orders, &mut portfolio);

        assert_eq!(orders[0].uuid, 1);
        assert!(portfolio.is_empty());
    }

    #[test]
    fn market_event_step_limit_executed() {
        let eps = 0.000001;
        let price = Price {
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };

        let order = Order {
            uuid: 1,
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::LimitOrder(2.),
        };

        let market_event = HashSet::from([price]);
        let mut orders = vec![order];
        let mut portfolio = HashMap::new();

        market_event_step(market_event, &mut orders, &mut portfolio);

        assert!(orders.is_empty());
        assert!((portfolio.get("AUD").unwrap() + 1.).abs() < eps);
        assert!((portfolio.get("MICROSOFT").unwrap() - 1.).abs() < eps);
    }

    #[test]
    fn market_event_step_limit_not_executed() {
        let price = Price {
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };

        let order = Order {
            uuid: 1,
            ex: ExchangablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::LimitOrder(0.5),
        };

        let market_event = HashSet::from([price]);
        let mut orders = vec![order];
        let mut portfolio = HashMap::new();

        market_event_step(market_event, &mut orders, &mut portfolio);

        assert_eq!(orders[0].uuid, 1);
        assert!(portfolio.is_empty());
    }
}
