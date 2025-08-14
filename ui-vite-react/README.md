# TPC-C Order Management UI

A modern React-based web interface for the TPC-C Order Management System.

## 🛠️ Tech Stack

- **Vite** - Build tool and dev server
- **React 18** - UI library with TypeScript
- **Material-UI** - Component library with default styling
- **TanStack Router** - Type-safe routing
- **TanStack Query** - Server state management
- **TypeScript** - Type safety and developer experience

## 🚀 Getting Started

### Prerequisites
- Node.js 20.19+ (currently using v20.18.0 with warnings)
- Rust Axum API server running on http://localhost:8080

### Installation & Development

```bash
# Install dependencies
npm install

# Start development server
npm run dev

# Build for production
npm run build

# Preview production build
npm run preview
```

The development server will start on http://localhost:3000

## 📁 Project Structure

```
src/
├── components/     # Reusable UI components
├── hooks/         # Custom React hooks
├── pages/         # Page components (legacy, using routes/ now)
├── routes/        # TanStack Router route definitions
├── services/      # API service functions
├── types/         # TypeScript type definitions
└── config/        # Configuration files
```

## 🎯 Features (Planned)

### Phase 1 - New Order Entry ✨
- [x] Project setup and routing
- [ ] Warehouse/District selection
- [ ] Customer search and selection
- [ ] Dynamic order lines with item search
- [ ] Real-time price calculation
- [ ] Form validation and submission

### Phase 2 - Orders Management 
- [ ] Orders list with filtering
- [ ] Order status indicators
- [ ] Sortable data table with pagination
- [ ] Order detail view

### Phase 3 - Advanced Features
- [ ] Carrier assignment for deliveries
- [ ] Dashboard with metrics
- [ ] Advanced filtering and search

## 🔌 API Integration

The UI connects to the Rust Axum TPC-C API server:
- Base URL: `http://localhost:8080` (configurable via `VITE_API_BASE_URL`)
- Uses existing endpoints: `/new-order`, `/warehouses`, etc.
- New endpoints needed: `/items`, `/customers`, `/orders`, `/districts`

## 🧪 Development

### Current Status
- ✅ Project scaffolded with Vite + React + TypeScript
- ✅ Material-UI theme and components installed
- ✅ TanStack Router configured with basic routes
- ✅ TanStack Query setup for API state management
- ✅ Basic navigation and layout
- ✅ Type definitions for TPC-C entities

### Next Steps
1. Implement warehouses/districts API calls
2. Create customer search functionality  
3. Build dynamic order line components
4. Add form validation and submission
5. Connect to existing `/new-order` API endpoint

### Development Tools
- React Query Devtools: Available in development
- TanStack Router Devtools: Available in development
- TypeScript strict mode enabled
- ESLint configured for code quality