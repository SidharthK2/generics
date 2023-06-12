fn main() {
    let mut number_list = vec![34, 50, 25, 100, 65];

    let largest = largest_num(&mut number_list);

    println!("The largest number is {:?}", number_list);
}

fn largest_num<T>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for num in list {
        if num > largest {
            largest = num;
        }
    }
    largest
}
