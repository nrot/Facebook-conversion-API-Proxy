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