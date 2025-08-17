# JOS API Documentation

## Overview

JOS (Join Our Session) is an API for managing RPG tables and sessions. This documentation describes how to use the API and access the interactive documentation.

## Documentation Access

### OpenAPI Specification
The OpenAPI specification is available at:

- **URL**: `http://localhost:3000/api-docs/openapi.json`
- **Description**: JSON specification of the API

### Using External Tools
You can use external tools to view the documentation:

1. **Swagger Editor**: Copy the JSON from `/api-docs/openapi.json` and paste it into [Swagger Editor](https://editor.swagger.io/)
2. **Postman**: Import the OpenAPI spec to generate collections
3. **Insomnia**: Import the OpenAPI spec for testing

## Main Endpoints

### Authentication

#### POST /v1/auth/signup
Create a new user account.

**Request Body:**
```json
{
  "name": "john_doe",
  "email": "john@example.com",
  "password": "password123",
  "confirm_password": "password123"
}
```

**Response:**
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "name": "john_doe",
  "email": "john@example.com"
}
```

#### POST /v1/auth/login
Authenticate user and obtain JWT token.

**Request Body:**
```json
{
  "email": "john@example.com",
  "password": "password123"
}
```

**Response:**
```json
"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

### Users

#### GET /v1/users/me
Get current user information (requires authentication).

**Headers:**
```
Authorization: Bearer <jwt_token>
```

**Response:**
```json
{
  "id": "123e4567-e89b-12d3-a456-426614174000",
  "email": "john@example.com",
  "name": "john_doe",
  "role": "user",
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": null
}
```

### RPG Tables

#### POST /v1/tables
Create a new RPG table.

**Request Body:**
```json
{
  "gm_id": "123e4567-e89b-12d3-a456-426614174000",
  "title": "Dungeons & Dragons Campaign",
  "description": "A thrilling adventure in the Forgotten Realms...",
  "game_system_id": "123e4567-e89b-12d3-a456-426614174001",
  "is_public": true,
  "max_players": 6,
  "player_slots": 6,
  "occupied_slots": 0,
  "bg_image_link": "https://example.com/bg-image.jpg"
}
```

**Response:**
```json
"123e4567-e89b-12d3-a456-426614174002"
```

#### GET /v1/tables
List all available tables.

**Response:**
```json
[
  {
    "gm_id": "123e4567-e89b-12d3-a456-426614174000",
    "title": "Dungeons & Dragons Campaign",
    "description": "A thrilling adventure in the Forgotten Realms...",
    "game_system_id": "123e4567-e89b-12d3-a456-426614174001",
    "is_public": true,
    "max_players": 6,
    "player_slots": 6,
    "occupied_slots": 0,
    "bg_image_link": "https://example.com/bg-image.jpg"
  }
]
```

### Table Requests

#### POST /v1/table-requests
Create a new request to join a table.

**Request Body:**
```json
{
  "user_id": "123e4567-e89b-12d3-a456-426614174000",
  "table_id": "123e4567-e89b-12d3-a456-426614174001",
  "message": "I would like to join this table!"
}
```

**Response:**
```json
"123e4567-e89b-12d3-a456-426614174002"
```

#### GET /v1/table-requests
List all table requests.

**Response:**
```json
[
  {
    "id": "123e4567-e89b-12d3-a456-426614174002",
    "user_id": "123e4567-e89b-12d3-a456-426614174000",
    "table_id": "123e4567-e89b-12d3-a456-426614174001",
    "message": "I would like to join this table!",
    "status": "pending",
    "created_at": "2024-01-01T00:00:00Z",
    "updated_at": null
  }
]
```

#### GET /v1/table-requests/{id}
Get a specific request by ID.

#### PUT /v1/table-requests/{id}
Update the status of a request.

**Request Body:**
```json
{
  "status": "approved"
}
```

#### DELETE /v1/table-requests/{id}
Delete a request.

## HTTP Status Codes

- `200` - Success
- `201` - Created successfully
- `400` - Validation error
- `401` - Unauthorized
- `404` - Not found
- `409` - Conflict (e.g., email already exists)
- `500` - Internal server error

## Authentication

The API uses JWT (JSON Web Tokens) for authentication. For protected endpoints, include the header:

```
Authorization: Bearer <jwt_token>
```

## Validations

### Users
- **Name**: 4-100 characters
- **Email**: Valid email format
- **Password**: 8-200 characters

### Tables
- **Title**: 8-60 characters
- **Description**: 50-1000 characters
- **Max players**: 1-20

### Table Requests
- **Message**: Maximum 500 characters
- **Status**: "pending", "approved", "rejected"

## Usage Examples

### Create account and login
```bash
# 1. Create account
curl -X POST http://localhost:3000/v1/auth/signup \
  -H "Content-Type: application/json" \
  -d '{
    "name": "john_doe",
    "email": "john@example.com",
    "password": "password123",
    "confirm_password": "password123"
  }'

# 2. Login
curl -X POST http://localhost:3000/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "john@example.com",
    "password": "password123"
  }'
```

### Create a table
```bash
curl -X POST http://localhost:3000/v1/tables \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer <jwt_token>" \
  -d '{
    "gm_id": "123e4567-e89b-12d3-a456-426614174000",
    "title": "D&D Campaign",
    "description": "A thrilling adventure in the Forgotten Realms...",
    "game_system_id": "123e4567-e89b-12d3-a456-426614174001",
    "is_public": true,
    "max_players": 6,
    "player_slots": 6,
    "occupied_slots": 0
  }'
```

## Development

### Run the server
```bash
cargo run
```

### Access the documentation
1. Start the server
2. Access `http://localhost:3000/api-docs/openapi.json` to get the OpenAPI specification
3. Use external tools like Swagger Editor, Postman, or Insomnia to view and test the API

### Documentation Structure
- **Schemas**: Data type definitions
- **Tags**: Endpoint organization by category
- **Responses**: Response examples for each endpoint
- **Parameters**: Path, query and body parameters
- **Security**: Authentication configuration

## Contributing

To add new endpoints to the documentation:

1. Add the `ToSchema` trait to DTOs
2. Use the `#[utoipa::path]` macro in handlers
3. Update schemas in `src/interfaces/http/openapi/schemas.rs`
4. Add new schemas to `ApiDoc` in `src/interfaces/http/openapi/api_doc.rs`
