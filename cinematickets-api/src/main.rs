use std::collections::HashMap;
use std::ops::Deref;
use std::sync::{Arc, Mutex};
use std::time::SystemTime;
use diesel::{RunQueryDsl, pg::Pg};
use diesel::dsl::{IntervalDsl, now};

#[macro_use]
extern crate diesel;
extern crate dotenv;

mod schema;
mod models;
mod db;
use schema::cities::dsl::*;

use diesel::prelude::*;
use crate::diesel::associations::HasTable;
use models::*;

use db::*;

fn data_registering() {
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
    let conn = db::establish_connection();
    // let res = cities.limit(5).load::<models::City>(&conn);
    // println!("{:?}", res.unwrap());
    // let new_city = NewCity {name: "São Paulo", state: "São Paulo"};
    // let _x: City = diesel::insert_into(cities::table).values(new_city).get_result(&conn).unwrap();

    // let new_movie = NewMovie {name: "O Batman", launch_date: std::time::SystemTime::now(), picture_url: "https://external-content.duckduckgo.com/iu/?u=https%3A%2F%2Fbamsmackpow.com%2Fwp-content%2Fblogs.dir%2F308%2Ffiles%2F2019%2F06%2FBatman_Landscape.jpg&f=1&nofb=1"};
    // let x: Movie = diesel::insert_into(movies).values(new_movie).get_result(&conn).unwrap();
    // println!("created movie: {:?}",x);

    // let new_cinemaplace = NewCinemaPlace {name: "Cinemark Eldorado", address: "Shopping iblablilu", city_id: 1};
    // let x: CinemaPlace = diesel::insert_into(cinemaplaces).values(new_cinemaplace).get_result(&conn).unwrap();
    // println!("created cinemaplace: {:?}",x);

    // let new_cinemaroom = NewCinemaRoom {name: "1", cinemaplace_id: 41, city_id: 1, total_seats: 200};
    // let x: CinemaRoom = diesel::insert_into(cinemarooms).values(new_cinemaroom).get_result(&conn).unwrap();
    // println!("created cinemaroom: {:?}",x);

    // let new_cinemasession = NewCinemaSession {movie_id: 1, cinemaroom_id: 1, starts_at: std::time::SystemTime::now(),cinemaplace_id: 1, city_id: 1, available_seats: 200};
    // let x: CinemaSession = diesel::insert_into(cinemasessions).values(new_cinemasession).get_result(&conn).unwrap();
    // println!("created cinemasession: {:?}",x);
    let cities_res = get_available_cities(&conn).unwrap();
    println!("available cities: {:?}", &cities_res);

    let res: Vec<(i64, String, String)> = movies_in_schedule_for_city(&conn, "São Paulo").unwrap();
    println!("movies in sao paulo: {:?}", res);

    let res: Vec<FullCinemaSession> = sessions_for_movie_in_city(&conn, "O Batman", "São Paulo").unwrap();
    println!("batman sessions in sao paulo: {:?}", res);
}
use warp::Filter;
#[macro_use]
use tokio;
#[macro_use]
use tokio::main;
use crate::db::establish_connection;

mod routes;

#[tokio::main]
async fn main() {

    // FIXME use https://crates.io/crates/deadpool-diesel
    let conns = Arc::new(Mutex::new(establish_connection()));
    let with_conn =  warp::any().map(move|| conns.clone());

    let citiez = warp::path("cities") // 3.
        .and(with_conn.clone())
        .map(|conns: Arc<Mutex<PgConnection>>|  {
            // let conn = &db::establish_connection();
            let conn = conns.lock().unwrap();
            let conn = conn.deref();
            warp::reply::json(&db::get_available_cities(conn).unwrap())
        });
    use serde::{Deserialize, Serialize};
    #[derive(Deserialize, Serialize)]
    struct MoviesQuery {
        city: String
    }
    let movies_on_city = warp::path!("movies")
        .and(warp::query::<MoviesQuery>())
        .and(with_conn.clone())
        .map(|q: MoviesQuery, conns: Arc<Mutex<PgConnection>>| {
            let conn = conns.lock().unwrap();
            let conn = conn.deref();
            warp::reply::json(&db::movies_in_schedule_for_city(conn, &q.city).unwrap())
        });

    #[derive(Deserialize, Serialize)]
    struct SessionsQuery {
        movie: String,
        city: String
    }

    let movie_sessions = warp::path!("sessions")
        .and(warp::query::<SessionsQuery>())
        .and(with_conn.clone())
        .map(|q: SessionsQuery, conns: Arc<Mutex<PgConnection>>| {
            let conn = conns.lock().unwrap();
            let conn = conn.deref();
            warp::reply::json(&db::sessions_for_movie_in_city(conn, &q.movie, &q.city).unwrap())
        });
    let routes = citiez.or(movies_on_city).or(movie_sessions);

    warp::serve(routes) // 5.
        .run(([0, 0, 0, 0], 8000)) // 6.
        .await;
}