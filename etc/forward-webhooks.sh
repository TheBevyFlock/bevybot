# Sets up a temporary webhook that forwards all events to the server running locally.
#
# This is meant for development, not production. Please manually create a webhook in that case.

gh webhook forward --repo=TheBevyFlock/bevybot --url=http://localhost:8000/webhook --events=*
