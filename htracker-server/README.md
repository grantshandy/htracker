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
/api/auth
```
 - Description: Validate your user's username and password.
 - Type: GET
 - Headers: `X-AuthToken: [username:password in base64]`.
 - Response: `{"error":"[ERROR]"}` or `{"valid":[bool]}`.

## Quotes
```
/api/quote
```
 - Description: Get a random inspirational quote.
 - Type: GET
 - Response: `{"quote":"[QUOTE]","author":"[AUTHOR]"}`.

### Tasks
 ```
/api/get_tasks
```
 - Description: Retrieve all user tasks.
 - Type: GET
 - Headers: `X-AuthToken: [username:password in base64]`.
 - Response: `{"auth_token":"[AUTH_TOKEN]","todos":[{"name":"[NAME]","id":"[ID]"}...]}`.

```
/api/add_task
```
 - Description: remove a task
 - Type: POST
 - Headers: `X-AuthToken: [username:password in base64]`.
 - Request Body: `{"name":"[NAME]"}`.
 - Response: `{"error":"[ERROR]"}` or `{"auth_token":"[AUTH_TOKEN]","todos":[{"name":"[NAME]","id":"[ID]"}...]}`.

```
/api/remove_task
```
 - Description: add a task
 - Type: POST
 - Headers: `X-AuthToken: [username:password in base64]`.
 - Request Body: `{"id":"[ID]"}`.
 - Response: `{"error":"[ERROR]"}` or `{"auth_token":"[AUTH_TOKEN]","todos":[{"name":"[NAME]","id":"[ID]"}...]}`.