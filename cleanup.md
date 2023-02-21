Notes: 
# High Priority
- [done] The JWK struct in Autholic was failing deserialization, by making all its field strings (vs enums as it was), it worked.
- [done] Me: Gotta delete/remove the auth0-angular/spa-js referenes in the repo.
- [done][using the github dist version without it] Beause I used the getAccessTokenSilently function in the auth0 client, we shouldnt need any of the role mapping changes Phoenix made (just all the the URL, paths, etc fixes he made).


# Medium 
- Tim: I had to disable the TLS validation on the Rust ReqWest call to the jwks of keycloak. This stumped me for a bit, but I finally remembered we had to do this for the gms_api as well. There's nothing we can really do about this in develop/localhost with self signed certs.

# Low/subjective-need
- The openfmb-hmi audience wasnt configured in keycloak. I'm assuming this is because we've only setup GMS so far. We an just use that, but 
  we'll want to enable that as the audience then in the rust validation.

# Last Steps
- Get docker images built with KeyCloak changes:
- - GMS JS Appliations
- - OpenFMB HMI
- Get the MVP/develop-keycloak branch updated
- - Update GMS-JS-Applications/Openfmb HMI tags in there
- - 
- Test the MVP/develop-keycloak branch
- - verify all the UI apps work
- - Make sure HMI is still working
- Make Merge request for MVP; MVP/develop-keycloak into MVP/main