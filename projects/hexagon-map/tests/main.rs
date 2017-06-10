use hexagon_map::{AxialPoint, CubePoint, HexagonMap, IsometricLine};
use itertools::Itertools;

#[test]
fn ready() {
    println!("it works!")
}

#[test]
fn test() {
    let map = HexagonMap::<bool>::width_first(3, 4, true);
    for (p, maze) in map.points_all() {
        println!("{p:?}: {maze}")
    }
}

#[test]
fn test_circuit() {
    let circuit = IsometricLine::new(CubePoint::new(0, 0), 0);
    assert_eq!(circuit.collect_vec(), vec![AxialPoint::new(0, 0)], "Circuit with radius 0 should contain only one point");
    let circuit = IsometricLine::new(CubePoint::new(1, 1), 1);
    assert_eq!(
        circuit.collect_vec(),
        vec![
            AxialPoint::new(1, 2),
            AxialPoint::new(2, 1),
            AxialPoint::new(1, 1),
            AxialPoint::new(1, 0),
            AxialPoint::new(0, 1),
            AxialPoint::new(1, 1)
        ],
        "Circuit with radius 1 should contain 6 points"
    );
}

#[test]
fn test_action_field() {
    let map = HexagonMap::<bool>::width_first(3, 4, true);
    let cost = map.action_field(CubePoint::new(0, 0), 10.0).with_cost(|p, _| (p.q + p.p).abs() as f64);
    for (p, maze) in cost.solve() {
        println!("{p}: {maze}")
    }
}

#[test]
fn test_path() {
    let map = HexagonMap::<bool>::rhombus(3, 4);
    let (path, cost) =
        map.path_finder(CubePoint::new(0, 0), CubePoint::new(1, -2)).with_cost(|p, _| (p.q + p.p).abs() as f64).solve_points();
    for point in path {
        println!("{point}: {cost}")
    }
}
