use hexagon_map::HexagonMap;

#[test]
fn ready() {
    println!("it works!")
}


#[test]
fn test() {
    let map = HexagonMap::<bool>::rhombus(5, 5);
    for (p, maze) in &map {
        println!("{p}: {maze}")
    }
    for p in map.points() {
        println!("{p:?}")
    }
}