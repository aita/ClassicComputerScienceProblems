#[derive(Debug)]
struct Item<'a> {
    name: &'a str,
    weight: usize,
    value: f32,
}

fn knapsack<'a>(items: &'a [Item], max_capacity: usize) -> Vec<&'a Item<'a>> {
    let mut table = vec![vec![0.0; max_capacity + 1]; items.len() + 1];
    for (i, item) in items.iter().enumerate() {
        for capacity in 1..max_capacity + 1 {
            let previous_item_value = table[i][capacity];
            if capacity >= item.weight {
                let value_freeing_weight_for_item = table[i][capacity - item.weight];
                table[i + 1][capacity] =
                    (value_freeing_weight_for_item + item.value).max(previous_item_value);
            } else {
                table[i + 1][capacity] = previous_item_value;
            }
        }
    }
    let mut solution = Vec::new();
    let mut capacity = max_capacity;
    for i in (1..items.len() + 1).rev() {
        if table[i - 1][capacity] != table[i][capacity] {
            solution.push(&items[i - 1]);
            capacity -= items[i - 1].weight;
        }
    }
    solution
}

fn main() {
    let items = vec![
        Item {
            name: "television",
            weight: 50,
            value: 500.0,
        },
        Item {
            name: "candlesticks",
            weight: 2,
            value: 300.0,
        },
        Item {
            name: "stereo",
            weight: 35,
            value: 400.0,
        },
        Item {
            name: "laptop",
            weight: 3,
            value: 1000.0,
        },
        Item {
            name: "food",
            weight: 15,
            value: 50.0,
        },
        Item {
            name: "clothing",
            weight: 20,
            value: 800.0,
        },
        Item {
            name: "jewelry",
            weight: 1,
            value: 4000.0,
        },
        Item {
            name: "books",
            weight: 100,
            value: 300.0,
        },
        Item {
            name: "printer",
            weight: 18,
            value: 30.0,
        },
        Item {
            name: "refrigerator",
            weight: 200,
            value: 700.0,
        },
        Item {
            name: "painting",
            weight: 10,
            value: 1000.0,
        },
    ];
    println!("{:?}", knapsack(&items, 75));
}
