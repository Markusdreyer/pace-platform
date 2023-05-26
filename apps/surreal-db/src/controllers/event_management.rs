// # REST API Endpoints

// # Event Management
// GET /organizers/{organizer_id}/events # List all events organized by a specific organizer
// POST /organizers/{organizer_id}/events # Create a new event by a specific organizer
// GET /organizers/{organizer_id}/events/{event_id} # Get details of a specific event
// PUT /organizers/{organizer_id}/events/{event_id} # Update details of a specific event
// DELETE /organizers/{organizer_id}/events/{event_id} # Delete a specific event

// # Results
// GET /events/{event_id}/results # Get results of a specific event
// POST /events/{event_id}/results # Update results of a specific event

// # Public API
// GET /events/{event_id}/public_data # Get public data of a specific event

// # WebSocket Endpoints
// WS /events/{event_id}/public_data # WebSocket endpoint for real-time public data updates
