# Zafely - Incident Reporting and Safety Map

**Zafely** is a location-based safety and incident reporting web application that allows users to report incidents (e.g., crime, accidents) and view incidents reported nearby. The application utilizes **Rust**, **PostgreSQL** with **PostGIS** extension, and **Axum** for the API.

---

## Features

- **Report incidents**: Users can report incidents with a description and geolocation (latitude, longitude).
- **View nearby incidents**: Users can query nearby incidents based on their location (using geospatial queries).
- **Fast and secure**: Built using Rust for high performance, with PostgreSQL for storage and PostGIS for geospatial data.

---

## Getting Started

### Prerequisites

- **Rust**: You can install it from [here](https://www.rust-lang.org/tools/install).
- **PostgreSQL**: Make sure PostgreSQL is installed with the **PostGIS** extension. [PostGIS installation guide](https://postgis.net/start/).

### Installation

1. **Clone the repository**:
    ```bash
    git clone https://github.com/AncutaIoan/zafely.git
    cd zafely
    ```

2. **Install dependencies**:
    ```bash
    cargo build
    ```

3. **Set up the database**:

    - Create a PostgreSQL database (if not already done):
      ```bash
      createdb incidentsdb
      ```

    - Connect to the database and enable PostGIS:
      ```sql
      CREATE EXTENSION postgis;
      ```

    - Create the `incidents` table with a spatial column for storing geolocation:
      ```sql
      CREATE TABLE incidents (
          id SERIAL PRIMARY KEY,
          title TEXT,
          description TEXT,
          incident_time TIMESTAMPTZ DEFAULT NOW(),
          location GEOGRAPHY(Point, 4326)
      );
      ```

4. **Run the application**:
    ```bash
    cargo run
    ```

   Your API should now be running at `http://localhost:8080`.

---

## API Endpoints

### `POST /report`
**Description**: Reports a new incident.

**Request body**:
```json
{
    "title": "Robbery",
    "description": "Robbery at the metro entrance.",
    "latitude": 44.4328,
    "longitude": 26.1043
}
