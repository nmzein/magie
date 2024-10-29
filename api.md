```bash
##################################################################################################

# Create a directory via.
# POST /api/directory/:parent_id/:name

# Delete a directory via.
# DELETE /api/directory/:id?mode=["soft"|"hard"]

# Rename a directory via.
# PATCH /api/directory/:id with body { name: string }

# Move a directory via.
# PATCH /api/directory/:id with body { parent_id: number }

# Hence a rename and move can be combined into a single request with body { name: string, parent_id: number }.

##################################################################################################

# Upload an image via.
# POST /api/image/:parent_id/:name with multipart-form body.

# Delete an image via.
# DELETE /api/image/:id

# Rename an image via.
# PATCH /api/image/:id with body { name: string }

# Move an image via.
# PATCH /api/image/:id with body { parent_id: number }

# Get image properties via.
# GET /api/image/:id/properties

# Get image thumbnail via.
# GET /api/image/:id/thumbnail

# Get image annotations via.
# GET /api/image/:image_id/annotations/:annotations_id

##################################################################################################

# WebSocket /api/websocket

# Get image tiles via.
# [msg_id: u8 = 0, id: u32, level: u32, x: u32, y: u32] Total: 17 bytes

##################################################################################################
```
