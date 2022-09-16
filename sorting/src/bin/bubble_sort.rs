fn main() {
    let mut list = [40, 55, 11, 32, 67, 5, 74, 89, 38, 66, 27, 36, 79, 99, 2];

    for _i in 0..list.len() {
        for j in 0..list.len() - 1 {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            };
        }
    }

    for i in list {
        print!("{} ", i);
    }
}
