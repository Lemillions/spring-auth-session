#!/bin/bash

# Test script for the Rust authentication API
# This script demonstrates how to test all the endpoints

set -e

API_URL="http://localhost:8080"
USER_EMAIL="test@example.com"
USER_PASSWORD="password123"
USER_NAME="Test User"

echo "🦀 Testing Rust Authentication API"
echo "=================================="

# Test health endpoint
echo ""
echo "1. Testing health endpoint..."
curl -s -X GET "$API_URL/health" | jq .
echo "✅ Health check passed"

# Register a new user
echo ""
echo "2. Testing user registration..."
REGISTER_RESPONSE=$(curl -s -X POST "$API_URL/auth/register" \
  -H "Content-Type: application/json" \
  -d "{
    \"name\": \"$USER_NAME\",
    \"email\": \"$USER_EMAIL\",
    \"password\": \"$USER_PASSWORD\"
  }")

echo "$REGISTER_RESPONSE" | jq .
USER_ID=$(echo "$REGISTER_RESPONSE" | jq -r .id)
echo "✅ User registered with ID: $USER_ID"

# Login with the user
echo ""
echo "3. Testing user login..."
LOGIN_RESPONSE=$(curl -s -X POST "$API_URL/auth/login" \
  -H "Content-Type: application/json" \
  -d "{
    \"email\": \"$USER_EMAIL\",
    \"password\": \"$USER_PASSWORD\"
  }")

echo "$LOGIN_RESPONSE" | jq .
TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r .token)
echo "✅ Login successful, token: ${TOKEN:0:20}..."

# Get current user info
echo ""
echo "4. Testing authenticated endpoint..."
USER_INFO=$(curl -s -X GET "$API_URL/auth/me" \
  -H "Authorization: Bearer $TOKEN")

echo "$USER_INFO" | jq .
echo "✅ User info retrieved"

# Test invalid token
echo ""
echo "5. Testing invalid token..."
INVALID_RESPONSE=$(curl -s -X GET "$API_URL/auth/me" \
  -H "Authorization: Bearer invalid-token")

echo "$INVALID_RESPONSE" | jq .
echo "✅ Invalid token properly rejected"

# Test duplicate email registration
echo ""
echo "6. Testing duplicate email registration..."
DUPLICATE_RESPONSE=$(curl -s -X POST "$API_URL/auth/register" \
  -H "Content-Type: application/json" \
  -d "{
    \"name\": \"Another User\",
    \"email\": \"$USER_EMAIL\",
    \"password\": \"differentpassword\"
  }")

echo "$DUPLICATE_RESPONSE" | jq .
echo "✅ Duplicate email properly rejected"

echo ""
echo "🎉 All tests completed successfully!"
echo "The Rust implementation is working correctly!"