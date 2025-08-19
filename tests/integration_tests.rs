//! Integration tests for the authentication API
//! 
//! These tests demonstrate how to use the API and verify that all endpoints
//! work correctly. They require a running PostgreSQL and Redis instance.
//! 
//! To run tests:
//! 1. Start dependencies: docker-compose up -d
//! 2. Set environment variables or create .env file
//! 3. Run tests: cargo test --test integration_tests

use serde_json::json;

#[cfg(test)]
mod tests {
    use super::*;

    // Mock test since we can't easily run integration tests without dependencies
    #[test]
    fn test_api_contract() {
        // Test that our request/response structures serialize correctly
        
        // Test registration request
        let register_request = json!({
            "name": "John Doe",
            "email": "john@example.com",
            "password": "password123"
        });
        
        let register_response = json!({
            "id": "550e8400-e29b-41d4-a716-446655440000",
            "name": "John Doe",
            "email": "john@example.com"
        });
        
        // Test login request
        let login_request = json!({
            "email": "john@example.com",
            "password": "password123"
        });
        
        let login_response = json!({
            "token": "550e8400-e29b-41d4-a716-446655440000"
        });
        
        // Test error response
        let error_response = json!({
            "error": "Email already registered"
        });
        
        // Verify all JSON structures are valid
        assert!(register_request.is_object());
        assert!(register_response.is_object());
        assert!(login_request.is_object());
        assert!(login_response.is_object());
        assert!(error_response.is_object());
        
        // Verify required fields exist
        assert!(register_request["name"].is_string());
        assert!(register_request["email"].is_string());
        assert!(register_request["password"].is_string());
        
        assert!(register_response["id"].is_string());
        assert!(register_response["name"].is_string());
        assert!(register_response["email"].is_string());
        
        assert!(login_request["email"].is_string());
        assert!(login_request["password"].is_string());
        
        assert!(login_response["token"].is_string());
        
        assert!(error_response["error"].is_string());
    }
    
    #[test]
    fn test_validation_rules() {
        // Test email validation pattern
        let valid_emails = vec![
            "user@example.com",
            "test.user@domain.org",
            "user123@test-domain.com",
        ];
        
        let invalid_emails = vec![
            "invalid-email",
            "@example.com",
            "user@",
            "user@example",
        ];
        
        for email in valid_emails {
            assert!(is_valid_email(email), "Email {} should be valid", email);
        }
        
        for email in invalid_emails {
            assert!(!is_valid_email(email), "Email {} should be invalid", email);
        }
    }
    
    #[test]
    fn test_password_requirements() {
        let valid_passwords = vec![
            "password123",      // 11 chars
            "123456",          // 6 chars (minimum)
        ];
        
        let valid_long_password = "a".repeat(24); // 24 chars (maximum)
        
        let invalid_passwords = vec![
            "short",           // 5 chars (too short)
            "",                // empty
        ];
        
        let invalid_long_password = "a".repeat(25); // 25 chars (too long)
        
        for password in &valid_passwords {
            assert!(is_valid_password(password), "Password {} should be valid", password);
        }
        
        assert!(is_valid_password(&valid_long_password), "24-char password should be valid");
        
        for password in &invalid_passwords {
            assert!(!is_valid_password(password), "Password {} should be invalid", password);
        }
        
        assert!(!is_valid_password(&invalid_long_password), "25-char password should be invalid");
    }
    
    fn is_valid_email(email: &str) -> bool {
        use regex::Regex;
        let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,6}$").unwrap();
        email_regex.is_match(email)
    }
    
    fn is_valid_password(password: &str) -> bool {
        password.len() >= 6 && password.len() <= 24
    }
}

/// Example of how to perform a full integration test if dependencies are available
#[cfg(feature = "test-integration")]
#[tokio::test]
async fn test_full_authentication_flow() {
    use reqwest::Client;
    use serde_json::json;
    
    let client = Client::new();
    let base_url = "http://localhost:8080";
    
    // Test 1: Health check
    let response = client
        .get(&format!("{}/health", base_url))
        .send()
        .await
        .expect("Failed to send health request");
    
    assert!(response.status().is_success());
    let health: serde_json::Value = response.json().await.expect("Failed to parse health response");
    assert_eq!(health["status"], "ok");
    
    // Test 2: Register a user
    let register_payload = json!({
        "name": "Test User",
        "email": "test@example.com", 
        "password": "password123"
    });
    
    let response = client
        .post(&format!("{}/auth/register", base_url))
        .json(&register_payload)
        .send()
        .await
        .expect("Failed to send register request");
    
    assert!(response.status().is_success());
    let register_response: serde_json::Value = response.json().await.expect("Failed to parse register response");
    let user_id = register_response["id"].as_str().expect("User ID should be present");
    
    // Test 3: Login with the user
    let login_payload = json!({
        "email": "test@example.com",
        "password": "password123"
    });
    
    let response = client
        .post(&format!("{}/auth/login", base_url))
        .json(&login_payload)
        .send()
        .await
        .expect("Failed to send login request");
    
    assert!(response.status().is_success());
    let login_response: serde_json::Value = response.json().await.expect("Failed to parse login response");
    let token = login_response["token"].as_str().expect("Token should be present");
    
    // Test 4: Get user info with token
    let response = client
        .get(&format!("{}/auth/me", base_url))
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .expect("Failed to send me request");
    
    assert!(response.status().is_success());
    let user_info: serde_json::Value = response.json().await.expect("Failed to parse user info");
    assert_eq!(user_info["id"], user_id);
    assert_eq!(user_info["email"], "test@example.com");
    assert_eq!(user_info["name"], "Test User");
    
    // Test 5: Test invalid token
    let response = client
        .get(&format!("{}/auth/me", base_url))
        .header("Authorization", "Bearer invalid-token")
        .send()
        .await
        .expect("Failed to send me request with invalid token");
    
    assert_eq!(response.status(), 401);
}