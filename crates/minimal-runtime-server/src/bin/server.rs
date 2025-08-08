//! # 🌐 SOLFUNMEME MetaMeme HTTP Server
//! 
//! A minimal, high-performance HTTP server for the SOLFUNMEME MetaMeme engine.
//! Provides RESTful APIs for all MetaMeme operations without requiring Solana.

use std::sync::{Arc, Mutex};
use std::net::SocketAddr;

use axum::{
    extract::{Path, Query, State},
    http::{StatusCode, HeaderMap},
    response::{Html, Json},
    routing::{get, post},
    Router,
};
use tower::ServiceBuilder;
use tower_http::cors::{CorsLayer, Any};
use tower_http::services::ServeDir;
use serde::{Deserialize, Serialize};
use serde_json::json;
use log::{info, error, warn};

use minimal_runtime_server::{
    MetaMemeRuntime, PoemRequest, QuineRequest, AnalysisRequest, NFTRequest,
    GeneratedPoem, QuineResult, AnalysisResult, RuntimeStats,
};

/// 🌟 Application state
type AppState = Arc<Mutex<MetaMemeRuntime>>;

/// 📝 Query parameters for various endpoints
#[derive(Debug, Deserialize)]
struct PaginationQuery {
    page: Option<usize>,
    limit: Option<usize>,
}

#[derive(Debug, Deserialize)]
struct SessionQuery {
    session_id: Option<String>,
}

/// 🎯 API Response wrapper
#[derive(Debug, Serialize)]
struct ApiResponse<T> {
    success: bool,
    data: Option<T>,
    error: Option<String>,
    timestamp: u64,
}

impl<T> ApiResponse<T> {
    fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
    
    fn error(message: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(message),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    info!("🚀 Starting SOLFUNMEME MetaMeme Server...");
    
    // Create the runtime
    let runtime = Arc::new(Mutex::new(MetaMemeRuntime::new()));
    
    // Build the router
    let app = Router::new()
        // 🏠 Home and documentation
        .route("/", get(home_handler))
        .route("/health", get(health_handler))
        .route("/stats", get(stats_handler))
        
        // 🎭 Core MetaMeme operations
        .route("/api/v1/poem", post(generate_poem_handler))
        .route("/api/v1/quine", post(create_quine_handler))
        .route("/api/v1/analyze", post(analyze_emoji_handler))
        .route("/api/v1/nft", post(generate_nft_handler))
        
        // 👤 Session management
        .route("/api/v1/session", post(create_session_handler))
        .route("/api/v1/session/:session_id", get(get_session_handler))
        
        // 📊 Data retrieval
        .route("/api/v1/poems", get(list_poems_handler))
        .route("/api/v1/poems/:poem_id", get(get_poem_handler))
        .route("/api/v1/nfts", get(list_nfts_handler))
        .route("/api/v1/nfts/:token_id", get(get_nft_handler))
        
        // 🌐 Interactive endpoints
        .route("/api/v1/repl", post(repl_handler))
        .route("/api/v1/batch", post(batch_handler))
        
        // 🧹 Maintenance
        .route("/api/v1/cleanup", post(cleanup_handler))
        
        // Static files (for web interface)
        .nest_service("/static", ServeDir::new("static"))
        
        // Add state
        .with_state(runtime)
        
        // Add middleware
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::new()
                    .allow_origin(Any)
                    .allow_methods(Any)
                    .allow_headers(Any))
        );
    
    // Start the server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("🌐 Server listening on http://{}", addr);
    info!("📚 API documentation available at http://{}/", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

/// 🏠 Home page with API documentation
async fn home_handler() -> Html<&'static str> {
    Html(r#"
<!DOCTYPE html>
<html>
<head>
    <title>🌀 SOLFUNMEME MetaMeme Server</title>
    <style>
        body { font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif; margin: 40px; background: #0a0a0a; color: #ffffff; }
        .header { text-align: center; margin-bottom: 40px; }
        .emoji { font-size: 2em; }
        .endpoint { background: #1a1a1a; padding: 15px; margin: 10px 0; border-radius: 8px; border-left: 4px solid #00ff88; }
        .method { color: #00ff88; font-weight: bold; }
        .path { color: #88aaff; font-family: monospace; }
        .description { color: #cccccc; margin-top: 5px; }
        .example { background: #2a2a2a; padding: 10px; margin: 10px 0; border-radius: 4px; font-family: monospace; font-size: 0.9em; }
        .section { margin: 30px 0; }
        h1, h2 { color: #00ff88; }
        a { color: #88aaff; text-decoration: none; }
        a:hover { text-decoration: underline; }
    </style>
</head>
<body>
    <div class="header">
        <h1><span class="emoji">🌀</span> SOLFUNMEME MetaMeme Server <span class="emoji">🎭</span></h1>
        <p>Blockchain-free Lambda Calculus Poetry Engine</p>
        <p><strong>Status:</strong> <span style="color: #00ff88;">🟢 ONLINE</span></p>
    </div>

    <div class="section">
        <h2>🎭 Core Operations</h2>
        
        <div class="endpoint">
            <div><span class="method">POST</span> <span class="path">/api/v1/poem</span></div>
            <div class="description">Generate poetry from emoji sequences</div>
            <div class="example">{"emoji_sequence": "🌀🎭🧬", "session_id": "optional"}</div>
        </div>
        
        <div class="endpoint">
            <div><span class="method">POST</span> <span class="path">/api/v1/quine</span></div>
            <div class="description">Create self-replicating expressions</div>
            <div class="example">{"seed": "🌀", "max_reduction_steps": 100}</div>
        </div>
        
        <div class="endpoint">
            <div><span class="method">POST</span> <span class="path">/api/v1/analyze</span></div>
            <div class="description">Analyze emoji sequences and lambda expressions</div>
            <div class="example">{"emoji_sequence": "🌀🎭", "include_trace": true}</div>
        </div>
        
        <div class="endpoint">
            <div><span class="method">POST</span> <span class="path">/api/v1/nft</span></div>
            <div class="description">Generate NFT metadata from emoji poetry</div>
            <div class="example">{"emoji_sequence": "🌀🎭🧬🌌", "custom_attributes": {"theme": "cosmic"}}</div>
        </div>
    </div>

    <div class="section">
        <h2>📊 Information</h2>
        
        <div class="endpoint">
            <div><span class="method">GET</span> <span class="path">/health</span></div>
            <div class="description">Server health check</div>
        </div>
        
        <div class="endpoint">
            <div><span class="method">GET</span> <span class="path">/stats</span></div>
            <div class="description">Runtime statistics and metrics</div>
        </div>
        
        <div class="endpoint">
            <div><span class="method">GET</span> <span class="path">/api/v1/poems</span></div>
            <div class="description">List generated poems (paginated)</div>
        </div>
        
        <div class="endpoint">
            <div><span class="method">GET</span> <span class="path">/api/v1/nfts</span></div>
            <div class="description">List generated NFTs (paginated)</div>
        </div>
    </div>

    <div class="section">
        <h2>🌟 Quick Examples</h2>
        
        <h3>Generate a Poem:</h3>
        <div class="example">
curl -X POST http://localhost:3000/api/v1/poem \
  -H "Content-Type: application/json" \
  -d '{"emoji_sequence": "🌀🎭🧬"}'
        </div>
        
        <h3>Create a Quine:</h3>
        <div class="example">
curl -X POST http://localhost:3000/api/v1/quine \
  -H "Content-Type: application/json" \
  -d '{"seed": "🌀"}'
        </div>
        
        <h3>Analyze Emojis:</h3>
        <div class="example">
curl -X POST http://localhost:3000/api/v1/analyze \
  -H "Content-Type: application/json" \
  -d '{"emoji_sequence": "🌀🎭", "include_trace": true}'
        </div>
    </div>

    <div class="section">
        <h2>🧬 About SOLFUNMEME</h2>
        <p>SOLFUNMEME is a revolutionary system that combines:</p>
        <ul>
            <li><strong>🧬 Lambda Calculus:</strong> Mathematical foundation for computation</li>
            <li><strong>🎭 Emoji Semantics:</strong> Visual language for expressing complex ideas</li>
            <li><strong>🌀 Self-Replication:</strong> Quines that achieve digital consciousness</li>
            <li><strong>🎨 NFT Generation:</strong> Unique digital art from mathematical beauty</li>
        </ul>
        <p>This server provides a blockchain-free environment for exploring computational creativity!</p>
    </div>

    <footer style="text-align: center; margin-top: 50px; color: #666;">
        <p>🚀 SOLFUNMEME MetaMeme Server - Where Code Becomes Poetry</p>
    </footer>
</body>
</html>
    "#)
}

/// 💚 Health check endpoint
async fn health_handler() -> Json<serde_json::Value> {
    Json(json!({
        "status": "healthy",
        "service": "solfunmeme-metameme-server",
        "version": "0.1.0",
        "timestamp": std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }))
}

/// 📊 Statistics endpoint
async fn stats_handler(State(state): State<AppState>) -> Result<Json<ApiResponse<RuntimeStats>>, StatusCode> {
    let runtime = state.lock().unwrap();
    match runtime.get_stats() {
        Ok(stats) => Ok(Json(ApiResponse::success(stats))),
        Err(e) => {
            error!("Failed to get stats: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

/// 🎭 Generate poem endpoint
async fn generate_poem_handler(
    State(state): State<AppState>,
    Json(request): Json<PoemRequest>,
) -> Result<Json<ApiResponse<GeneratedPoem>>, StatusCode> {
    let mut runtime = state.lock().unwrap();
    match runtime.generate_poem(request) {
        Ok(poem) => Ok(Json(ApiResponse::success(poem))),
        Err(e) => {
            error!("Failed to generate poem: {}", e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

/// 🌀 Create quine endpoint
async fn create_quine_handler(
    State(state): State<AppState>,
    Json(request): Json<QuineRequest>,
) -> Result<Json<ApiResponse<QuineResult>>, StatusCode> {
    let mut runtime = state.lock().unwrap();
    match runtime.create_quine(request) {
        Ok(quine) => Ok(Json(ApiResponse::success(quine))),
        Err(e) => {
            error!("Failed to create quine: {}", e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

/// 🔍 Analyze emoji endpoint
async fn analyze_emoji_handler(
    State(state): State<AppState>,
    Json(request): Json<AnalysisRequest>,
) -> Result<Json<ApiResponse<AnalysisResult>>, StatusCode> {
    let mut runtime = state.lock().unwrap();
    match runtime.analyze_emoji(request) {
        Ok(analysis) => Ok(Json(ApiResponse::success(analysis))),
        Err(e) => {
            error!("Failed to analyze emoji: {}", e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

/// 🎨 Generate NFT endpoint
async fn generate_nft_handler(
    State(state): State<AppState>,
    Json(request): Json<NFTRequest>,
) -> Result<Json<ApiResponse<emoji_semantics::NFTMetadata>>, StatusCode> {
    let mut runtime = state.lock().unwrap();
    match runtime.generate_nft(request) {
        Ok(nft) => Ok(Json(ApiResponse::success(nft))),
        Err(e) => {
            error!("Failed to generate NFT: {}", e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

/// 👤 Create session endpoint
async fn create_session_handler(
    State(state): State<AppState>,
) -> Result<Json<ApiResponse<minimal_runtime_server::Session>>, StatusCode> {
    let runtime = state.lock().unwrap();
    match runtime.create_session() {
        Ok(session) => Ok(Json(ApiResponse::success(session))),
        Err(e) => {
            error!("Failed to create session: {}", e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}

/// 👤 Get session endpoint
async fn get_session_handler(
    State(state): State<AppState>,
    Path(session_id): Path<String>,
) -> Result<Json<ApiResponse<minimal_runtime_server::Session>>, StatusCode> {
    let runtime = state.lock().unwrap();
    let sessions = runtime.sessions.read().unwrap();
    
    match sessions.get(&session_id) {
        Some(session) => Ok(Json(ApiResponse::success(session.clone()))),
        None => Ok(Json(ApiResponse::error("Session not found".to_string()))),
    }
}

/// 📝 List poems endpoint
async fn list_poems_handler(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<Vec<GeneratedPoem>>>, StatusCode> {
    let runtime = state.lock().unwrap();
    let poems = runtime.poems_cache.read().unwrap();
    
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10).min(100); // Max 100 per page
    let offset = (page - 1) * limit;
    
    let poems_vec: Vec<GeneratedPoem> = poems.values()
        .skip(offset)
        .take(limit)
        .cloned()
        .collect();
    
    Ok(Json(ApiResponse::success(poems_vec)))
}

/// 📝 Get specific poem endpoint
async fn get_poem_handler(
    State(state): State<AppState>,
    Path(poem_id): Path<String>,
) -> Result<Json<ApiResponse<GeneratedPoem>>, StatusCode> {
    let runtime = state.lock().unwrap();
    let poems = runtime.poems_cache.read().unwrap();
    
    match poems.get(&poem_id) {
        Some(poem) => Ok(Json(ApiResponse::success(poem.clone()))),
        None => Ok(Json(ApiResponse::error("Poem not found".to_string()))),
    }
}

/// 🎨 List NFTs endpoint
async fn list_nfts_handler(
    State(state): State<AppState>,
    Query(pagination): Query<PaginationQuery>,
) -> Result<Json<ApiResponse<Vec<emoji_semantics::NFTMetadata>>>, StatusCode> {
    let runtime = state.lock().unwrap();
    let nfts = runtime.nft_cache.read().unwrap();
    
    let page = pagination.page.unwrap_or(1);
    let limit = pagination.limit.unwrap_or(10).min(100);
    let offset = (page - 1) * limit;
    
    let nfts_vec: Vec<emoji_semantics::NFTMetadata> = nfts.values()
        .skip(offset)
        .take(limit)
        .cloned()
        .collect();
    
    Ok(Json(ApiResponse::success(nfts_vec)))
}

/// 🎨 Get specific NFT endpoint
async fn get_nft_handler(
    State(state): State<AppState>,
    Path(token_id): Path<u32>,
) -> Result<Json<ApiResponse<emoji_semantics::NFTMetadata>>, StatusCode> {
    let runtime = state.lock().unwrap();
    let nfts = runtime.nft_cache.read().unwrap();
    
    match nfts.get(&token_id) {
        Some(nft) => Ok(Json(ApiResponse::success(nft.clone()))),
        None => Ok(Json(ApiResponse::error("NFT not found".to_string()))),
    }
}

/// 🔄 REPL-style interaction endpoint
#[derive(Debug, Deserialize)]
struct ReplRequest {
    command: String,
    session_id: Option<String>,
}

async fn repl_handler(
    State(state): State<AppState>,
    Json(request): Json<ReplRequest>,
) -> Result<Json<ApiResponse<serde_json::Value>>, StatusCode> {
    let mut runtime = state.lock().unwrap();
    
    let response = match request.command.as_str() {
        cmd if cmd.starts_with(":help") => {
            json!({
                "type": "help",
                "message": "Available commands: :help, :stats, :quit, or enter emoji sequences"
            })
        }
        cmd if cmd.starts_with(":stats") => {
            match runtime.get_stats() {
                Ok(stats) => json!({"type": "stats", "data": stats}),
                Err(e) => json!({"type": "error", "message": e.to_string()}),
            }
        }
        cmd if cmd.starts_with(":quit") => {
            json!({"type": "quit", "message": "Goodbye from the MetaMeme universe!"})
        }
        emoji_sequence => {
            // Treat as emoji sequence
            let analysis_request = AnalysisRequest {
                emoji_sequence: emoji_sequence.to_string(),
                include_trace: false,
                session_id: request.session_id.clone(),
            };
            
            match runtime.analyze_emoji(analysis_request) {
                Ok(analysis) => json!({
                    "type": "analysis",
                    "input": analysis.input,
                    "expression": analysis.lambda_expression,
                    "reduced": analysis.reduced_expression,
                    "resonance": analysis.resonance_score,
                    "steps": analysis.reduction_steps,
                    "output": analysis.output_emoji,
                    "is_quine": analysis.is_quine
                }),
                Err(e) => json!({"type": "error", "message": e.to_string()}),
            }
        }
    };
    
    Ok(Json(ApiResponse::success(response)))
}

/// 📦 Batch processing endpoint
#[derive(Debug, Deserialize)]
struct BatchRequest {
    operations: Vec<BatchOperation>,
    session_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
enum BatchOperation {
    #[serde(rename = "poem")]
    Poem { emoji_sequence: String },
    #[serde(rename = "quine")]
    Quine { seed: String },
    #[serde(rename = "analyze")]
    Analyze { emoji_sequence: String },
    #[serde(rename = "nft")]
    Nft { emoji_sequence: String },
}

async fn batch_handler(
    State(state): State<AppState>,
    Json(request): Json<BatchRequest>,
) -> Result<Json<ApiResponse<Vec<serde_json::Value>>>, StatusCode> {
    let mut runtime = state.lock().unwrap();
    let mut results = Vec::new();
    
    for (i, operation) in request.operations.iter().enumerate() {
        let result = match operation {
            BatchOperation::Poem { emoji_sequence } => {
                let poem_request = PoemRequest {
                    emoji_sequence: emoji_sequence.clone(),
                    session_id: request.session_id.clone(),
                    max_reduction_steps: Some(50),
                };
                
                match runtime.generate_poem(poem_request) {
                    Ok(poem) => json!({"index": i, "type": "poem", "success": true, "data": poem}),
                    Err(e) => json!({"index": i, "type": "poem", "success": false, "error": e.to_string()}),
                }
            }
            BatchOperation::Quine { seed } => {
                let quine_request = QuineRequest {
                    seed: seed.clone(),
                    session_id: request.session_id.clone(),
                    max_reduction_steps: Some(50),
                };
                
                match runtime.create_quine(quine_request) {
                    Ok(quine) => json!({"index": i, "type": "quine", "success": true, "data": quine}),
                    Err(e) => json!({"index": i, "type": "quine", "success": false, "error": e.to_string()}),
                }
            }
            BatchOperation::Analyze { emoji_sequence } => {
                let analysis_request = AnalysisRequest {
                    emoji_sequence: emoji_sequence.clone(),
                    include_trace: false,
                    session_id: request.session_id.clone(),
                };
                
                match runtime.analyze_emoji(analysis_request) {
                    Ok(analysis) => json!({"index": i, "type": "analyze", "success": true, "data": analysis}),
                    Err(e) => json!({"index": i, "type": "analyze", "success": false, "error": e.to_string()}),
                }
            }
            BatchOperation::Nft { emoji_sequence } => {
                let nft_request = NFTRequest {
                    emoji_sequence: emoji_sequence.clone(),
                    session_id: request.session_id.clone(),
                    custom_attributes: None,
                };
                
                match runtime.generate_nft(nft_request) {
                    Ok(nft) => json!({"index": i, "type": "nft", "success": true, "data": nft}),
                    Err(e) => json!({"index": i, "type": "nft", "success": false, "error": e.to_string()}),
                }
            }
        };
        
        results.push(result);
    }
    
    Ok(Json(ApiResponse::success(results)))
}

/// 🧹 Cleanup endpoint
async fn cleanup_handler(State(state): State<AppState>) -> Result<Json<ApiResponse<String>>, StatusCode> {
    let runtime = state.lock().unwrap();
    match runtime.cleanup() {
        Ok(()) => Ok(Json(ApiResponse::success("Cleanup completed successfully".to_string()))),
        Err(e) => {
            error!("Cleanup failed: {}", e);
            Ok(Json(ApiResponse::error(e.to_string())))
        }
    }
}
