create extension if not exists "cube";
create extension if not exists "earthdistance";
create extension if not exists "uuid-ossp";

create table if not exists users
(
    id uuid primary key default uuid_generate_v4()
);

create table if not exists user_details
(
    id      uuid primary key default uuid_generate_v4(),
    user_id uuid         not null references users (id),
    email   varchar(254) not null
);

create table if not exists user_auth_providers
(
    id       uuid primary key default uuid_generate_v4(),
    user_id  uuid        not null references users (id),
    provider varchar(24) not null,
    token    text        not null
);

create table if not exists songs
(
    id     uuid primary key default uuid_generate_v4(),
    title  text not null,
    artist text not null
);

create table if not exists song_links
(
    id       uuid primary key default uuid_generate_v4(),
    song_id  uuid        not null references songs (id),
    provider varchar(24) not null,
    uri      text        not null
);

create table if not exists worlds
(
    id   uuid primary key default uuid_generate_v4(),
    name varchar(128) not null
);

create table if not exists stations
(
    id  uuid primary key default uuid_generate_v4(),
    lat double precision not null,
    lon double precision not null
);

create table if not exists song_history
(
    id         uuid primary key default uuid_generate_v4(),
    user_id    uuid             not null references users (id),
    station_id uuid             not null references stations (id),
    song_id    uuid             not null references songs (id),
    world_id   uuid references worlds (id),
    lat        double precision not null,
    lon        double precision not null,
    created_at timestamptz      default now(),
    updated_at timestamptz      default null
);

create index if not exists station_location_index
    on stations using gist (ll_to_earth(lat, lon));
