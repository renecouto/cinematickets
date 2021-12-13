table! {
    cinemaplaces (id) {
        id -> Int8,
        name -> Text,
        city_id -> Int8,
        address -> Text,
    }
}

table! {
    cinemarooms (id) {
        id -> Int8,
        name -> Text,
        total_seats -> Int4,
        cinemaplace_id -> Int8,
        city_id -> Int8,
    }
}

table! {
    cinemasessions (id) {
        id -> Int8,
        cinemaplace_id -> Int8,
        city_id -> Int8,
        cinemaroom_id -> Int8,
        movie_id -> Int8,
        starts_at -> Timestamp,
        available_seats -> Int4,
    }
}

table! {
    cities (id) {
        id -> Int8,
        name -> Text,
        state -> Text,
    }
}

table! {
    movies (id) {
        id -> Int8,
        name -> Text,
        launch_date -> Timestamp,
        picture_url -> Text,
    }
}

joinable!(cinemaplaces -> cities (city_id));
joinable!(cinemarooms -> cinemaplaces (cinemaplace_id));
joinable!(cinemarooms -> cities (city_id));
joinable!(cinemasessions -> cinemaplaces (cinemaplace_id));
joinable!(cinemasessions -> cinemarooms (cinemaroom_id));
joinable!(cinemasessions -> cities (city_id));
joinable!(cinemasessions -> movies (movie_id));

allow_tables_to_appear_in_same_query!(
    cinemaplaces,
    cinemarooms,
    cinemasessions,
    cities,
    movies,
);
