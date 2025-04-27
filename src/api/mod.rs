use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;
use std::net::Ipv4Addr;
use rusqlite::Connection;

use crate::database::devices::{init_db, insert_device};
use dhcp_project::utils::device_parser::DeviceInfo;

// Data structures for API responses
#[derive(Serialize)]
pub struct Device {
    ip: String,
    mac: String,
    device_type: String,
    brand: String,
    connection_type: String,
    data_transferred: String,
    last_connection: String,
    status: String,
}

#[derive(Serialize)]
pub struct User {
    id: i32,
    username: String,
    role: String,
    email: String,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    success: bool,
    data: T,
    message: String,
}

// API handlers
async fn get_devices() -> impl Responder {
    let conn = match Connection::open("devices.db") {
        Ok(conn) => conn,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                data: Vec::<Device>::new(),
                message: format!("Database error: {}", e),
            });
        }
    };

    let mut stmt = match conn.prepare("SELECT * FROM devices") {
        Ok(stmt) => stmt,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                data: Vec::<Device>::new(),
                message: format!("Query error: {}", e),
            });
        }
    };

    let device_iter = match stmt.query_map([], |row| {
        Ok(Device {
            ip: row.get(2)?,
            mac: row.get(1)?,
            device_type: row.get(3)?,
            brand: row.get(4)?,
            connection_type: row.get(5)?,
            data_transferred: format!("{} MB", row.get::<_, i64>(6)? / 1024 / 1024),
            last_connection: "2024-03-20 14:30".to_string(), // Placeholder
            status: "connecté".to_string(), // Placeholder
        })
    }) {
        Ok(iter) => iter,
        Err(e) => {
            return HttpResponse::InternalServerError().json(ApiResponse {
                success: false,
                data: Vec::<Device>::new(),
                message: format!("Row mapping error: {}", e),
            });
        }
    };

    let mut devices = Vec::new();
    for device in device_iter {
        if let Ok(device) = device {
            devices.push(device);
        }
    }

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: devices,
        message: "Devices retrieved successfully".to_string(),
    })
}

async fn get_users() -> impl Responder {
    // Sample user data - replace with actual database query
    let users = vec![
        User {
            id: 1,
            username: "admin".to_string(),
            role: "admin".to_string(),
            email: "admin@example.com".to_string(),
        },
        User {
            id: 2,
            username: "client1".to_string(),
            role: "client".to_string(),
            email: "client1@example.com".to_string(),
        },
        User {
            id: 3,
            username: "client2".to_string(),
            role: "client".to_string(),
            email: "client2@example.com".to_string(),
        },
    ];

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: users,
        message: "Users retrieved successfully".to_string(),
    })
}

async fn get_stats() -> impl Responder {
    // Sample stats data - replace with actual calculations
    let stats = serde_json::json!({
        "devices": 3,
        "users": 3,
        "server_status": "Actif",
        "traffic": "4.15 GB"
    });

    HttpResponse::Ok().json(ApiResponse {
        success: true,
        data: stats,
        message: "Stats retrieved successfully".to_string(),
    })
}

// API routes configuration
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .route("/devices", web::get().to(get_devices))
            .route("/users", web::get().to(get_users))
            .route("/stats", web::get().to(get_stats))
    );
}

// Start the API server
pub async fn start_api_server() -> std::io::Result<()> {
    println!("Starting API server on http://127.0.0.1:8080");
    
    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();
            
        App::new()
            .wrap(cors)
            .configure(configure_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
} 