use std::collections::HashMap;

fn main() {
    let (ordering_rules, pages_to_produce) = include_str!("input.txt").split_once("\n\n").unwrap();

    let mut number_orders: HashMap<usize, Vec<usize>> = HashMap::new();
    ordering_rules.split("\n").for_each(|line| {
        let (first_digit, second_digit) = line.split_once("|").unwrap_or(("0", "0"));
        number_orders
            .entry(first_digit.parse::<usize>().unwrap())
            .and_modify(|vector| vector.push(second_digit.parse::<usize>().unwrap()))
            .or_insert_with(|| {
                let mut vec: Vec<usize> = Vec::new();
                vec.push(second_digit.parse::<usize>().unwrap());
                vec
            });
    });

    let score = pages_to_produce
        .split("\n")
        .filter_map(|line| {
            let updates: Vec<usize> = line
                .split(",")
                .map(|update| update.parse::<usize>().unwrap())
                .collect();

            match is_sorted(&updates, &number_orders) {
                true => Some(get_middle_element(&updates)),
                false => None,
            }
        })
        .sum::<usize>();
    println!("{:?}", score);
}

fn is_sorted(updates: &Vec<usize>, updates_order: &HashMap<usize, Vec<usize>>) -> bool {
    updates.iter().enumerate().all(|(i, validating_update)| {
        updates.iter().take(i).all(
            |previous_update| match updates_order.get(validating_update) {
                Some(x) => !x.contains(previous_update),
                None => false,
            },
        )
    })
}
fn get_middle_element(updates: &Vec<usize>) -> usize {
    let middle_index = updates.len().div_ceil(2) - 1;
    *updates.get(middle_index).unwrap()
}

//NOT NEEDED
fn sort_orders(number_orders: HashMap<usize, Vec<usize>>) -> Vec<usize> {
    let mut sorted_vaules: Vec<usize> = Vec::new();
    println!("{:?}", number_orders.keys());
    while sorted_vaules.len() != number_orders.len() {
        let key_wtih_no_occurence = number_orders
            .keys()
            .filter(|key| !sorted_vaules.contains(key))
            .find(|next_key_to_append| {
                number_orders
                    .iter()
                    .filter(|(key, _value)| !sorted_vaules.contains(key))
                    .all(|(_key, value)| value.contains(next_key_to_append))
            });
        println!("{:?}", key_wtih_no_occurence);
        sorted_vaules.push(*key_wtih_no_occurence.unwrap());
    }
    sorted_vaules.push(
        *number_orders
            .get(sorted_vaules.last().unwrap())
            .unwrap()
            .get(0)
            .unwrap(),
    );
    sorted_vaules
}

fn sort_update(mut updates: Vec<usize>, updates_order: Vec<usize>) -> Vec<usize> {
    updates.sort_by_key(|value_to_sort| updates_order.iter().position(|x| x == value_to_sort));
    updates
}
