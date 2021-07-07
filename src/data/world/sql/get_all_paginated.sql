select id, name
from worlds
order by name
offset $1 limit $2