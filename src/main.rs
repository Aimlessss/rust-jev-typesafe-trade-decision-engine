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
    let sellOrder = Order {
        id : 2,
        symbol : "AAPL".to_string(),
        side : Side::Sell,
        quantity : 10,
        price_cents : 150000
    };

    let mut orders : Vec<Order> = Vec::new();
    orders.push(order);
    orders.push(sellOrder);
    for currOrder in &orders {
        let side_text = match currOrder.side{
            Side::BUY => "BUY",
            Side::SELL => "SELL"
        }
        let currOrderQunat = currOrder.value_cents();
        println!("Order {}, shares {} with quantity {}, to {} with quant {}", currOrder.id, currOrder.symbol, currOrder.quantity, currOrder.side, currOrderQunat);
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
