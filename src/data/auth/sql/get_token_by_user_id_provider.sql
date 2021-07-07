select token
from user_auth_providers
where user_id = $1
  and provider = $2