use crate::fill::FillEvent;
use crate::market::MarketEvent;
use crate::order::OrderEvent;

enum DefaultEvent {
    Market(MarketEvent),
    Order(OrderEvent),
    Fill(FillEvent),
}

enum Event<T> {
    Default(DefaultEvent),
    Custom(T),
}
