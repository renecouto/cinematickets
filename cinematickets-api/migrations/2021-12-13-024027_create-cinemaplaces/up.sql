-- Your SQL goes here
create table cinemaplaces(
    id bigserial primary key,
    name text not null,
    city_id bigint not null,
    address text not null,
    CONSTRAINT fk_cinemaplace_city
      FOREIGN KEY(city_id) 
	  REFERENCES cities(id)
)