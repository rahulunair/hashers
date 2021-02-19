## Hashers

A password encoding/hasing library for Python using [Djangohashers](https://github.com/racum/rust-djangohashers) Rust crate.

## Installation

```bash
pip3 install hashers
```

### Available APIs


```python
def hashpw(password: str, algorithm: str)  -> str
def hashpw_with_salt(password: str, salt: str, algorithm: str)  -> str
def chk_password(password: str, hashpw: str) -> bool
```

### Examples

```python
>> from hashers import hashpw, hashpw_with_salt, chk_password
>> encoded_pass_1 = hashpw("password123", "blake2")
>> encoded_pass_2 = hashpw_with_salt("password123", "sdpql", "blake2")
>> chk_password(encoded_pass_1, "password123")
```

### Available algorithms:

- Argon2
- BCrypt
- PBKDF2 (Default)
- PBKDF2SHA1
- BCryptSHA256
