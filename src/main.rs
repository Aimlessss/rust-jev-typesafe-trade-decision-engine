use std::sync::{Arc, Mutex};
use std::thread;

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
    let book = Arc::new(Mutex::new(OrderBook {
        orders: Vec::new(),
    }));
    let mut handles = Vec::new();
    for _ in 0..2 {
        let shared_book = Arc::clone(&book);
        let handle = thread::spawn(move || {
            let order = Order {
                id : 1,
                symbol : "AAPL".to_string(),
                side : Side::Buy,
                quantity : 10,
                price_cents: 150000
            };
            let accepted = {
                let mut guard = shared_book.lock().unwrap();
                guard.add_order(order);
            };
            return accepted;
        });
        handles.push(handle);
    }

    for handle in handles {
        let accepted = handle.join().unwrap();
        println!("Accepted: {}", accepted);
    }
    let guard = book.lock().unwrap();
    println!("Orders in book: {}", guard.orders.len());

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
    fn add_order(&mut self, order : Order) -> bool {
        if !order.is_valid() {
            return false;
        }
        for exisiting_orders in &self.orders {
            if order.id == exisiting_orders.id {
                return false;
            }
        }
        self.orders.push(order);
        return true;
    }

    fn canc_order(&mut self, id : u64) -> bool {
        for index in 0..self.orders.len() {
            if self.orders[index].id == id {
                self.orders.remove(index);
                return true;
            }
        }
        return false;
    }
}
