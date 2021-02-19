## Hashers

A password encoding/hasing library for Python using [Djangohashers](https://github.com/racum/rust-djangohashers) Rust crate.

## Installation

```bash
pip3 install hashers
```

### Available APIs


```bash
def enc_password(password: str, algorithm: str)  -> str
def enc_password_with_salt(password: str, salt: str, algorithm: str)  -> str
def chk_password(password: str, enc_password: str) -> bool
```

### Examples

```python
>> from hashers import enc_password, enc_password_with_salt, chk_password
>> encoded_pass_1 = enc_password("password123", "bcrypt")
>> print(encoded_pass_1)
>> 'bcrypt$$2b$12$8ThKQUSmaZmRtRUzPcUPxu1DyK.JugtxgLKHGnoI7YaLHQRONBbeK'
>> encoded_pass_2 = enc_password_with_salt("password123", "sdpql", "bcrypt")
>> chk_password("password123", encoded_pass_1)
>> True
```

### Available algorithms:

- Argon2
- BCrypt
- PBKDF2 (Default)
- PBKDF2SHA1
- BCryptSHA256
