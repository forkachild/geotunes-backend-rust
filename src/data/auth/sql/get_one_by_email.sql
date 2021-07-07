select user_id
from user_details
where email = $1
limit 1