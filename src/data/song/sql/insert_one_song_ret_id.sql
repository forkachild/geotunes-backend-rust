insert into songs (title, artist)
values ($1, $2)
returning id