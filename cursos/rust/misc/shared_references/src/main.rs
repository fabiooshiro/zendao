use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
pub struct MyId {
    id: u8,
}

#[derive(Debug)]
pub struct SharedData<'a> {
    pub my_id: &'a MyId,
}

mod private_items {
    use super::*;
    static mut ITEMS: Lazy<HashMap<u8, SharedData>> = Lazy::new(|| {
        let m: HashMap<u8, SharedData> = HashMap::new();
        m
    });
    pub struct StateHolder {}
    impl StateHolder {
        pub fn get(id: u8) -> Option<&'static SharedData<'static>> {
            unsafe { ITEMS.get(&id) }
        }
        pub fn put(id: u8, data: SharedData<'static>) {
            unsafe {
                ITEMS.insert(id, data);
            }
        }
    }
}

fn main() {
    // single thread
    static mut MY_ID: MyId = MyId { id: 2 };
    let data = unsafe {
        SharedData { my_id: &MY_ID }
    };
    private_items::StateHolder::put(2, data);

    let x1 = private_items::StateHolder::get(1);
    println!("x1 = {:?}", &x1);
    let x2 = private_items::StateHolder::get(2);
    println!("x2 = {:?}", &x2);
    let x3 = private_items::StateHolder::get(2);
    unsafe {
        MY_ID = MyId { id: 3 };
    }
    println!("x3 = {:?}", &x3);
    let inner_data = x3.unwrap();
    assert_eq!(inner_data.my_id.id, 3);
}
