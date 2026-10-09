use crate::Order;

pub(crate) enum Command {
    AddOrder(Order),
    CancelOrder(u64),
}