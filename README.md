# CinemaTickets

## Tables
### Cities
- used to list possible cities to look for sessions
- could be deducted from sessions
```
create table cities (
    id bigserial,
    name text not null,
    state text not null
)
```
### Movies
```
create table movies (
    id bigserial,
    name timestamp,
    launch_date timestmap,
    picture_url text
)
```
### CinemaPlaces
- Anywhere than can have cinema rooms
```
create table cinemaplaces(
    id bigserial,
    name text,
    city_id biginteger,
    address text
)
```
### CinemaRooms
```
create table cinemarooms(
    id bigserial,
    cinemaplace_id biginteger not null,
    city_id biginteger not null,
    number integer not null,
    total_seats integer not null
)
```

### Sessions
```
create table sessions(
    id bigserial,
    cinemaplace_id biginteger not null,
    city_id biginteger not null,
    cinemaroom_id biginteger not null,
    movie_id biginteger not null,
    starts_at timestamp not null,
    available_seats integer not null
)
```

## Queries
### Get available cities
```
select * from cities
```

### Get movies in schedule for city for the next days
- It might make sense to make this a materialized view, another table, and certainly cache the results
```
select m.id, m.name, m.picture_url
from sessions s
join movies m
    on m.id = s.movie_id
join cities c
    on s.city_id = c.id
where starts_at between current_timestamp and current_timestamp + interval '10' days
    and c.name = 'saopaulo'
group by m.id, m.name, m.picture_url
```

### Get sessions for movie in a specific day
```
select 
    s.id
    , s.name
    , cp.name
    , m.picture_url
    , s.starts_at
    , cp.address
    , cr.available_seats
    , cr.number
from sessions s
join movies m
    on m.id = s.movie_id
join cities c
    on s.city_id = c.id
join cinemaplaces cp
    on s.cinemaplace_id = cp.id
join cinemarooms cr
    on s.cinemaroom_id = cr.id
where starts_at between 'dayx' and 'dayx' + interval '1' days
    and c.name = 'saopaulo'
    and m.name = 'batman'
```


- Copia do ingresso.com
    - features
        - carrinho
        - filmes
        - salas por cidade
        - horarios por filme por sala
        - assentos por sala
        - autenticação
    - features+
        - reserva assento quando seleciona
        - fila de espera para salas congestionadas?
    - tecnologias
        - postgresql de banco
        - rust vs scala vs node vs go no back
        - react no front
    - por onde começar
        - desenhar fluxo de compra
    - login
        - usando keycloak fica top
        - usando ou netlify tambem funciona
    - fluxo de compra
        - escolhe cidade
            - /api/cidades -> cidades disponiveis
        - escolhe filme
            - /api/filmes?cidade=saopaulo -> entrega filmes disponiveis na cidade
        - escolhe horário dentro de sala
            - /api/sessoes?cidade=saopaulo?filme=batman -> entrega horarios de salas para filme na cidade nos proximos dias
        - escolhe assento
            - /api/sessoes/sessao_x/assentos?
        - reserva assento
        - espera processamento do pagamento
        - confirma reserva de assento