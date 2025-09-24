use bcrypt::{hash, verify, DEFAULT_COST};

#[tokio::test]
async fn test_bcrypt_basic_functionality() {
    let password = "SecurePass123!";
    
    // Test hashing
    let hashed = hash(password, DEFAULT_COST).unwrap();
    assert!(hashed.starts_with("$2b$"));
    
    // Test verification
    let is_valid = verify(password, &hashed).unwrap();
    assert!(is_valid);
    
    // Test with wrong password
    let is_invalid = verify("WrongPass123", &hashed).unwrap();
    assert!(!is_invalid);
    
    // Test that same password produces different hashes (due to salt)
    let hashed2 = hash(password, DEFAULT_COST).unwrap();
    assert_ne!(hashed, hashed2);
    
    println!("✅ Bcrypt functionality test passed!");
}

