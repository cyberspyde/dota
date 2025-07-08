# 🎮 CYBERSPYDE Plays Dota - Dota 2 Helper

A comprehensive Dota 2 hero recommendation and build guide application with mood-based hero selection, detailed build guides, and multi-language support.

## ✨ Features

- **🎯 Mood-Based Hero Selection**: Choose your gaming mood and get personalized hero recommendations
- **📚 Detailed Build Guides**: Complete item builds, playstyle tips, and game phase strategies
- **🔍 Advanced Hero Search**: Filter heroes by role, difficulty, and availability of build guides
- **🌍 Multi-Language Support**: Available in English, Russian, and Uzbek
- **🌙 Dark/Light Theme**: Toggle between dark and light modes
- **📱 Responsive Design**: Optimized for desktop, tablet, and mobile devices
- **🗄️ PostgreSQL Backend**: Robust database storage for heroes and builds data

## 🚀 Tech Stack

### Frontend
- **React 18** with TypeScript
- **Tailwind CSS** for styling
- **Lucide React** for icons
- **Vite** for development and building

### Backend
- **Node.js** with Express
- **PostgreSQL** database
- **CORS** enabled for cross-origin requests

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

- **Node.js** (version 16 or higher)
- **npm** (comes with Node.js)
- **PostgreSQL** (version 12 or higher)

## 🛠️ Local Development Setup

### 1. Clone the Repository

```bash
git clone <your-repository-url>
cd dota-helper
```

### 2. Install Dependencies

```bash
npm install
```

### 3. Database Setup

#### Create PostgreSQL Database

1. **Start PostgreSQL service** (varies by OS):
   ```bash
   # macOS (with Homebrew)
   brew services start postgresql
   
   # Ubuntu/Debian
   sudo systemctl start postgresql
   
   # Windows (if installed as service)
   net start postgresql
   ```

2. **Create database and user**:
   ```sql
   -- Connect to PostgreSQL as superuser
   psql -U postgres
   
   -- Create database
   CREATE DATABASE dota_helper;
   
   -- Create user (optional, you can use existing user)
   CREATE USER your_username WITH PASSWORD 'your_password';
   
   -- Grant privileges
   GRANT ALL PRIVILEGES ON DATABASE dota_helper TO your_username;
   
   -- Exit psql
   \q
   ```

#### Configure Environment Variables

1. **Copy the environment template**:
   ```bash
   cp .env.example .env
   ```

2. **Edit `.env` file** with your database credentials:
   ```env
   # Database Configuration
   DATABASE_URL=postgresql://your_username:your_password@localhost:5432/dota_helper
   DB_HOST=localhost
   DB_PORT=5432
   DB_NAME=dota_helper
   DB_USER=your_username
   DB_PASSWORD=your_password
   
   # Server Configuration
   PORT=3001
   ```

### 4. Database Migration

Run the migration script to create tables and populate data:

```bash
node server/migrate.js
```

You should see output like:
```
✅ Database schema created successfully
🧹 Cleared existing heroes data
✅ Inserted XXX heroes successfully
🧹 Cleared existing builds data
✅ Inserted XXX builds successfully
🎉 Migration completed successfully!
```

### 5. Start the Application

#### Option 1: Start Both Frontend and Backend Together (Recommended)

```bash
npm run dev:full
```

This will start:
- Backend API server on `http://localhost:3001`
- Frontend development server on `http://localhost:5173`

#### Option 2: Start Services Separately

**Terminal 1 - Backend:**
```bash
npm run dev:server
```

**Terminal 2 - Frontend:**
```bash
npm run dev
```

### 6. Access the Application

Open your browser and navigate to:
- **Frontend**: http://localhost:5173
- **API**: http://localhost:3001/api

## 📁 Project Structure

```
├── server/                 # Backend API server
│   ├── index.js           # Express server and API routes
│   ├── database.js        # PostgreSQL connection setup
│   ├── migrate.js         # Database migration script
│   └── schema.sql         # Database schema definition
├── src/
│   ├── components/        # React components
│   ├── contexts/          # React context providers
│   ├── data/             # Static data files (legacy)
│   ├── services/         # API service layer
│   └── types/            # TypeScript type definitions
├── .env.example          # Environment variables template
├── .env                  # Your environment variables (not in git)
└── README.md            # This file
```

## 🔧 API Endpoints

The backend provides the following REST API endpoints:

- `GET /api/health` - Health check
- `GET /api/heroes` - Get all heroes
- `GET /api/heroes/:id` - Get specific hero
- `GET /api/builds` - Get all builds
- `GET /api/heroes/:heroId/builds` - Get builds for specific hero
- `GET /api/heroes/:heroId/builds/:mood` - Get specific build

## 🔍 Troubleshooting

### Database Connection Issues

1. **Check PostgreSQL is running**:
   ```bash
   pg_isready -h localhost -p 5432
   ```

2. **Verify database exists**:
   ```bash
   psql -U your_username -d dota_helper -c "SELECT 1;"
   ```

3. **Check environment variables** in `.env` file

### API Connection Issues

1. **Verify backend server is running** on port 3001
2. **Check browser console** for CORS or network errors
3. **Test API directly**:
   ```bash
   curl http://localhost:3001/api/health
   ```

### Port Conflicts

If ports 3001 or 5173 are in use:

1. **Change API port** in `.env`:
   ```env
   PORT=3002
   ```

2. **Update API URL** in `src/services/api.ts`:
   ```typescript
   const API_BASE_URL = 'http://localhost:3002/api';
   ```

## 🚀 Production Deployment

### Build for Production

```bash
npm run build
```

### Environment Variables for Production

Update your `.env` for production:

```env
DATABASE_URL=your_production_database_url
DB_HOST=your_production_host
DB_PORT=5432
DB_NAME=your_production_db
DB_USER=your_production_user
DB_PASSWORD=your_production_password
PORT=3001
```

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/new-feature`
3. Commit your changes: `git commit -am 'Add new feature'`
4. Push to the branch: `git push origin feature/new-feature`
5. Submit a pull request

## 📝 License

This project is licensed under the MIT License.

## 🎮 About

Created by **CYBERSPYDE** - A comprehensive tool for Dota 2 players to discover heroes and builds based on their current gaming mood and playstyle preferences.