use crate::order::{self, OrderEvent, OrderType};
use crate::portfolio::Portfolio;
use crate::util::{ExchangeablePair, Price};
use std::collections::HashSet;

pub type MarketEvent = HashSet<Price>;

pub fn market_event_step(
    market_event: MarketEvent,
    orders: &mut Vec<OrderEvent>,
    portfolio: &mut Portfolio,
) {
    for price in market_event.iter() {
        orders.retain(|order| !order::try_fill(price, order, portfolio));
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn market_event_step_market_executed() {
        let eps = 0.000001;
        let price = Price {
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };

        let order = OrderEvent {
            uuid: 1,
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::MarketOrder,
        };

        let market_event = HashSet::from([price]);
        let mut orders = vec![order];
        let mut portfolio = Portfolio::new();

        market_event_step(market_event, &mut orders, &mut portfolio);

        assert!(orders.is_empty());
        assert!((portfolio.get("AUD").unwrap() + 1.).abs() < eps);
        assert!((portfolio.get("MICROSOFT").unwrap() - 1.).abs() < eps);
    }

    #[test]
    fn market_event_step_market_not_executed() {
        let price = Price {
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };

        let order = OrderEvent {
            uuid: 1,
            ex: ExchangeablePair {
                tx: "USD",
                rx: "AMAZON",
            },
            amt: 1.,
            order_type: OrderType::MarketOrder,
        };

        let market_event = HashSet::from([price]);
        let mut orders = vec![order];
        let mut portfolio = Portfolio::new();

        market_event_step(market_event, &mut orders, &mut portfolio);

        assert_eq!(orders[0].uuid, 1);
        assert!(portfolio.is_empty());
    }

    #[test]
    fn market_event_step_limit_executed() {
        let eps = 0.000001;
        let price = Price {
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };

        let order = OrderEvent {
            uuid: 1,
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::LimitOrder(2.),
        };

        let market_event = HashSet::from([price]);
        let mut orders = vec![order];
        let mut portfolio = Portfolio::new();

        market_event_step(market_event, &mut orders, &mut portfolio);

        assert!(orders.is_empty());
        assert!((portfolio.get("AUD").unwrap() + 1.).abs() < eps);
        assert!((portfolio.get("MICROSOFT").unwrap() - 1.).abs() < eps);
    }

    #[test]
    fn market_event_step_limit_not_executed() {
        let price = Price {
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            value: 1.,
        };

        let order = OrderEvent {
            uuid: 1,
            ex: ExchangeablePair {
                tx: "AUD",
                rx: "MICROSOFT",
            },
            amt: 1.,
            order_type: OrderType::LimitOrder(0.5),
        };

        let market_event = HashSet::from([price]);
        let mut orders = vec![order];
        let mut portfolio = Portfolio::new();

        market_event_step(market_event, &mut orders, &mut portfolio);

        assert_eq!(orders[0].uuid, 1);
        assert!(portfolio.is_empty());
    }
}
