# API Docs

## Auth
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
 - Description: Validate your user's username and password.
 - Type: GET
 - Headers: `X-AuthToken: [username:password in base64]`.
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
 - Response: `{"error":"[ERROR]"}` or `{"auth_token":"[AUTH_TOKEN]","todos":[{"name":"[NAME]","id":"[ID]"},...]}`.

```
/api/add_task
```
 - Description: remove a task
 - Type: POST
 - Headers: `X-SessionToken: [SESSIONTOKEN]`.
 - Request Body: `{"name":"[NAME]" (optional: "description":"[DESCRIPTION]")}`.
 - Response: `{"error":"[ERROR]"}` or `{"auth_token":"[AUTH_TOKEN]","todos":[{"name":"[NAME]","id":"[ID]"},...]}`.

```
/api/remove_task
```
 - Description: add a task
 - Type: POST
 - Headers: `X-SessionToken: [SESSIONTOKEN]`.
 - Request Body: `{"id":"[ID]"}`.
 - Response: `{"error":"[ERROR]"}` or `{"auth_token":"[AUTH_TOKEN]","todos":[{"name":"[NAME]","id":"[ID]"},...]}`.