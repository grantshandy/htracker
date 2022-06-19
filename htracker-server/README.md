# htracker server and website

## API Docs
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

 ```
/api/get_data
```
 - Description: Retrieve all user data.
 - Type: GET
 - Headers: `X-AuthToken: [username:password in base64]`.
 - Response: `{"auth_token":"[AUTH_TOKEN]","todos":[{"name":"[NAME]"}...]}`.

```
/api/add_todo
```
 - Description: add a todo task
 - Type: POST
 - Headers: `X-AuthToken: [username:password in base64]`.
 - Request Body: `{"name":"[NAME]"}`.
 - Response: `{"error":"[ERROR]"}` or `{"todos":[{"name":"[NAME]"}...]}`.

```
/api/remove_todo
```
 - Description: add a todo task
 - Type: POST
 - Headers: `X-AuthToken: [username:password in base64]`.
 - Request Body: `{"id":"[ID]"}`.
 - Response: `{"error":"[ERROR]"}` or `{"todos":[{"name":"[NAME]"}...]}`.