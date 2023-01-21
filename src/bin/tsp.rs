use itertools::Itertools;
use lazy_static::lazy_static;
use maplit::hashmap;
use std::collections::HashMap;

lazy_static! {
    static ref VT_DISTANCES: HashMap<&'static str, HashMap<&'static str, u32>> = hashmap! {
        "Rutland" => hashmap!{
            "Burlington" => 67,
            "White River Junction" => 46,
            "Bennington" => 55,
            "Brattleboro" => 75},
        "Burlington" => hashmap!{
            "Rutland" => 67,
            "White River Junction" => 91,
            "Bennington" => 122,
            "Brattleboro" => 153},
        "White River Junction" => hashmap!{
            "Rutland" => 46,
            "Burlington" => 91,
            "Bennington" => 98,
            "Brattleboro" => 65},
        "Bennington" => hashmap!{
            "Rutland" => 55,
            "Burlington" => 122,
            "White River Junction" => 98,
            "Brattleboro" => 40},
        "Brattleboro" => hashmap!{
            "Rutland" => 75,
            "Burlington" => 153,
            "White River Junction" => 65,
            "Bennington" => 40}
    };
    static ref VT_CITIES: Vec<&'static str> = VT_DISTANCES.keys().cloned().collect();
    static ref CITY_PERMUTATIONS: Vec<Vec<&'static str>> = VT_CITIES
        .iter()
        .cloned()
        .permutations(VT_CITIES.len())
        .collect();
    static ref TSP_PATHS: Vec<Vec<&'static str>> = CITY_PERMUTATIONS
        .iter()
        .map(|path| {
            let mut path = path.clone();
            path.push(path[0]);
            path
        })
        .collect();
}

fn main() {
    let mut best_path = None;
    let mut min_distance = u32::MAX;
    for path in TSP_PATHS.iter() {
        let mut distance = 0;
        let mut last = path[0];
        for next in path.iter().skip(1) {
            distance += VT_DISTANCES[last][next];
            last = next;
        }
        if distance < min_distance {
            min_distance = distance;
            best_path = Some(path);
        }
    }
    println!(
        "The shortest path is {:?} in {} miles.",
        best_path.unwrap(),
        min_distance
    );
}
