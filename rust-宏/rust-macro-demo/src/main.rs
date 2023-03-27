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

macro_rules! my_struct {
    ($name:ident, $field_name:ident : $field_type:ty) => {
        #[derive(Debug)]
        struct $name {
            $field_name: $field_type,
        }
    };
}
// #[derive(Debug)]
// struct MyStruct {
//     name: String,
// }

fn main() {
    println!("Hello, world!, {}", format!("xxx"));
    prnt_ln!("xxx");
    my_struct!(MyStruct, my_field: i32);
    print!(
        "{:?}",
        MyStruct {
            // name: "saber".to_string(),
            my_field: 1
        }
    )
    // prnt_ln!("xxx", "xxxaaa");
}
