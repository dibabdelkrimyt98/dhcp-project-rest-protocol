# DHCP Manager

A comprehensive DHCP management system with a modern web interface.

## Features

- DHCP server functionality
- Device management
- User management
- Real-time statistics
- Dark/light mode
- Responsive design

## Project Structure

- `front-end/`: Frontend web application
- `src/`: Backend Rust code
  - `api/`: API endpoints for the frontend
  - `server/`: DHCP server implementation
  - `database/`: Database models and operations
  - `utils/`: Utility functions

## Requirements

- Rust (latest stable version)
- Python 3.x (for frontend development server)
- SQLite

## Installation

1. Clone the repository
2. Install dependencies:
   ```
   cargo build
   ```

## Running the Application

### Start the Backend Server

```
start_server.bat
```

This will start both the DHCP server and the API server.

### Start the Frontend Development Server

```
start_frontend.bat
```

This will start a simple HTTP server for the frontend at http://localhost:3000.

## Usage

1. Open your browser and navigate to http://localhost:3000
2. Log in with your credentials
3. Manage your DHCP network

## API Endpoints

- `GET /api/devices`: Get all devices
- `GET /api/users`: Get all users
- `GET /api/stats`: Get system statistics

## License

MIT