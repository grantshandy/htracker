# Htracker server and website

## API Docs

```
/api/auth
```
 - Type: GET
 - Headers: `X-AuthToken: (username:password in base64)`
 - Returns: `{"error":"[ERROR]"}` or `{"valid":bool}`