
fn main() {

    let array = [10_u32, 2_u32];
    let total = sum(&array);
    match total {
        Some(c) => println!("Sum array total ={}", c),
        None => println!("Sum overflow")
    }

    let array1 = [u32::MAX, 1_u32];
    let total1 = sum(&array1);
    match total1 {
        Some(c) => println!("Sum array total ={}", c),
        None => println!("Sum overflow")
    }
}

///Array sum
fn sum(array: &[u32]) -> Option<u32> {
    array.iter().try_fold(0_u32, |acc, &x| acc.checked_add(x))
}