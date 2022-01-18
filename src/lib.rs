use std::collections::HashMap;

// type Data = Vec<f64>;
type Item = &'static str;
type Portfolio = HashMap<Item, f64>;

// enum Order {
//     MarketOrder,
//     LimitOrder,
// }

// #[derive(PartialEq, Eq, Hash)]
// enum Item {
//     Currency(&'static str),
//     Stock(&'static str),
// }

// impl PartialEq for Item {}

// impl Order {
//     fn submit_order(
//         order: Order,
//         give: Item,
//         receive: Item,
//         give_amt: f64,
//         receive_amt: f64,
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

fn trade(give: Item, receive: Item, give_amt: f64, receive_amt: f64, portfolio: &mut Portfolio) {
    let amt = portfolio.entry(give).or_insert(0.);
    *amt -= give_amt;

    let amt = portfolio.entry(receive).or_insert(0.);
    *amt += receive_amt;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trade_aud_bitcoin() {
        let mut portfolio = HashMap::from([("AUD", 1000.)]);
        trade("AUD", "BITCOIN", 100., 1., &mut portfolio);

        assert!((portfolio.get("AUD").unwrap() - 900.).abs() < 0.000001);
        assert!((portfolio.get("BITCOIN").unwrap() - 1.).abs() < 0.000001);
    }
}
