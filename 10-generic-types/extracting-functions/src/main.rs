fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let largest = get_largest(number_list);

    println!("The largest number is {}", largest);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let largest = get_largest(char_list);

    println!("The largest char is {}", largest);
}

fn get_largest<T: PartialOrd + Copy>(element_list: Vec<T>) -> T {
    let mut largest = element_list[0];
    for element in element_list {
        if element > largest {
            largest = element;
        }
    }
    largest
}
