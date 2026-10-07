enum Side {
    Buy, Sell
}

struct Order {
    id : u64,
    symbol : String, 
    side : Side, 
    quantity : u32,
    price_cents : u64
}

struct OrderBook {
    orders : Vec<Order>
}
fn main(){
    //buy order
    let order = Order {
        id : 1,
        symbol : "AAPL".to_string(),
        side : Side::Buy,
        quantity : 10,
        price_cents: 150000
    };

    //sell order
    let sell_order = Order {
        id : 2,
        symbol : "AAPL".to_string(),
        side : Side::Sell,
        quantity : 10,
        price_cents : 150000
    };

    let mut orders : OrderBook = Vec::new();
    orders.push(order);
    orders.push(sell_order);
    for curr_order in &orders {
        let side_text = match curr_order.side {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        };
        let curr_order_qunat = curr_order.value_cents();
        
        println!("Order {}, shares {} with quantity {}, to {} with quant {}", curr_order.id, curr_order.symbol, curr_order.quantity, side_text, curr_order_qunat);
    };
}

impl Order {
    fn value_cents(&self) -> u64 {
        let order_val = u64::from(self.quantity) * self.price_cents;
        return order_val;
    }
    fn is_valid(&self) -> bool {
        self.quantity > 0 && self.price_cents > 0
    }
}
