use std::collections::{HashMap, HashSet};

type Item = &'static str;
type Quantity = f64;
type Portfolio = HashMap<Item, Quantity>;
type Tick = HashSet<Price>;
type UUID = u32;

#[derive(PartialEq, Eq)]
struct ExchangablePair {
    tx: Item,
    rx: Item,
}

// Price struct which represents how much tx you need to spend for one unit of rx
struct Price {
    ex: ExchangablePair,
    amt: Quantity,
}

enum Order {
    MarketOrder { ex: ExchangablePair, amt: Quantity },
    LimitOrder { price: Price, amt: Quantity },
}

// #[derive(PartialEq, Eq, Hash)]
// enum Item {
//     Currency(&'static str),
//     Stock(&'static str),
// }

// impl PartialEq for Item {}

// impl Order {
//     fn submit_order(
//         order: Order,
//         tx: Item,
//         rx: Item,
//         tx_amt: f64,
//         rx_amt: f64,
//         portfolio: &mut Portfolio,
//     ) {
//         match order {
//             MarketOrder => {
//                 let count = portfolio.entry(word).or_insert(0);
//                 *count += 1;
//             }
//             _ => panic!("Invalid order provided"),
//         }
//     }
// }

fn trade(
    ExchangablePair { tx, rx }: ExchangablePair,
    tx_amt: f64,
    rx_amt: f64,
    portfolio: &mut Portfolio,
) {
    let amt = portfolio.entry(tx).or_insert(0.);
    *amt -= tx_amt;

    let amt = portfolio.entry(rx).or_insert(0.);
    *amt += rx_amt;
}

fn fill(tick: Tick, orders: &mut HashMap<UUID, Order>, portfolio: &mut Portfolio) {
    let filled_orders = Vec::new();

    for (uuid, order) in orders.iter() {
        match order {
            Order::MarketOrder {
                ex: order_ex,
                amt: order_amt,
            } => {
                for Price {
                    ex: price_ex,
                    amt: price_amt,
                } in tick
                {
                    if price_ex == order_ex {
                        trade(order_ex, price_amt * order_amt, order_amt, portfolio);
                        filled_orders.push(&order);
                    }
                }
            }
            Order::LimitOrder {
                price:
                    Price {
                        ex: order_ex,
                        amt: limit,
                    },
                amt: order_amt,
            } => {
                for Price {
                    ex: price_ex,
                    amt: price_amt,
                } in tick
                {
                    if (price_ex == order_ex) && (price_amt <= limit) {
                        trade(order_ex, price_amt * order_amt, order_amt, portfolio);
                        filled_orders.push(&order);
                    }
                }
            }
        }
    }
}

fn is_fill(
    Price {
        ex: price_ex,
        amt: price_amt,
    }: Price,
    order: Order,
) -> bool {
    match order {
        Order::MarketOrder { ex: order_ex, .. } => price_ex == order_ex,
        Order::LimitOrder {price: Price {ex: order_ex, amt: order_amt},.. } =>}
    }
}
// fn backtest(ticks: Vec<HashMap<Item, f64>>) {
//     let mut orders: HashSet<Order> = HashSet::new();

//     for tick in ticks {
//         MarketOrder
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trade_aud_bitcoin() {
        let mut portfolio = HashMap::from([("AUD", 1000.)]);
        let ex = ExchangablePair {
            tx: "AUD",
            rx: "BITCOIN",
        };
        trade(ex, 100., 1., &mut portfolio);

        assert!((portfolio.get("AUD").unwrap() - 900.).abs() < 0.000001);
        assert!((portfolio.get("BITCOIN").unwrap() - 1.).abs() < 0.000001);
    }

    #[test]
    fn short_aud_microsoft() {
        let mut portfolio = HashMap::new();
        let ex = ExchangablePair {
            tx: "AUD",
            rx: "MICROSOFT",
        };
        trade(ex, 100., 1., &mut portfolio);

        assert!((portfolio.get("AUD").unwrap() + 100.).abs() < 0.000001);
        assert!((portfolio.get("MICROSOFT").unwrap() - 1.).abs() < 0.000001);
    }
}
