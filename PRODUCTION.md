# Production Deployment Guide

This guide explains how to build and deploy the TPC-C playground for production use.

## How It Works

### Automatic Mode Detection

**Backend (Rust/Axum)**: Detects mode based on presence of built frontend
- **Production Mode** (`ui-vite-react/dist/` exists): Serves static files + API with `/api/` prefix, no CORS
- **Development Mode** (`ui-vite-react/dist/` missing): API only with CORS enabled

**Frontend (React/Vite)**: Detects mode based on the port it's accessed from
- **Development** (port 5173/3000): Uses `http://localhost:8080/orders` (cross-origin)
- **Production** (port 8080/80/443): Uses `/api/orders` (same-origin relative paths)

### File Structure Trigger

```
tccp-playground/
├── rust-axum-rest-api/
│   └── cargo run                 # Auto-detects ../ui-vite-react/dist/
└── ui-vite-react/
    ├── src/                     # Source code
    └── dist/                    # Built assets (triggers production mode)
```

The presence of the `dist/` directory switches both backend routing and frontend API detection.

## Quick Start

### 1. Build the Frontend

```bash
cd ui-vite-react
npm run build
```

This creates a `dist/` directory with the optimized, production-ready frontend.

### 2. Start the Production Server

```bash
cd rust-axum-rest-api
cargo run --release
```

The server will automatically detect the built frontend and serve it along with the API.

### 3. Access the Application

Open your browser to: http://localhost:8080

- Frontend: `http://localhost:8080/` (served from `ui-vite-react/dist/`)
- API: `http://localhost:8080/api/` (all API endpoints prefixed with `/api/`)

## How It Works

### Backend Changes

The Axum router now has two modes:

1. **Development Mode** (when `ui-vite-react/dist/` doesn't exist):
   - Serves API routes directly at root level (`/warehouses`, `/orders`, etc.)
   - Frontend served separately by Vite dev server on port 5173

2. **Production Mode** (when `ui-vite-react/dist/` exists):
   - Serves API routes under `/api/` prefix (`/api/warehouses`, `/api/orders`, etc.)
   - Serves static frontend files at root level (`/`, `/orders`, etc.)
   - SPA fallback: all non-API routes serve `index.html` for client-side routing

### Frontend Changes

The frontend API configuration automatically detects the environment:

- **Development**: Uses `http://localhost:8080` (Axum dev server)
- **Production**: Uses `/api` (relative paths to same server)
- **Custom**: Uses `VITE_API_BASE_URL` environment variable if set

## Environment Variables

### Backend (.env)

```bash
# Database configuration
DATABASE_URL=postgres://username:password@localhost/tpcc

# Server configuration (optional)
PORT=8080
HOST=0.0.0.0
```

### Frontend (.env)

```bash
# Optional: Override API base URL
# VITE_API_BASE_URL=https://your-production-domain.com/api
```

## Deployment Options

### 1. Single Binary Deployment

Build and deploy just the Rust binary:

```bash
# Build frontend first
cd ui-vite-react && npm run build

# Build optimized backend
cd ../rust-axum-rest-api
cargo build --release

# Deploy the binary and ensure ui-vite-react/dist/ is accessible
./target/release/rust-axum-rest-api
```

### 2. Docker Deployment

Create a `Dockerfile`:

```dockerfile
FROM node:18-alpine AS frontend-builder
WORKDIR /app/frontend
COPY ui-vite-react/package*.json ./
RUN npm ci --only=production
COPY ui-vite-react/ ./
RUN npm run build

FROM rust:1.75-slim AS backend-builder
WORKDIR /app
COPY rust-axum-rest-api/ ./
RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=backend-builder /app/target/release/rust-axum-rest-api .
COPY --from=frontend-builder /app/frontend/dist ./ui-vite-react/dist/
EXPOSE 8080
CMD ["./rust-axum-rest-api"]
```

### 3. Development vs Production

```bash
# Development (two servers)
cd rust-axum-rest-api && cargo run &  # Backend API at :8080/orders
cd ui-vite-react && npm run dev &     # Frontend at :5173 → calls localhost:8080

# Production (single server)  
cd ui-vite-react && npm run build     # Creates dist/ directory
cd ../rust-axum-rest-api && cargo run # Combined at :8080 → frontend calls /api/orders
```

## Route Structure

### Production URLs

- **Frontend Routes** (served from `dist/index.html`):
  - `http://localhost:8080/` - Dashboard
  - `http://localhost:8080/new-order` - New Order page
  - `http://localhost:8080/orders` - Order Management page

- **API Endpoints** (JSON responses):
  - `http://localhost:8080/api/warehouses`
  - `http://localhost:8080/api/orders`
  - `http://localhost:8080/api/new-order` (POST)
  - `http://localhost:8080/api/order-status`
  - And all other existing endpoints...

### Development URLs

- **Frontend**: `http://localhost:5173/` (Vite dev server)
- **API**: `http://localhost:8080/warehouses` (direct, no `/api/` prefix)

## Troubleshooting

### "API not found" errors

Make sure you built the frontend first:
```bash
cd ui-vite-react && npm run build
```

### Static files not serving

Verify the `dist/` directory exists:
```bash
ls -la ui-vite-react/dist/
```

### SPA routing issues (404 on refresh)

If you get 404 errors when refreshing frontend routes like `/new-order`:
- This is automatically handled by the SPA fallback configuration
- All non-API routes serve `index.html` to enable client-side routing
- Make sure you rebuilt the frontend after any routing changes

### CORS issues

In production mode, CORS is automatically handled since frontend and API are served from the same origin.

### Router nesting error

If you get `Nesting at the root is no longer supported. Use fallback_service instead.`:
- This was fixed by using `fallback_service` instead of `nest_service` for static file serving
- The fix is already implemented in the current version

### API environment detection

The frontend automatically detects whether it's running in development or production:
- **Development** (port 5173/3000): Uses `http://localhost:8080` for API calls
- **Production** (port 8080 or standard ports): Uses `/api` for API calls
- **Custom**: Set `VITE_API_BASE_URL` environment variable

### Database connection

Ensure your `.env` file in `rust-axum-rest-api/` has the correct `DATABASE_URL`.

## Performance Notes

- Frontend assets are optimized and minified by Vite
- Static file serving is handled efficiently by `tower-http`
- API responses include appropriate caching headers
- SPA routing is handled with fallback to `index.html`

## Security Considerations

- All API routes require the `/api/` prefix in production
- Static file serving is restricted to the `dist/` directory
- CORS is properly configured for cross-origin requests
- SQL injection protection via parameterized queries
- Input validation on all endpoints

---

*This production setup provides a single, deployable artifact that serves both the modern React frontend and the high-performance Rust API.*