fn main() {
    let mut number_list = vec![34, 50, 25, 100, 65];

    // let mut largest = &number_list[0];

    // for number in &number_list {
    //     if number > largest {
    //         largest = number;
    //     }
    // }
    let a = g1(&mut number_list);
    println!("{:p} {:p}", &number_list[0], &a);
}

fn g1(thing: &mut Vec<i32>) -> i32 {
    let tmp = thing;

    tmp[0] += 1;
    tmp[0]
}
