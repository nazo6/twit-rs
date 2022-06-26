# twit

Library to interact with twitter api.

## Progress
- Auth
  - [ ] Oauth1.0a
    - [ ] Auth user using web server
  - [ ] Oauth2
    - [x] App-only auth
    - [ ] Auth code flow with PKCE
  - [ ] XAuth
- APIs
  - [ ] v1
    - [ ] search/tweets
    - [ ] search/universal
    - [ ] ...
  - [ ] v2
    - [ ] search/recent
    - [ ] ...

## Usage
There are four types for usage for twitter api.
- Oauth1.0a x v1.1
- Oauth2(App-only) x v1.1
- Oauth1.0a x v2
- Oauth2 x v2

If you have Oauth1.0a token, you can use both v1.1 and v2 apis.
