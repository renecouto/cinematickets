-- Your SQL goes here
create table cities (
    id bigserial primary key,
    name text not null,
    state text not null
)