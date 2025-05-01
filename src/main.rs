use axum::http::Method;
use axum::{
    extract::{Query, State},
    routing::{get, post},
    Json, Router,
};
use geojson::{Feature, FeatureCollection, Geometry, Value};
use http::header::HeaderName;
use serde::Deserialize;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio_postgres::NoTls;
use tower_http::cors::{Any, CorsLayer};

#[derive(Debug, Deserialize)]
struct IncidentInput {
    title: String,
    description: String,
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Deserialize)]
struct NearbyQuery {
    lat: f64,
    lon: f64,
    dist: f64,
}

#[derive(Clone)]
struct AppState {
    db: Arc<tokio_postgres::Client>,
}

#[tokio::main]
async fn main() {
    let conn_str = "host=localhost port=5432 user=postgres password=12345 dbname=postgres";

    let (client, connection) = tokio_postgres::connect(conn_str, NoTls)
        .await
        .expect("DB connection failed");

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let state = AppState {
        db: Arc::new(client),
    };

    // CORS middleware setup
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow requests from any origin (adjust this for production security)
        .allow_methods(vec![Method::GET, Method::POST]) // Allow only GET and POST methods
        .allow_headers(vec![
            HeaderName::from_static("content-type"),
            HeaderName::from_static("authorization"),
        ]);
    let app = Router::new()
        .route("/report", post(report_incident))
        .route("/nearby", get(get_nearby_incidents))
        .with_state(state)
        .layer(cors);  // Add CORS middleware here

    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("Listening on {}", addr);
    axum::serve(listener, app).await.unwrap();
}

async fn report_incident(
    State(state): State<AppState>,
    Json(payload): Json<IncidentInput>,
) -> &'static str {
    let q = "INSERT INTO incidents (title, description, location)
             VALUES ($1, $2, ST_SetSRID(ST_MakePoint($3, $4), 4326))";
    let _ = state
        .db
        .execute(q, &[&payload.title, &payload.description, &payload.longitude, &payload.latitude])
        .await;
    "Reported"
}

async fn get_nearby_incidents(
    State(state): State<AppState>,
    Query(q): Query<NearbyQuery>,
) -> Json<FeatureCollection> {
    let query = r#"
        SELECT title, description, ST_X(location::geometry), ST_Y(location::geometry)
        FROM incidents
        WHERE ST_DWithin(location, ST_SetSRID(ST_MakePoint($1, $2), 4326)::geography, $3)
    "#;

    let rows = state
        .db
        .query(query, &[&q.lon, &q.lat, &q.dist])
        .await
        .unwrap();

    let features = rows
        .into_iter()
        .map(|row| {
            let lon: f64 = row.get(2);
            let lat: f64 = row.get(3);
            Feature {
                geometry: Some(Geometry::new(Value::Point(vec![lon, lat]))),
                properties: Some(serde_json::json!({
                    "title": row.get::<_, String>(0),
                    "description": row.get::<_, String>(1),
                }).as_object().unwrap().clone()),
                ..Default::default()
            }
        })
        .collect();

    Json(FeatureCollection {
        features,
        ..Default::default()
    })
}
