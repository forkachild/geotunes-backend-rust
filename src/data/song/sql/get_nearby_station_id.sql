select id
from stations
where earth_distance(ll_to_earth(lat, lon), ll_to_earth($1, $2)) < $3
limit 1