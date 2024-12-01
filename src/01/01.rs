use std::collections::BinaryHeap;

fn main() {
    let mut first_heap: BinaryHeap<usize> = BinaryHeap::new();
    let mut second_heap: BinaryHeap<usize> = BinaryHeap::new();

    include_bytes!("input.txt") //Wczytanie pliku
        .split(|b| b == &b'\n') //Podzielenie pliku na linijki
        .for_each(|line| { // Iteracja po każdej linijce
            let mut iterator = std::str::from_utf8(line).unwrap().split_whitespace(); //Podzielenie linijki na 2 liczby

            first_heap.push(iterator.next().unwrap().parse::<usize>().unwrap()); // Wyłuskanie 1 liczby oraz wrzucenie jej do pierwszej struktury
            second_heap.push(iterator.next().unwrap().parse::<usize>().unwrap()); // Wyłuskanie 2 liczby oraz wrzucenie jej do pierwszej struktury
        });
    let total_distance = (0..first_heap.len())
    .map(|_n| {
        first_heap.pop().unwrap().abs_diff(second_heap.pop().unwrap()) // Wyliczenie różnicy oraz wartość bezwględna
    }).sum::<usize>(); //Zsumowanie wszystkich odległości z każdej liniki
    print!("{}",total_distance);
}
