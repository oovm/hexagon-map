use hexagon_map::{AxialPoint, HexagonMap};

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let map = HexagonMap::<bool>::width_first(3, 4, true);
    for (p, maze) in map.points_all() {
        println!("{p}: {maze}")
    }
}

#[test]
fn test_action_field() {
    let map = HexagonMap::<bool>::width_first(3, 4, true);
    let cost = map.action_field(AxialPoint::new(0, 0), 10.0).with_cost(|p, _| (p.q + p.p).abs() as f64);
    for (p, maze) in cost.solve() {
        println!("{p}: {maze}")
    }
}

#[test]
fn test_path() {
    let map = HexagonMap::<bool>::rhombus(3, 4);
    let (path, cost) = map
        .path_finder(AxialPoint::new(0, 0), AxialPoint::new(1, -2))
        .with_cost(|p, _| (p.q + p.p).abs() as f64)
        .solve_points();
    for point in path {
        println!("{point}: {cost}")
    }
}
