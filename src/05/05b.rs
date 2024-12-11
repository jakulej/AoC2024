use std::collections::HashMap;

fn main() {
    let (ordering_rules, pages_to_produce) = include_str!("input.txt").split_once("\r\n\r\n").unwrap();

    let mut number_orders: HashMap<usize, Vec<usize>> = HashMap::new();
    ordering_rules.split("\r\n").for_each(|line| {
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
        .split("\r\n")
        .filter_map(|line| {
            let updates: Vec<usize> = line
                .split(",")
                .map(|update| update.parse::<usize>().unwrap())
                .collect();

            match !is_sorted(&updates, &number_orders) {
                true => {
                    print!("\n");
                    print!("Not sorted: {:?}, ",updates);
                    let sorted = &sort_orders(number_orders.clone(), updates);
                    let middle_elemment = get_middle_element(sorted);
                    println!("after sort: {:?}, middle element: {:?}",sorted,middle_elemment);
                    Some(middle_elemment)
                },
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
fn sort_orders(number_orders: HashMap<usize, Vec<usize>>, current_updates: Vec<usize>) -> Vec<usize> {
    let mut sorted_vaules: Vec<usize> = current_updates.clone();

    for i in 0..current_updates.len() {
        for j in 0..i {
            let update = sorted_vaules[i];
            let previous_update = sorted_vaules[j];
            let is_current_update_sorted = match  number_orders.get(&update) {
                Some(x) => !x.contains(&previous_update),
                None => true,
            };

            if !is_current_update_sorted {
                println!("{:?} should be after {:?}",update,previous_update);
                println!("Removing value: {:?}, i {:?}, pushing to: {:?}",update,i,j);
                let deleted = sorted_vaules.remove(i);
                sorted_vaules.insert(j, deleted);
                println!("Actual table: {:?}",sorted_vaules);
            }
        }
    }
    sorted_vaules
}

fn sort_update(mut updates: Vec<usize>, updates_order: Vec<usize>) -> Vec<usize> {
    updates.sort_by_key(|value_to_sort| updates_order.iter().position(|x| x == value_to_sort));
    updates
}
