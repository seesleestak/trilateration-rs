#[derive(Debug)]
struct Beacon {
    x: f32,
    y: f32,
    distance: f32,
}

fn calc(beacons: [Beacon; 3]) {
    println!("beacons --- {:#?}", beacons);
}

fn main() {
    let beacons: [Beacon; 3] = [
        Beacon {
            x: 2.0,
            y: 4.0,
            distance: 5.7,
        },
        Beacon {
            x: 5.5,
            y: 13.0,
            distance: 6.8,
        },
        Beacon {
            x: 11.5,
            y: 2.0,
            distance: 6.4,
        },
    ];

    calc(beacons)
}
