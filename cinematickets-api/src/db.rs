

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use diesel::dsl::{IntervalDsl, now};
use crate::models::*;
use crate::schema;
use crate::models;
use schema::cities::dsl::*;

use diesel::prelude::*;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_available_cities(conn: &PgConnection) -> Result<Vec<City>, diesel::result::Error> {
    use schema::cities;
    cities.load::<models::City>(conn)
}

pub fn movies_in_schedule_for_city(conn: &PgConnection, city_name: &str) -> Result<Vec<(i64, String, String)>, diesel::result::Error> {
    use schema::cities::dsl::*;
    use schema::cities;
    use schema::movies;
    use schema::cinemaplaces;
    use schema::cinemaplaces::dsl::*;
    use schema::cinemarooms;
    use schema::cinemarooms::dsl::*;
    use schema::cinemasessions;
    use schema::cinemasessions::dsl::*;
    use schema::movies::dsl::*;
    cinemasessions
        .filter(starts_at.gt(now - 24.hours()))
        .filter(starts_at.lt(now + 24.hours()))
        .inner_join(movies)
        .inner_join(cities)
        .filter(cities::name.eq(city_name))
        .select((movies::id, movies::name, movies::picture_url))
        .distinct()
        .load(conn)
}

pub fn sessions_for_movie_in_city(conn: &PgConnection, movie_name: &str, city_name: &str) -> Result<Vec<FullCinemaSession>, diesel::result::Error> {
    use schema::cities::dsl::*;
    use schema::cities;
    use schema::movies;
    use schema::cinemaplaces;
    use schema::cinemaplaces::dsl::*;
    use schema::cinemarooms;
    use schema::cinemarooms::dsl::*;
    use schema::cinemasessions;
    use schema::cinemasessions::dsl::*;
    use schema::movies::dsl::*;
    let q = cinemasessions
        .inner_join(movies)
        .inner_join(cinemaplaces)
        .inner_join(cinemarooms)
        .inner_join(cities)
        .filter(movies::name.eq(movie_name))
        .filter(cities::name.eq(city_name))
        .filter(cinemasessions::starts_at.lt(now + 60.hours()))
        .filter(cinemasessions::starts_at.gt(now - 60.hours()))
        .select((movies::name, cinemasessions::id, cinemaplaces::name, movies::picture_url, cinemasessions::starts_at, cinemaplaces::address, cinemasessions::available_seats, cinemarooms::name));
    q.load(conn)
}