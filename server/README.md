# htracker backend/webserver

This is a monolithic webserver/api written in Rust. It has cli args, https support, rate limiting, a custom auth flow, and a mongodb backend. Not to mention that all of this compiles down to one independent binary! No external dependencies (besides maybe glibc) required! This makes it easy to send to and execute on your server.

This server won't compile unless you've first build the frontend through the cargo-make system, so it's recommended that you build it in the way described in the main README.md

# API Docs

## Authentication
```
/api/register
```
 - Description: Create a new account.
 - Type: POST
 - Request Body: `{"username":"[USERNAME]","password":"[PASSWORD]","email":"[EMAIL]"}`.
 - Response: `{"error":"[ERROR]"}` or `{"info":"Validation email sent to your inbox, be sure to check your spam."}`.

```
/api/login
```
 - Description: Get a new session token
 - Type: POST
 - Body: `{"username":"[USERNAME]","password":"[PASSWORD IN ARGON2]"}`.
 - Response: `{"error":"[ERROR]"}` or `{"sessionToken":[SESSION TOKEN]}`.

 ```
/api/logout
```
 - Description: Discard your session token
 - Type: GET
 - Headers: `X-SessionToken: [SESSIONTOKEN]`.
 - Response: `{"error":"[ERROR]"}` or `{"sessionToken":[SESSION TOKEN]}`.

## Quotes
```
/api/quote
```
 - Description: Get a random inspirational quote.
 - Type: GET
 - Headers: `X-SessionToken: [SESSIONTOKEN]`.
 - Response: `{"quote":"[QUOTE]","author":"[AUTHOR]"}`.

### Tasks
 ```
/api/get_tasks
```
 - Description: Retrieve all user tasks.
 - Type: GET
 - Headers: `X-SessionToken: [SESSIONTOKEN]`.
 - Response: `{"error":"[ERROR]"}` or `{"auth_token":"[AUTH_TOKEN]","todos":[{"name":"[NAME]","id":"[ID]"},...]}...`.

```
/api/add_task
```
 - Description: remove a task
 - Type: POST
 - Headers: `X-SessionToken: [SESSIONTOKEN]`.
 - Request Body: `{"name":"[NAME]" (optional: "description":"[DESCRIPTION]")}`.
 - Response: `{"error":"[ERROR]"}` or `{"auth_token":"[AUTH_TOKEN]","todos":[{"name":"[NAME]","id":"[ID]"},...]}...`.

```
/api/remove_task
```
 - Description: add a task
 - Type: POST
 - Headers: `X-SessionToken: [SESSIONTOKEN]`.
 - Request Body: `{"id":"[ID]"}`.
 - Response: `{"error":"[ERROR]"}` or `{"auth_token":"[AUTH_TOKEN]","todos":[{"name":"[NAME]","id":"[ID]"},...]}...`.