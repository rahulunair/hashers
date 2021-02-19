"""password hasher algorithms
Signature:

    def hashpw(password: str, algorithm: str)  -> str
    def hashpw_with_salt(password: str, salt: str, algorithm: str)  -> str
    def chk_password(password: str, hashpw: str) -> bool

Eg.:

    from hashers import hashpw, hashpw_with_salt, chk_password
    encoded_pass_1 = hashpw("password123", "bcrypt")
    encoded_pass_2 = enc_pass_with_salt("password123", "sdpql", "bcrypt")
    chk_password(encoded_pass_1, "password123")

"""
from __future__ import absolute_import

from hashers.hashers import hashpw
from hashers.hashers import hashpw_with_salt
from hashers.hashers import chk_password

__all__ = ["hashpw", "hashpw_with_salt", "chk_password"]
