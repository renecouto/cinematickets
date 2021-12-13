use diesel::Queryable;
#[macro_use]
use serde::{Serialize, Deserialize};

#[derive(Queryable, Debug,  Serialize, Deserialize)]
pub struct City {
    pub id: i64,
    pub name: String,
    pub state: String,
}


use super::schema::cities;

#[derive(Insertable)]
#[table_name="cities"]
pub struct NewCity<'a> {
    pub name: &'a str,
    pub state: &'a str,
}

use super::schema::movies;

#[derive(Queryable, Debug)]
pub struct Movie {
    pub id: i64,
    pub name: String,
    pub launch_date: std::time::SystemTime,
    pub picture_url: String,
}

#[derive(Insertable)]
#[table_name="movies"]
pub struct NewMovie<'a> {
    pub name: &'a str,
    pub launch_date: std::time::SystemTime,
    pub picture_url: &'a str,
}

use super::schema::cinemaplaces;

#[derive(Queryable, Debug)]
pub struct CinemaPlace {
    pub id: i64,
    pub name: String,
    pub city_id: i64,
    pub address: String,
}


#[derive(Insertable)]
#[table_name="cinemaplaces"]
pub struct NewCinemaPlace<'a> {
    pub name: &'a str,
    pub city_id: i64,
    pub address: &'a str,
}

use super::schema::cinemarooms;

#[derive(Queryable, Debug)]
pub struct CinemaRoom {
    pub id: i64,
    pub name: String,
    pub total_seats: i32,
    pub cinemaplace_id: i64,
    pub city_id: i64,
}


#[derive(Insertable)]
#[table_name="cinemarooms"]
pub struct NewCinemaRoom<'a> {
    pub name: &'a str,
    pub total_seats: i32,
    pub cinemaplace_id: i64,
    pub city_id: i64,
}

use super::schema::cinemasessions;

#[derive(Queryable, Debug)]
pub struct CinemaSession {
    pub id: i64,
    pub cinemaplace_id: i64,
    pub city_id: i64,
    pub cinemaroom_id: i64,
    pub movie_id: i64,
    pub starts_at: std::time::SystemTime,
    pub available_seats: i32,
}


#[derive(Insertable)]
#[table_name="cinemasessions"]
pub struct NewCinemaSession {
    pub cinemaplace_id: i64,
    pub city_id: i64,
    pub cinemaroom_id: i64,
    pub movie_id: i64,
    pub starts_at: std::time::SystemTime,
    pub available_seats: i32,
}

#[derive(Queryable, Debug, Serialize, Deserialize)]
pub struct FullCinemaSession {
    pub movie_name: String,
    pub cinemasession_id: i64,
    pub cinemaplace_name: String,
    pub movie_picture_url: String,
    pub cinemasession_starts_at: std::time::SystemTime,
    pub cinemaplace_address: String,
    pub cinemasession_available_seats: i32,
    pub cinemaroom_name: String,
}