# `bevybot` Webhook Setup

`bevybot` works by setting up a repository-level webhook that listens to specific events. It listens at `/webhook` for the payload (for instance `https://example.com/webhook`), and it requires the `application/json` content type. It does not support secrets at this time. SSL verification is recommended, but dependent on how `bevybot` is deployed.

## Required events

When you create the webhook, you must enable the following events:

- [Issue comments](https://docs.github.com/en/webhooks/webhook-events-and-payloads#issue_comment)

All other events will be ignored. While it is possible to select "Send me eveything", it may slow down `bevybot` and is not recommended.
