#![feature(proc_macro_hygiene, decl_macro)]

use clap::{App, Arg};
use rand::Rng;

fn random_coordinates(x0: f64, y0: f64, radius: f64) -> (f64, f64) {
    let mut rng = rand::thread_rng();
    let random = 2. * std::f64::consts::PI * rng.gen_range(0.0..1.);
    let r = radius * (rng.gen_range(0.0..1.) as f64).sqrt();

    (r * random.cos() + x0, r * random.sin() + y0)
}

fn get_random_point(
    start_location: geoutils::Location,
    radius: i64,
) -> Result<(geoutils::Location, geoutils::Distance), anyhow::Error> {
    loop {
        let random_coordinates = random_coordinates(
            start_location.latitude(),
            start_location.longitude(),
            (1. / 27.) * radius as f64 / 3500.,
        );
        let destination = geoutils::Location::new(random_coordinates.0, random_coordinates.1);

        match start_location
            .is_in_circle(&destination, geoutils::Distance::from_meters(radius as f64))
        {
            Ok(res) => {
                if res {
                    let distance = start_location.distance_to(&destination);

                    match distance {
                        Ok(dist) => return Ok((destination, dist)),
                        Err(err) => {
                            return Err(anyhow::anyhow!(
                                "Error at calculating distance between two points: {}",
                                err
                            ))
                        }
                    }
                }
            }
            Err(err) => {
                log::info!("Error at `in_radius` function: {}", err);
            }
        }
    }
}

fn main() {
    let matches = App::new("Random Location")
        .about("It takes random coordinates within circle radius")
        .arg(
            Arg::with_name("current_location")
                .short("l")
                .long("location")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("radius")
                .short("r")
                .long("radius")
                .takes_value(true),
        )
        .get_matches();

    let coordinates = matches
        .value_of("current_location")
        .unwrap()
        .split(',')
        .collect::<Vec<&str>>();
    let radius = matches.value_of("radius").unwrap().parse::<i64>().unwrap();

    let latitude = coordinates[0].parse::<f64>().unwrap();
    let longitude = coordinates[1].parse::<f64>().unwrap();

    let my_location = geoutils::Location::new(latitude, longitude);

    match get_random_point(my_location, radius) {
        Ok(tuple) => println!(
            "Lat and Lon: {:?}\nDistance: {}\nGoogle URL: {}",
            tuple.0,
            tuple.1,
            format!(
                "https://www.google.com/maps/dir/?api=1&destination={},{}",
                tuple.0.latitude(),
                tuple.0.longitude()
            )
        ),
        Err(err) => log::info!("Error at `get_random_point` function: {}", err),
    }
}
