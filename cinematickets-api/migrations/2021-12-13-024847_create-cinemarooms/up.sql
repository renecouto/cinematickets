-- Your SQL goes here
create table cinemarooms(
    id bigserial primary key,
    name text not null,
    total_seats integer not null,
    cinemaplace_id bigint not null,
    city_id bigint not null,
    CONSTRAINT fk_cinemaroom_city
        FOREIGN KEY(city_id) 
        REFERENCES cities(id),
    CONSTRAINT fk_cinemaroom_cinemaplace
        FOREIGN KEY(cinemaplace_id)
        REFERENCES cinemaplaces(id)
)