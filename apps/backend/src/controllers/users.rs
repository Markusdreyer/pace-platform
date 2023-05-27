// # User Authentication
// POST /auth/login # User login, returns JWT
// POST /auth/register # User registration

// # User Management
// GET /users # List all users (requires admin role)
// POST /users # Create a new user (requires admin role)
// GET /users/{user_id} # Get details of a specific user (requires admin role)
// PUT /users/{user_id} # Update details of a specific user (requires admin role)
// DELETE /users/{user_id} # Delete a specific user (requires admin role)

// # Organizer Management
// GET /organizers # List all organizers (requires admin role)
// POST /organizers # Create a new organizer (requires admin role)
// GET /organizers/{organizer_id} # Get details of a specific organizer
// PUT /organizers/{organizer_id} # Update details of a specific organizer
// DELETE /organizers/{organizer_id} # Delete a specific organizer (requires admin role)
