-- Your SQL goes here
create table cinemasessions(
    id bigserial primary key,
    cinemaplace_id bigint not null,
    city_id bigint not null,
    cinemaroom_id bigint not null,
    movie_id bigint not null,
    starts_at timestamp not null,
    available_seats integer not null,
    CONSTRAINT fk_cinemasessions_city
        FOREIGN KEY(city_id) 
        REFERENCES cities(id),
    CONSTRAINT fk_cinemasessions_cinemaplace
        FOREIGN KEY(cinemaplace_id)
        REFERENCES cinemaplaces(id),
    CONSTRAINT fk_cinemasessions_cinemaroom
        FOREIGN KEY(cinemaroom_id)
        REFERENCES cinemarooms(id),
    CONSTRAINT fk_cinemasessions_movie
            FOREIGN KEY(movie_id)
            REFERENCES movies(id)
)