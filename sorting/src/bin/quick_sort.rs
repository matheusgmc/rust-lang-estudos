fn partition(list: &mut Vec<i32>, pivot_index: usize, end: usize) -> usize {
    let mut store_index = pivot_index + 1;
    println!("Pivot = {} | {}", pivot_index, list[pivot_index]);
    for i in pivot_index + 1..end {
        println!(
            "Compare {} <= {} is {}",
            list[i],
            list[pivot_index],
            list[i] <= list[pivot_index]
        );
        if list[i] <= list[pivot_index] {
            println!(
                "Swapping  {} - {} with element at {} - {}",
                i, list[i], store_index, list[store_index]
            );
            list.swap(i, store_index);
            store_index += 1;
            println!("store_index is now = {}", store_index);
        }
    }
    println!(
        "Swapping  {} - {} with element at {} - {}\n",
        pivot_index,
        list[pivot_index],
        store_index - 1,
        list[store_index - 1]
    );
    list.swap(pivot_index, store_index - 1);

    store_index - 1
}
fn quick_sort(list: &mut Vec<i32>, start: usize, end: usize) {
    if start < end {
        let pivot = partition(list, start, end);

        quick_sort(list, start, pivot);
        quick_sort(list, pivot + 1, end);
    }
}
fn main() {
    //let mut list = Vec::from([29, 10, 14, 37, 14]);
    let mut list = Vec::from([25, 40, 48, 20, 44, 35, 38, 22, 10, 25, 50]);

    let end = list.len();
    quick_sort(&mut list, 0, end);

    for i in list {
        print!("{} ", i);
    }
}
