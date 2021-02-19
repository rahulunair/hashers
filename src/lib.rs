use djangohashers::*;
use pyo3::prelude::*;

//password primitives from Django hashers in Python
#[pymodule(hashers)]
fn ruuid(_py: Python, m: &PyModule) -> PyResult<()> {
    #[pyfn(m, "chk_password")]
    // check a password
    fn chk_password(_py: Python, password: &str, encoded: &str) -> PyResult<bool> {
        Ok(check_password(password, encoded).unwrap())
    }
    #[pyfn(m, "hashpw")]
    // encode a password with algorithm
    // // available algorithms = argon2, bcrypt, bcrypt_sha256,
    // // pbkdf2_sha1, pbkdf2
    fn hashpw(_py: Python, password: &str, algorithm: &str) -> PyResult<String> {
        let encoded = match algorithm {
            "argon2" => make_password_with_algorithm(password, Algorithm::Argon2),
            "bcrypt" => make_password_with_algorithm(password, Algorithm::BCrypt),
            "bcrypt_sha256" => make_password_with_algorithm(password, Algorithm::BCryptSHA256),
            "pbkdf2_sha1" => make_password_with_algorithm(password, Algorithm::PBKDF2SHA1),
            "pbkdf2" => make_password_with_algorithm(password, Algorithm::PBKDF2),
            _ => make_password(password),
        };
        Ok(encoded)
    }
    #[pyfn(m, "hashpw_with_salt")]
    // encode a password with a salt and algorithm
    // // available algorithms = argon2, bcrypt, bcrypt_sha256,
    // // pbkdf2_sha1, pbkdf2
    fn hashpw_with_salt(
        _py: Python,
        password: &str,
        salt: &str,
        algorithm: &str,
    ) -> PyResult<String> {
        let encoded = match algorithm {
            "argon2" => make_password_with_settings(password, salt, Algorithm::Argon2),
            "bcrypt" => make_password_with_settings(password, salt, Algorithm::BCrypt),
            "bcrypt_sha256" => make_password_with_settings(password, salt, Algorithm::BCryptSHA256),
            "pbkdf2" => make_password_with_settings(password, salt, Algorithm::PBKDF2),
            "pbkdf2_sha1" => make_password_with_settings(password, salt, Algorithm::PBKDF2SHA1),
            _ => make_password_with_settings(password, salt, Algorithm::PBKDF2),
        };
        Ok(encoded)
    }
    Ok(())
}
