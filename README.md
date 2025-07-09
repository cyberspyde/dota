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
- **Vercel Serverless Functions** with Node.js
- **Neon PostgreSQL** database
- **Prisma ORM** for database operations
- **CORS** enabled for cross-origin requests

## 📋 Prerequisites

Before you begin, ensure you have the following installed:

- **Node.js** (version 16 or higher)
- **npm** (comes with Node.js)
- **Vercel CLI** (for deployment)
- **Neon PostgreSQL** database

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

#### Create Neon Database

1. **Go to [neon.tech](https://neon.tech)** and create a free account
2. **Create a new project** and get your connection string
3. **The connection string will look like**: `postgresql://username:password@host/database?sslmode=require`

#### Configure Environment Variables

1. **Copy the environment template**:
   ```bash
   cp .env.example .env
   ```

2. **Edit `.env` file** with your database credentials:
   ```env
   # Database Configuration
   DATABASE_URL=postgresql://username:password@host/database?sslmode=require
   ```

### 4. Database Migration

Run the migration script to create tables and populate data:

```bash
npm run db:migrate
```

You should see output like:
```
✅ Database schema created successfully
✅ Inserted XXX heroes successfully
✅ Inserted XXX builds successfully
🎉 Migration completed successfully!
```

### 5. Start the Application

#### Option 1: Start Both Frontend and Backend Together (Recommended)

```bash
npm run dev
```

This will start:
- Frontend development server on `http://localhost:5173`
- Vercel functions will be available at `http://localhost:3000/api`

#### Option 2: Start Services Separately

**Terminal 1 - Frontend:**
```bash
npm run dev
```

**Terminal 2 - Backend (optional for local development):**
```bash
npm run dev:server
```

### 6. Access the Application

Open your browser and navigate to:
- **Frontend**: http://localhost:5173
- **API**: http://localhost:3000/api

## 📁 Project Structure

```
├── api/                    # Vercel serverless functions
│   ├── heroes/            # Hero-related API endpoints
│   ├── builds/            # Build-related API endpoints
│   └── health.js          # Health check endpoint
├── server/                # Local development server
│   ├── index.js           # Express server and API routes
│   ├── database.js        # PostgreSQL connection setup
│   └── migrate.js         # Database migration script
├── src/
│   ├── components/        # React components
│   ├── contexts/          # React context providers
│   ├── data/             # Static data files (legacy)
│   ├── services/         # API service layer
│   └── types/            # TypeScript type definitions
├── prisma/               # Prisma schema and migrations
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

## 🚀 Production Deployment

### Deploy to Vercel

1. **Install Vercel CLI**:
   ```bash
   npm install -g vercel
   ```

2. **Login to Vercel**:
   ```bash
   vercel login
   ```

3. **Deploy your project**:
   ```bash
   vercel
   ```

4. **Set environment variables**:
   ```bash
   vercel env add DATABASE_URL production
   ```

### Environment Variables for Production

Update your `.env` for production:

```env
DATABASE_URL=your_neon_production_database_url
```

## 🔍 Troubleshooting

### Database Connection Issues

1. **Check Neon database is accessible**:
   ```bash
   psql "your_neon_connection_string"
   ```

2. **Verify environment variables** in `.env` file

3. **Test database connection**:
   ```bash
   npm run db:migrate
   ```

### API Connection Issues

1. **Verify Vercel functions are running** locally or on production
2. **Check browser console** for CORS or network errors
3. **Test API directly**:
   ```bash
   curl http://localhost:3000/api/health
   ```

### Port Conflicts

If ports 3000 or 5173 are in use:

1. **Change API port** in Vercel configuration
2. **Update API URL** in `src/services/api.ts`

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

## 📚 Additional Resources

- [Vercel Deployment Guide](VERCEL_DEPLOYMENT.md) - Detailed deployment instructions
- [Prisma Documentation](https://www.prisma.io/docs/) - Database ORM documentation
- [Neon Documentation](https://neon.tech/docs) - PostgreSQL hosting documentation