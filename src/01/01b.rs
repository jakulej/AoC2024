use std::collections::HashMap;

fn main() {
    let mut first_numbers: Vec<usize> = Vec::new();
    let mut second_numbers_occurrences: HashMap<usize, usize> = HashMap::new();

    include_bytes!("input.txt") //Wczytanie pliku
        .split(|b| b == &b'\n') //Podzielenie pliku na linijki
        .for_each(|line| {
            // Iteracja po każdej linijce
            let mut iterator = std::str::from_utf8(line).unwrap().split_whitespace(); //Podzielenie linijki na 2 liczby

            first_numbers.push(iterator.next().unwrap().parse::<usize>().unwrap()); // Wyłuskanie 1 liczby oraz wrzucenie jej do pierwszej struktury
            second_numbers_occurrences
                .entry(iterator.next().unwrap().parse::<usize>().unwrap())
                .and_modify(|counter| *counter += 1)
                .or_insert(1);
        });
    let total_distance = first_numbers
        .iter()
        .map(|n| {
            n * second_numbers_occurrences.get(n).or(Some(&0)).unwrap()
        })
        .sum::<usize>(); //Zsumowanie wszystkich odległości z każdej liniki
    print!("{}", total_distance);
}
