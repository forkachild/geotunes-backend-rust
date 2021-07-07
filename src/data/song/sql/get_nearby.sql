select s.id         as id,
       s.title      as title,
       s.artist     as artist,
       sl.provider  as provider,
       sl.uri       as uri,
       count(sh.id) as play_count
from songs s
         join song_links sl on s.id = sl.song_id
         join song_history sh on s.id = sh.song_id
         join stations st on sh.station_id = st.id
where earth_distance(ll_to_earth(st.lat, st.lon), ll_to_earth($1, $2)) < $3
  and sl.provider = $4
group by s.id, sl.id