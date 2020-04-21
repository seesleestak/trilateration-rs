#[derive(Debug)]
struct Beacon {
    x: f32,
    y: f32,
    d: f32,
}

#[derive(Debug)]
struct Location {
    x: f32,
    y: f32,
}

fn calc(b: [Beacon; 3]) -> Location {
    let k: f32 = ((b[0].x).powi(2) + (b[0].y).powi(2)
        - (b[1].x).powi(2)
        - (b[1].y).powi(2)
        - (b[0].d).powi(2)
        + (b[1].d).powi(2))
        / (2.0 * (b[0].y - b[1].y))
        - ((b[0].x).powi(2) + (b[0].y).powi(2)
            - (b[2].x).powi(2)
            - (b[2].y).powi(2)
            - (b[0].d).powi(2)
            + (b[2].d).powi(2))
            / (2.0 * (b[0].y - b[2].y));

    let j: f32 = ((b[2].x - b[0].x) / (b[0].y - b[2].y)) - ((b[1].x - b[0].x) / (b[0].y - b[1].y));

    let x: f32 = k / j;

    let y: f32 = ((b[1].x - b[0].x) / (b[0].y - b[1].y)) * x
        + ((b[0].x).powi(2) + (b[0].y).powi(2)
            - (b[1].x).powi(2)
            - (b[1].y).powi(2)
            - (b[0].d).powi(2)
            + (b[1].d).powi(2))
            / (2.0 * (b[0].y - b[1].y));

    Location { x, y }
}

fn main() {
    let beacons: [Beacon; 3] = [
        Beacon {
            x: 2.0,
            y: 4.0,
            d: 5.7,
        },
        Beacon {
            x: 5.5,
            y: 13.0,
            d: 6.8,
        },
        Beacon {
            x: 11.5,
            y: 2.0,
            d: 6.4,
        },
    ];

    let location: Location = calc(beacons);
    println!("location --- {:#?}", location);
}
