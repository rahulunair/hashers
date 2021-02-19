## Hashers

A password encoding/hasing library for Python using Djangohashers Rust crate.

### Available APIs


```bash
def enc_password(password: str, algorithm: str)  -> str
def enc_password_with_salt(password: str, salt: str, algorithm: str)  -> str
def chk_password(password: str, enc_password: str) -> bool
```

### Examples

```
from hashers import enc_password, enc_password_with_salt, chk_password
encoded_pass_1 = enc_password("password123", "bcrypt")
encoded_pass_2 = enc_pass_with_salt("password123", "sdpql", "bcrypt")
chk_password(encoded_pass_1, "password123")
```

### Available algorithms:

- Argon2
- BCrypt
- PBKDF2 (Default)
- PBKDF2SHA1
- BCryptSHA256
