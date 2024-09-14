fn main() {
    let ans = is_even(11);
    println!("{} hello ji!! ji aaya nu",ans);
}


fn is_even(num: i32) -> bool {  //u32, i64, u64 i is signed number, u is unsigned number (i means + - )
    if num % 2 == 0 {
        return true
    }

    return false
}