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
    let order = Order {
        id : 1,
        symbol : "AAPL".to_string(),
        side : Side::Buy,
        quantity : 10,
        price_cents: 150000
    };
    let side_text = match order.side {
        Side::Buy => "BUY",
        Side::Sell => "SELL"
    };
    let order_val = order.value_cents();
    let isValid = order.is_valid();


    println!("Symbol: {} {} {} {}", side_text, order.quantity, order.symbol, order.price_cents);
    println!("At price: {} cents", order_val);
    println!("Is valid {}", isValid);
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
