use serde_json::Value;
use crate::core::MessageType;

mod core;
pub mod models;




 pub fn deserialize(value: &Value) -> MessageType {
     core::init(value)
 }



/*pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}*/
