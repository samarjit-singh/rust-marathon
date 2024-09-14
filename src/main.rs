fn main() {
    let ans = fib(4);
    println!("{} hello ji!! ji aaya nu",ans);
}


// fn is_even(num: i32) -> bool {  //u32, i64, u64 i is signed number, u is unsigned number (i means + - )
//     if num % 2 == 0 {
//         return true
//     }

//     return false
// }


fn fib(num: i32) -> i32 {
     let mut first = 0; // mutable variable
     let mut second = 1;

     if num == 0 {
        return first;
     }

     if num == 1{
        return second;
     }

     for _ in 0..num-1 {
        let temp = second;
        second = second+first;
        first = temp;
     }

     return second;
}