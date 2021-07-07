select song_id
from song_links
where uri = $1
  and provider = $2
limit 1
