struct User {  // similar to interface in typescript
    first_name: String,
    last_name: String,
    age: i32
}


fn main() {
    let user = User {
        first_name: String::from("Samarjit"),
        last_name: String::from("Singh"),
        age:22
    };

    println!("name: {} {} age: {}", user.first_name, user.last_name, user.age);
}


// fn is_even(num: i32) -> bool {  //u32, i64, u64 i is signed number, u is unsigned number (i means + - )
//     if num % 2 == 0 {
//         return true
//     }

//     return false
// }


// fn fib(num: i32) -> i32 {
//      let mut first = 0; // mutable variable
//      let mut second = 1;

//      if num == 0 {
//         return first;
//      }

//      if num == 1{
//         return second;
//      }

//      for _ in 0..num-1 {
//         let temp = second;
//         second = second+first;
//         first = temp;
//      }

//      return second;
// }

// fn get_str_len(str: String) -> usize {
//     str.chars().count()  //implicit return
// }