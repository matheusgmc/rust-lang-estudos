fn main() {
    let mut list = Vec::from([37, 48, 8, 18, 21, 32, 35, 37, 36, 2, 33, 33]);

    for i in 0..list.len() {
        let mut current_min = i;
        for j in i + 1..list.len() {
            println!(
                "compare {} < {} is {}",
                list[j],
                list[current_min],
                list[j] < list[current_min]
            );
            if list[j] < list[current_min] {
                println!("set {} as the new current_min", j);
                current_min = j;
            }
        }

        println!(
            "swap the current_min {} with the first unsorted element {}\n",
            list[current_min], list[i]
        );
        list.swap(i, current_min);
    }

    for i in list {
        print!("{} ", i);
    }
}
