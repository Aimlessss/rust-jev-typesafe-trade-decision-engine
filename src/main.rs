enum Side {
    Buy, Sell
}

struct Order {
    symbol : String, 
    side : Side, 
    quantity : u32,
    price_cents : u64
}

fn main(){
    let order = Order {
        symbol : "AAPL".to_string(),
        side : Side::Buy,
        quantity : 10,
        price_cents: 15000
    };
    let side_text = match order.side {
        Side::Buy => "BUY",
        Side::Sell => "SELL"
    };
    let order_val = u64::from(order.quantity) * order.price_cents;


    println!("Symbol: {} {} {} {}", side_text, order.quantity, order.symbol, order.price_cents);
    println!("At price: {} cents", order_val);
}

impl Order {
    fn 
}