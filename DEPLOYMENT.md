# 🚀 Deploying to Netlify

This guide will help you deploy your Dota 2 Helper app to Netlify.

## Prerequisites

1. **Netlify Account**: Sign up at [netlify.com](https://netlify.com)
2. **PostgreSQL Database**: You'll need a hosted PostgreSQL database (e.g., Supabase, Railway, or Neon)
3. **Git Repository**: Your code should be in a Git repository (GitHub, GitLab, etc.)

## Step 1: Set Up Your Database

### Option A: Supabase (Recommended)
1. Go to [supabase.com](https://supabase.com) and create a free account
2. Create a new project
3. Go to Settings > Database to get your connection string
4. Run the migration script locally to populate your database:
   ```bash
   npm run db:migrate
   ```

### Option B: Railway
1. Go to [railway.app](https://railway.app) and create an account
2. Create a new PostgreSQL database
3. Get the connection string from the database settings

### Option C: Neon
1. Go to [neon.tech](https://neon.tech) and create an account
2. Create a new project with PostgreSQL
3. Get the connection string from the dashboard

## Step 2: Deploy to Netlify

### Method 1: Deploy via Netlify UI (Recommended)

1. **Connect Your Repository**:
   - Go to [app.netlify.com](https://app.netlify.com)
   - Click "New site from Git"
   - Choose your Git provider (GitHub, GitLab, etc.)
   - Select your repository

2. **Configure Build Settings**:
   - **Build command**: `npm run build`
   - **Publish directory**: `dist`
   - **Node version**: `18` (or higher)

3. **Set Environment Variables**:
   - Go to Site settings > Environment variables
   - Add the following variable:
     - **Key**: `DATABASE_URL`
     - **Value**: Your PostgreSQL connection string (e.g., `postgresql://username:password@host:port/database`)

4. **Deploy**:
   - Click "Deploy site"
   - Wait for the build to complete

### Method 2: Deploy via Netlify CLI

1. **Install Netlify CLI**:
   ```bash
   npm install -g netlify-cli
   ```

2. **Login to Netlify**:
   ```bash
   netlify login
   ```

3. **Initialize and Deploy**:
   ```bash
   netlify init
   netlify deploy --prod
   ```

## Step 3: Configure Your Domain

1. **Custom Domain** (Optional):
   - Go to Site settings > Domain management
   - Add your custom domain
   - Follow the DNS configuration instructions

2. **Default Domain**:
   - Netlify provides a default domain like `your-app-name.netlify.app`
   - You can customize this in Site settings > Domain management

## Step 4: Verify Deployment

1. **Check Your Site**:
   - Visit your Netlify URL
   - Test the hero search functionality
   - Verify that build guides load correctly

2. **Check Function Logs**:
   - Go to Functions tab in your Netlify dashboard
   - Check for any errors in the API function logs

## Troubleshooting

### Common Issues

1. **Build Fails**:
   - Check the build logs in Netlify
   - Ensure all dependencies are in `package.json`
   - Verify Node.js version compatibility

2. **API Errors**:
   - Check the Functions tab for error logs
   - Verify your `DATABASE_URL` environment variable
   - Ensure your database is accessible from Netlify

3. **Database Connection Issues**:
   - Verify your database connection string
   - Check if your database allows external connections
   - Ensure SSL is properly configured

### Environment Variables

Make sure these environment variables are set in Netlify:
- `DATABASE_URL`: Your PostgreSQL connection string
- `NODE_ENV`: Should be `production` (set automatically by Netlify)

### Local Testing

To test the Netlify function locally:

1. **Install Netlify CLI**:
   ```bash
   npm install -g netlify-cli
   ```

2. **Run Locally**:
   ```bash
   netlify dev
   ```

This will start both your frontend and the Netlify functions locally.

## Performance Optimization

1. **Enable Caching**:
   - Add cache headers to your Netlify function responses
   - Consider using Netlify's edge caching

2. **Database Optimization**:
   - Add indexes to your database tables
   - Optimize your SQL queries
   - Consider using connection pooling

3. **Function Optimization**:
   - Keep functions lightweight
   - Use proper error handling
   - Implement request caching where appropriate

## Monitoring

1. **Netlify Analytics**:
   - Enable analytics in your site settings
   - Monitor function execution times
   - Track error rates

2. **Database Monitoring**:
   - Monitor your database performance
   - Set up alerts for connection issues
   - Track query performance

## Support

If you encounter issues:

1. Check the [Netlify documentation](https://docs.netlify.com/)
2. Review the [Netlify Functions documentation](https://docs.netlify.com/functions/overview/)
3. Check your database provider's documentation
4. Review the build and function logs in your Netlify dashboard

## Security Considerations

1. **Environment Variables**: Never commit sensitive data like database URLs to your repository
2. **CORS**: The Netlify function includes CORS headers for cross-origin requests
3. **Database Security**: Use SSL connections and strong passwords
4. **Rate Limiting**: Consider implementing rate limiting for your API endpoints

Your Dota 2 Helper app should now be live on Netlify! 🎮 