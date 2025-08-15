# TPC-C UI (Vite + React)

A modern React frontend for the TPC-C benchmark system, built with Vite, TypeScript, and Material-UI.

## 🎯 Project Status

### ✅ **Phase 1: New Order Transaction - COMPLETED**
Complete end-to-end new order flow with full TPC-C compliance:
- **Progressive form flow**: Warehouse → District → Customer → Items → Submission
- **Dynamic order lines**: Add/remove items with real-time stock checking
- **Smart validation**: Client-side validation with comprehensive error handling
- **Order confirmation**: Beautiful success page with detailed order breakdown
- **TPC-C compliance**: Proper stock management, tax calculations, and business rules

### 🚀 **Next Phase: Order Management**
Ready to implement Phase 2 - Order Status and Management:
- Order lookup and search
- Order details display
- Order history
- Status tracking

## 🛠️ Tech Stack

- **React 18** with TypeScript
- **Vite** for fast development and building
- **TanStack Router** for type-safe routing
- **TanStack Query** for server state management and caching
- **Material-UI (MUI)** for components and responsive design
- **ESLint** for code quality

## 🚀 Getting Started

### Prerequisites
- Node.js 18+ 
- npm or yarn
- Rust backend API running on port 8080

### Installation
```bash
npm install
```

### Development
```bash
npm run dev
```
Navigate to http://localhost:5173

### Build
```bash
npm run build
```

### Type Checking
```bash
npm run type-check
```

## 📁 Project Structure

```
src/
├── components/         # Reusable UI components
│   ├── WarehouseSelect.tsx      # Warehouse selection dropdown
│   ├── DistrictSelect.tsx       # District selection (filtered by warehouse)
│   ├── CustomerAutocomplete.tsx # Customer search with debouncing
│   ├── ItemAutocomplete.tsx     # Item search and selection
│   ├── SupplyWarehouseSelect.tsx # Supply warehouse override
│   ├── OrderLines.tsx           # Dynamic order lines management
│   └── OrderConfirmation.tsx    # Order success confirmation
├── routes/             # Route components (TanStack Router)  
│   ├── __root.tsx      # Root layout with navigation
│   ├── index.tsx       # Landing/dashboard page
│   └── new-order.tsx   # Complete new order flow
├── hooks/              # Custom React hooks
│   ├── useWarehouses.ts    # Warehouse data fetching
│   ├── useDistricts.ts     # District data fetching  
│   ├── useCustomers.ts     # Customer search with debouncing
│   ├── useItems.ts         # Item search and stock info
│   ├── useOrderSubmission.ts # Order submission mutation
│   └── useDebounce.ts      # Generic debounce hook
├── services/           # API service functions
│   ├── warehouseService.ts  # Warehouse API calls
│   ├── districtService.ts   # District API calls
│   ├── customerService.ts   # Customer search API
│   ├── itemService.ts       # Item search and stock API
│   └── orderService.ts      # Order submission API
├── utils/              # Utility functions
│   └── orderValidation.ts   # Order form validation logic
├── types/              # TypeScript type definitions
├── config/             # Configuration files
│   └── api.ts          # API endpoints and configuration
└── main.tsx            # Application entry point
```

## 🎯 Key Features Implemented

### **New Order Transaction**
- **Progressive Disclosure**: Each step unlocks the next (W→D→C→Items)
- **Smart Search**: Debounced autocomplete for customers and items
- **Real-time Validation**: Live stock checking and form validation
- **Dynamic Order Lines**: Add/remove items with quantity controls
- **Supply Warehouse Override**: Per-item warehouse selection
- **Price Calculations**: Live totals with tax and discount application
- **Order Confirmation**: Professional order summary with detailed breakdown
- **Error Handling**: Comprehensive validation and user-friendly error messages

### **Technical Features**
- **Type Safety**: Full TypeScript coverage with strict typing
- **Performance**: React Query caching and debounced searches
- **Responsive Design**: Works on desktop, tablet, and mobile
- **Accessibility**: Proper ARIA labels and keyboard navigation
- **State Management**: Clean state handling with automatic resets

## 🔧 Available Scripts

- `npm run dev` - Start development server with HMR
- `npm run build` - Build for production
- `npm run preview` - Preview production build
- `npm run type-check` - Run TypeScript type checking
- `npm run lint` - Run ESLint

## 🌍 Environment Variables

Create a `.env` file for local configuration:

```env
VITE_API_BASE_URL=http://localhost:8080
```

## 🚀 Testing the New Order Flow

1. **Start the Rust backend** (port 8080)
2. **Start the React dev server** (`npm run dev`)
3. **Navigate to `/new-order`**
4. **Complete the flow**:
   - Select warehouse and district
   - Search and select a customer
   - Add items with quantities
   - Review and submit order
   - See confirmation page

## 📊 TPC-C Business Rules Implemented

- **Stock Management**: Allows over-stock orders with TPC-C overflow handling (+91 units)
- **Order Limits**: Maximum 99 items per line, 15 lines per order
- **Tax Calculations**: Warehouse and district tax application
- **Remote Orders**: Cross-warehouse order tracking
- **Brand/Generic**: Proper item classification logic

## 🎯 Next Steps

### **Phase 2: Order Management** (Ready to Implement)
- Order status lookup and search
- Order details and history display
- Order status tracking and updates

### **Phase 3: Payment Processing**
- Customer payment interface
- Payment history and balance management

### **Phase 4: Delivery Management**  
- Delivery processing workflow
- Batch delivery operations

### **Phase 5: Reporting & Analytics**
- Stock level reports
- Performance dashboards
- TPC-C metrics visualization