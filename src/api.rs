#[macro_export]
macro_rules! dbgs {
    () => {
        
    };
    ($($arg:tt)*)=>{
        if cfg!(debug_assertions){
            println!($($arg)*);
        };
    }
}

use serde_json::{Value, Result};


#[cfg(test)]
mod tests {
    #[test]
    fn serde_research(){
        use serde_json::{Value, Result};
        let templ_json = r#"{
            "obj": {
                "key": "value",
                "int": 255,
                "arr": [
                    0,
                    1
                ]
            }
        }"#;
        let val: Result<Value> = serde_json::from_str(templ_json);
        match val {
            Ok(res)=>{},
            Err(e)=>{}
        }

    }
}