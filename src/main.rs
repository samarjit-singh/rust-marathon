fn main() {
    // let ans = fib(4);
    let name = String::from("Samarjit");
    let len = get_str_len(name);
    println!("{} hello ji!! ji aaya nu",len);
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

fn get_str_len(str: String) -> usize {
    str.chars().count()  //implicit return
}