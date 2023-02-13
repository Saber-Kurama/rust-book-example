macro_rules! add {
    // macth like arm for macro
    ($a:expr,$b:expr) => {
        // macro expand to this code
        {
            // $a and $b will be templated using the value/variable provided to macro
            $a + $b
        }
    };
}

// macro_rules! attr {
//     (($($attr:tt)*), $($item:tt)+) => {
//         $(#$attr)* $($item)+
//     };
// }

// macro_rules! once {
//     (($($tts:tt)*) $($tail:tt)*) => {
//         $($tts)*
//     };
// }

macro_rules! prnt_ln {
    ($($arg:tt)*) => {
        println!("hello {}", format!($($arg)*));
    };
}
fn main() {
    println!("Hello, world!, {}", format!("xxx", "sss"));
    prnt_ln!("xxx");
    // prnt_ln!("xxx", "xxxaaa");
}
