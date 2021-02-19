"""password hasher algorithms
Signature:

    def enc_password(password: str, algorithm: str)  -> str
    def enc_password_with_salt(password: str, salt: str, algorithm: str)  -> str
    def chk_password(password: str, enc_password: str) -> bool

Eg.:

    from hashers import enc_password, enc_password_with_salt, chk_password
    encoded_pass_1 = enc_password("password123", "bcrypt")
    encoded_pass_2 = enc_pass_with_salt("password123", "sdpql", "bcrypt")
    chk_password(encoded_pass_1, "password123")

"""
from __future__ import absolute_import

from hashers.hashers import enc_password
from hashers.hashers import enc_password_with_salt
from hashers.hashers import chk_password

__all__ = ["enc_password", "enc_password_with_salt", "chk_password"]
