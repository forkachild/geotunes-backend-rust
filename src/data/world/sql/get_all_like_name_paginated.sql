select id, name
from worlds
where name ilike %$1%
order by name
offset $2 limit $3