# 🎮 CYBERSPYDE Plays Dota - Dota 2 Helper

A comprehensive Dota 2 hero recommendation and build guide application with mood-based hero selection, detailed build guides, and multi-language support.

## ✨ Features

- **🎯 Mood-Based Hero Selection**: Choose your gaming mood and get personalized hero recommendations
- **📚 Detailed Build Guides**: Complete item builds, playstyle tips, and game phase strategies
- **🔍 Advanced Hero Search**: Filter heroes by role, difficulty, and availability of build guides
- **🌍 Multi-Language Support**: Available in English, Russian, and Uzbek
- **🌙 Dark/Light Theme**: Toggle between dark and light modes
- **📱 Responsive Design**: Optimized for desktop, tablet, and mobile devices
- **🗄️ Supabase Backend**: Modern PostgreSQL database with real-time capabilities

## 🚀 Tech Stack

### Frontend
- **React 18** with TypeScript
- **Tailwind CSS** for styling
- **Lucide React** for icons
- **Vite** for development and building

### Backend
- **Netlify Functions** with TypeScript
- **Supabase** for PostgreSQL database and authentication
- **Row Level Security** for data protection
- **CORS** enabled for cross-origin requests

## 📋 Prerequisites

Before you begin, ensure you have the following:

- **Node.js** (version 18 or higher)
- **npm** (comes with Node.js)
- **Supabase account** (free tier available)
- **Netlify account** (free tier available)

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

### 3. Supabase Setup

#### Create Supabase Project

1. **Go to [supabase.com](https://supabase.com)** and create a free account
2. **Create a new project**
3. **Wait for the project to be fully set up** (this may take a few minutes)
4. **Get your project credentials**:
   - Go to **Settings** → **API**
   - Copy your **Project URL** and **anon public key**
   - Copy your **service_role key** (keep this secret!)

#### Configure Environment Variables

1. **Copy the environment template**:
   ```bash
   cp .env.example .env
   ```

2. **Edit `.env` file** with your Supabase credentials:
   ```env
   VITE_SUPABASE_URL=https://your-project-id.supabase.co
   VITE_SUPABASE_ANON_KEY=your-anon-key-here
   SUPABASE_SERVICE_ROLE_KEY=your-service-role-key-here
   ```

### 4. Database Setup

#### Run Database Migration

1. **Go to your Supabase project dashboard**
2. **Navigate to SQL Editor**
3. **Copy and paste the content** from `supabase/migrations/01_create_dota_schema.sql`
4. **Click "Run"** to create all tables and security policies

#### Populate Database with Sample Data

```bash
npm run db:populate
```

You should see output like:
```
🚀 Populating heroes...
✅ Inserted 150+ heroes
🚀 Populating builds...
✅ Inserted 50+ builds
🎉 Database population completed successfully!
```

### 5. Start the Application

```bash
npm run dev
```

The application will start on `http://localhost:8888` (Netlify Dev server)

> **Note**: We use `netlify dev` instead of `vite` to ensure both the frontend and Netlify Functions are served locally. This allows the API endpoints (`/api/heroes`, `/api/builds`) to work correctly during development.

If you need to run only the Vite frontend server (without API functionality), you can use:
```bash
npm run dev:vite
```

## 🚀 Production Deployment

### Deploy to Netlify

1. **Connect your repository**:
   - Go to [netlify.com](https://netlify.com)
   - Click "New site from Git"
   - Connect your repository

2. **Configure build settings**:
   - **Build command**: `npm run build`
   - **Publish directory**: `dist`
   - **Functions directory**: `netlify/functions`

3. **Set environment variables**:
   - Go to **Site settings** → **Environment variables**
   - Add your Supabase credentials:
     - `VITE_SUPABASE_URL`
     - `VITE_SUPABASE_ANON_KEY`
     - `SUPABASE_SERVICE_ROLE_KEY`

4. **Deploy**:
   - Click "Deploy site"
   - Your app will be live at `https://your-site-name.netlify.app`

### Environment Variables for Production

Ensure these are set in Netlify:
- `VITE_SUPABASE_URL`: Your Supabase project URL
- `VITE_SUPABASE_ANON_KEY`: Your Supabase anonymous key
- `SUPABASE_SERVICE_ROLE_KEY`: Your Supabase service role key

### ⏰ Keeping Supabase Awake (Scheduled Keepalive)

Free Supabase projects can pause after a period of inactivity (leading to a slow first request). This repo includes a Netlify Scheduled Function (`keepalive`) that runs daily (06:00 UTC) and performs a minimal read query against the database so it stays active.

How it works:
- Function file: `netlify/functions/keepalive.ts`
- Schedule defined in `netlify.toml` under `[[scheduled.functions]]`
- Executes a single lightweight `SELECT id FROM heroes LIMIT 1`

To adjust frequency:
1. Edit the cron expression in `netlify.toml` (e.g., `0 */6 * * *` for every 6 hours)
2. Commit & deploy – Netlify applies the new schedule automatically.

To verify it runs:
- Check Netlify -> Functions -> Scheduled Functions -> Logs
- Or manually invoke: `curl https://<your-site>.netlify.app/.netlify/functions/keepalive`

If you prefer an external uptime pinger instead, you can disable this by removing the `[[scheduled.functions]]` block.

## 📁 Project Structure

```
├── netlify/
│   └── functions/         # Netlify serverless functions
├── scripts/
│   └── populate-database.ts # Database population script
├── src/
│   ├── components/        # React components
│   ├── contexts/          # React context providers
│   ├── data/             # Static data files
│   ├── lib/              # Library configurations
│   ├── services/         # API service layer
│   └── types/            # TypeScript type definitions
├── supabase/
│   └── migrations/       # Database migration files
├── .env.example          # Environment variables template
├── .env                  # Your environment variables (not in git)
├── netlify.toml          # Netlify configuration
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

## 🗄️ Database Management

### Adding New Heroes

1. **Edit the data file**: `src/data/heroes.ts`
2. **Add your hero data** following the existing format
3. **Run the population script**:
   ```bash
   npm run db:populate
   ```

### Adding New Builds

1. **Edit the data file**: `src/data/builds.ts`
2. **Add your build data** following the existing format
3. **Run the population script**:
   ```bash
   npm run db:populate
   ```

### Database Schema Updates

1. **Create a new migration file** in `supabase/migrations/`
2. **Write your SQL changes**
3. **Run the migration in Supabase SQL Editor**
4. **Update TypeScript types** if needed:
   ```bash
   npm run db:types
   ```

### Viewing Database

1. **Go to your Supabase project dashboard**
2. **Navigate to Table Editor**
3. **Browse and edit your data** directly

## 🔍 Troubleshooting

### Database Connection Issues

1. **Check Supabase project status** in the dashboard
2. **Verify environment variables** in `.env` file
3. **Test database connection**:
   ```bash
   npm run db:populate
   ```

### API Connection Issues

1. **Check Netlify function logs** in the dashboard
2. **Verify environment variables** are set in Netlify
3. **Test API directly**:
   ```bash
   curl https://your-site.netlify.app/.netlify/functions/api/health
   ```

### Build Issues

1. **Check Netlify build logs**
2. **Verify all dependencies** are in `package.json`
3. **Test build locally**:
   ```bash
   npm run build
   ```

### Migration Issues

1. **Check Supabase SQL Editor** for error messages
2. **Verify migration syntax** is correct
3. **Run migrations one at a time**
4. **Check table permissions** and RLS policies

## 📱 Features in Detail

### Mood-Based Hero Selection
- Choose from 5 different moods: Aggressive, Defensive, Experimental, Creative, Chaos
- Each mood filters heroes that match your current gaming style
- Smart algorithm considers hero complexity and playstyle

### Build Guides
- **Item Progression**: Step-by-step item builds with costs and timing
- **Playstyle Tips**: Do's, don'ts, and pro tips for each build
- **Game Plan**: Detailed strategy for early, mid, and late game phases

### Multi-Language Support
- English, Russian, and Uzbek translations
- Localized hero names and descriptions
- Cultural adaptations for different regions

### Responsive Design
- Mobile-first approach
- Tablet and desktop optimizations
- Touch-friendly interface elements

## 🤝 Contributing

1. **Fork the repository**
2. **Create a feature branch**: `git checkout -b feature/new-feature`
3. **Add your changes** to the appropriate data files
4. **Test locally** with `npm run dev`
5. **Update database** with `npm run db:populate`
6. **Commit your changes**: `git commit -am 'Add new feature'`
7. **Push to the branch**: `git push origin feature/new-feature`
8. **Submit a pull request**

### Adding New Heroes

1. **Research the hero** thoroughly
2. **Add hero data** to `src/data/heroes.ts`
3. **Include all required fields**: id, name, role, difficulty, moods, description, strengths, weaknesses
4. **Test the hero** appears correctly in the UI
5. **Add translations** if needed in `src/contexts/LanguageContext.tsx`

### Adding New Builds

1. **Create a complete build guide** with items, playstyle, and gameplan
2. **Add build data** to `src/data/builds.ts`
3. **Test the build** displays correctly
4. **Verify all item costs** and descriptions are accurate
5. **Include detailed playstyle guidance**

## 📝 License

This project is licensed under the MIT License.

## 🎮 About

Created by **CYBERSPYDE** - A comprehensive tool for Dota 2 players to discover heroes and builds based on their current gaming mood and playstyle preferences.

The application uses modern web technologies to provide a smooth, responsive experience while maintaining a beautiful, game-inspired design that appeals to Dota 2 players.

## 📚 Additional Resources

- [Supabase Documentation](https://supabase.com/docs) - Database and backend services
- [Netlify Functions Documentation](https://docs.netlify.com/functions/overview/) - Serverless functions
- [Vite Documentation](https://vitejs.dev/) - Frontend build tool
- [React Documentation](https://react.dev/) - Frontend framework
- [Tailwind CSS Documentation](https://tailwindcss.com/) - CSS framework

## 🔗 Links

- **Live Demo**: https://your-site.netlify.app
- **Supabase Dashboard**: https://supabase.com/dashboard
- **Netlify Dashboard**: https://app.netlify.com

---

Happy gaming! 🎮✨