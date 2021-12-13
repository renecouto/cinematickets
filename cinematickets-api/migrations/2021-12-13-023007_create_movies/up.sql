-- Your SQL goes here
create table movies (
    id bigserial primary key,
    name text not null,
    launch_date timestamp not null,
    picture_url text not null
)