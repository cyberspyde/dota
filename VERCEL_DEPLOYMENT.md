# 🚀 Deploying to Vercel with Neon Database

This guide will help you deploy your Dota 2 Helper app to Vercel using Neon PostgreSQL database.

## Prerequisites

1. **Vercel Account**: Sign up at [vercel.com](https://vercel.com)
2. **Neon Database**: Create a Neon PostgreSQL database
3. **Vercel CLI**: Install the Vercel CLI
4. **Git Repository**: Your code should be in a Git repository

## Step 1: Set Up Neon Database

### Create Neon Database
1. Go to [neon.tech](https://neon.tech) and create a free account
2. Create a new project
3. Get your connection string from the dashboard
4. The connection string will look like: `postgresql://username:password@host/database?sslmode=require`

### Install Vercel CLI
```bash
npm install -g vercel
```

## Step 2: Configure Environment Variables

### Local Development
Create a `.env` file in your project root:
```env
DATABASE_URL=your_neon_connection_string_here
```

### Vercel Environment Variables
You'll set these in the Vercel dashboard or via CLI:
```bash
vercel env add DATABASE_URL
```

## Step 3: Deploy to Vercel

### Method 1: Deploy via Vercel CLI (Recommended)

1. **Login to Vercel**:
   ```bash
   vercel login
   ```

2. **Deploy your project**:
   ```bash
   vercel
   ```

3. **Follow the prompts**:
   - Link to existing project or create new
   - Set up environment variables
   - Deploy

4. **Set environment variables**:
   ```bash
   vercel env add DATABASE_URL production
   # Enter your Neon connection string when prompted
   ```

### Method 2: Deploy via Vercel Dashboard

1. **Connect your repository**:
   - Go to [vercel.com/dashboard](https://vercel.com/dashboard)
   - Click "New Project"
   - Import your Git repository

2. **Configure build settings**:
   - **Framework Preset**: Vite
   - **Build Command**: `npm run vercel-build`
   - **Output Directory**: `dist`
   - **Install Command**: `npm install`

3. **Set environment variables**:
   - Go to Project Settings > Environment Variables
   - Add `DATABASE_URL` with your Neon connection string

4. **Deploy**:
   - Click "Deploy"

## Step 4: Database Migration

### Run Migration Locally First
```bash
npm run db:migrate
```

### Or Run Migration on Vercel
You can run the migration script on Vercel using their CLI:
```bash
vercel --prod
```

## Step 5: Update API Base URL

After deployment, update the production URL in `src/services/api.ts`:

```typescript
this.baseUrl = process.env.NODE_ENV === 'production' 
  ? 'https://your-app-name.vercel.app/api'  // Replace with your actual Vercel URL
  : 'http://localhost:3000/api';
```

## Step 6: Verify Deployment

1. **Check your live site**: Visit your Vercel URL
2. **Test API endpoints**: 
   - `https://your-app.vercel.app/api/health`
   - `https://your-app.vercel.app/api/heroes`
3. **Check function logs**: In Vercel dashboard under Functions tab

## Local Development

### Start Development Server
```bash
npm run dev
```

### Test API Locally
The API routes will be available at `http://localhost:3000/api/*`

## Troubleshooting

### Common Issues

1. **Build Fails**:
   - Check Vercel build logs
   - Ensure all dependencies are in `package.json`
   - Verify Node.js version compatibility

2. **Database Connection Issues**:
   - Verify your `DATABASE_URL` environment variable
   - Check if Neon database is accessible
   - Ensure SSL is properly configured

3. **API Errors**:
   - Check Vercel function logs
   - Verify environment variables are set
   - Test database connection

### Environment Variables

Make sure these are set in Vercel:
- `DATABASE_URL`: Your Neon PostgreSQL connection string

### Local Testing

To test the Vercel functions locally:

1. **Install Vercel CLI**:
   ```bash
   npm install -g vercel
   ```

2. **Run locally**:
   ```bash
   vercel dev
   ```

This will start both your frontend and the Vercel functions locally.

## Performance Optimization

1. **Enable Caching**:
   - Add cache headers to your API responses
   - Consider using Vercel's edge caching

2. **Database Optimization**:
   - Add indexes to your database tables
   - Optimize your Prisma queries
   - Consider using connection pooling

3. **Function Optimization**:
   - Keep functions lightweight
   - Use proper error handling
   - Implement request caching where appropriate

## Monitoring

1. **Vercel Analytics**:
   - Enable analytics in your project settings
   - Monitor function execution times
   - Track error rates

2. **Database Monitoring**:
   - Monitor your Neon database performance
   - Set up alerts for connection issues
   - Track query performance

## Security Considerations

1. **Environment Variables**: Never commit sensitive data like database URLs to your repository
2. **CORS**: The API functions include CORS headers for cross-origin requests
3. **Database Security**: Use SSL connections and strong passwords
4. **Rate Limiting**: Consider implementing rate limiting for your API endpoints

## Support

If you encounter issues:

1. Check the [Vercel documentation](https://vercel.com/docs)
2. Review the [Vercel Functions documentation](https://vercel.com/docs/functions)
3. Check your Neon database documentation
4. Review the build and function logs in your Vercel dashboard

Your Dota 2 Helper app should now be live on Vercel! 🎮 