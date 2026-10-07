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
    let mut book = OrderBook {
        orders: Vec::new(),
    };

    let order_accepted = book.add_order(order);
    let order_accepted_sell = book.add_order(sell_order);

    println!(" orders {}, {}", order_accepted, order_accepted_sell);

    for curr_order in &book.orders {
        let side_text = match curr_order.side {
            Side::Buy => "BUY",
            Side::Sell => "SELL",
        };
        let curr_order_qunat = curr_order.value_cents();
        
        println!("Order {}, shares {} with quantity {}, to {} with quant {}, is true{}", curr_order.id, curr_order.symbol, curr_order.quantity, side_text, curr_order_qunat, curr_order.is_valid());
    };
    println!("lenght {}", book.orders.len());
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


impl OrderBook { 
    fn add_order(&self, order : Order) -> bool {
        if !order.is_valid() {
            return false;
        }
        self.orders.push(order);
        return true;
    }
}
