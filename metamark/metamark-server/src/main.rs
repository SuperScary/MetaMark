use axum::{
    extract::{Path, State, WebSocketUpgrade},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, RwLock},
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::broadcast;
use tower_http::cors::CorsLayer;
use tracing::{error, info};

// Types for document collaboration
type DocumentId = String;
type UserId = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    id: UserId,
    username: String,
    password_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    sub: UserId,
    exp: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct LoginResponse {
    token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum CollaborationMessage {
    Edit {
        document_id: DocumentId,
        user_id: UserId,
        changes: Vec<Change>,
    },
    Cursor {
        document_id: DocumentId,
        user_id: UserId,
        position: usize,
    },
    Join {
        document_id: DocumentId,
        user_id: UserId,
    },
    Leave {
        document_id: DocumentId,
        user_id: UserId,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Change {
    position: usize,
    deleted: String,
    inserted: String,
}

struct AppState {
    users: RwLock<HashMap<UserId, User>>,
    documents: RwLock<HashMap<DocumentId, broadcast::Sender<CollaborationMessage>>>,
    jwt_secret: String,
}

impl AppState {
    fn new(jwt_secret: String) -> Self {
        Self {
            users: RwLock::new(HashMap::new()),
            documents: RwLock::new(HashMap::new()),
            jwt_secret,
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create application state
    let state = Arc::new(AppState::new("your-secret-key".to_string())); // In production, use a secure secret

    // Create router
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/login", post(login))
        .route("/ws/:document_id", get(websocket_handler))
        .layer(CorsLayer::permissive())
        .with_state(Arc::new(state));

    // Start server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// Health check endpoint
async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}

// Login endpoint
async fn login(
    State(state): State<Arc<AppState>>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let users = state.users.read().unwrap();
    
    // In a real application, you would:
    // 1. Verify the password hash
    // 2. Use proper error handling
    // 3. Implement proper user management
    
    let user = users
        .values()
        .find(|u| u.username == request.username)
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let claims = Claims {
        sub: user.id.clone(),
        exp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize
            + 3600, // 1 hour expiration
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse { token }))
}

// WebSocket handler
async fn websocket_handler(
    ws: WebSocketUpgrade,
    Path(document_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(|socket| handle_socket(socket, document_id, state))
}

// WebSocket connection handler
async fn handle_socket(
    socket: axum::extract::ws::WebSocket,
    document_id: DocumentId,
    state: Arc<AppState>,
) {
    let (mut sender, mut receiver) = socket.split();

    // Get or create document channel
    let tx = {
        let mut documents = state.documents.write().unwrap();
        documents
            .entry(document_id.clone())
            .or_insert_with(|| broadcast::channel(100).0)
            .clone()
    };

    // Subscribe to document changes
    let mut rx = tx.subscribe();

    // Handle incoming messages
    while let Some(Ok(msg)) = receiver.next().await {
        if let Ok(text) = msg.to_text() {
            if let Ok(collab_msg) = serde_json::from_str::<CollaborationMessage>(text) {
                if let Err(e) = tx.send(collab_msg) {
                    error!("Failed to broadcast message: {}", e);
                    break;
                }
            }
        }
    }

    // Handle disconnection
    let mut documents = state.documents.write().unwrap();
    if let Some(tx) = documents.get(&document_id) {
        if tx.receiver_count() == 0 {
            documents.remove(&document_id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    #[tokio::test]
    async fn test_health_check() {
        let state = Arc::new(AppState::new("test-secret".to_string()));
        let app = Router::new()
            .route("/health", get(health_check))
            .with_state(state);

        let response = app
            .oneshot(Request::builder().uri("/health").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }
} 