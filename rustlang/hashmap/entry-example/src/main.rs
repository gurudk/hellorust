#![allow(unused)]
fn main() {
    use std::collections::HashMap;

// type inference lets us omit an explicit type signature (which
// would be `HashMap<&str, u8>` in this example).
    let mut player_stats = HashMap::new();

    fn random_stat_buff() -> u16 {
        // could actually return some random value here - let's just return
        // some fixed value for now
        42
    }

// insert a key only if it doesn't already exist

    player_stats.insert("health",100);
    player_stats.insert("health",111);
    player_stats.insert("health",444);
    // if let Some(x) = player_stats.get_mut("health"){
    //     *x = 200;
    // }else {
    //     player_stats.entry("health").or_insert(300);
    // }

// insert a key using a function that provides a new value only if it
// doesn't already exist
    player_stats.entry("defence").or_insert_with(random_stat_buff);

    println!("{:?}", player_stats);

// update a key, guarding against the key possibly not being set
    let stat = player_stats.entry("attack").or_insert(100);
    *stat += random_stat_buff();
    println!("{:?}", player_stats);

// modify an entry before an insert with in-place mutation
    player_stats.entry("mana").and_modify(|mana| *mana += 200).or_insert(100);
    player_stats.entry("mana").and_modify(|mana| *mana += 200).or_insert(100);
    println!("{:?}", player_stats);
}