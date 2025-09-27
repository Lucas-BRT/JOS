use bcrypt::{DEFAULT_COST, hash, verify};

#[tokio::test]
async fn test_bcrypt_basic_functionality() {
    let password = "SecurePass123!";

    let hashed = hash(password, DEFAULT_COST).unwrap();
    assert!(hashed.starts_with("$2b$"));

    let is_valid = verify(password, &hashed).unwrap();
    assert!(is_valid);

    let is_invalid = verify("WrongPass123", &hashed).unwrap();
    assert!(!is_invalid);

    let hashed2 = hash(password, DEFAULT_COST).unwrap();
    assert_ne!(hashed, hashed2);

    println!("✅ Bcrypt functionality test passed!");
}
