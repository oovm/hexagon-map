use hexagon_map::HexagonMap;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test() {
    let map = HexagonMap::<bool>::width_first(3, 4, true);
    for (p, maze) in &map {
        println!("{p}: {maze}")
    }
    for p in map.points() {
        println!("{p:?}")
    }
}