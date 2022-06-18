# htracker server and website

## API Docs
```
/api/auth
```
 - Description: Validate your user's username and password.
 - Type: GET
 - Headers: `X-AuthToken: [username:password in base64]`.
 - Response: `{"error":"[ERROR]"}` or `{"valid":[bool]}`.

```
/api/register
```
 - Description: Create a new account.
 - Type: POST
 - Request Body: `{"username":"[USERNAME]","password":"[PASSWORD]","email":"[EMAIL]"}`.
 - Response: `{"error":"[ERROR]"}` or `{\"info\":\"Validation email sent to your inbox, be sure to check your spam.\"}`.

 ```
 /api/get_data
 ```
  - Description: Retrieve all user data.
  - Type: GET
  - Headers: `X-AuthToken: [username:password in base64]`.
  - Response `{"auth_token":"[AUTH_TOKEN]","todos":[{"name":"[NAME]"}...]}`.