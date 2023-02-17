Notes: 
- The JWK struct in Autholic was failing deserialization, by making all its field strings (vs enums as it was), it worked.