insert into stations (lat, lon)
values ($1, $2)
returning id;