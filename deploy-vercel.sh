#!/bin/bash

echo "🚀 Dota 2 Helper - Vercel Deployment Script"
echo "=============================================="

# Check if Vercel CLI is installed
if ! command -v vercel &> /dev/null; then
    echo "❌ Vercel CLI is not installed. Installing..."
    npm install -g vercel
fi

# Check if user is logged in to Vercel
if ! vercel whoami &> /dev/null; then
    echo "🔐 Please log in to Vercel..."
    vercel login
fi

# Build the project
echo "📦 Building the project..."
npm run build

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please check the errors above."
    exit 1
fi

echo "✅ Build completed successfully!"

# Deploy to Vercel
echo "🚀 Deploying to Vercel..."
vercel --prod

if [ $? -eq 0 ]; then
    echo "🎉 Deployment completed successfully!"
    echo "🌐 Your site is now live on Vercel!"
    echo ""
    echo "📝 Next steps:"
    echo "1. Set up your DATABASE_URL environment variable in Vercel"
    echo "2. Run the database migration if you haven't already"
    echo "3. Test your site functionality"
    echo "4. Update the production URL in src/services/api.ts"
else
    echo "❌ Deployment failed. Please check the errors above."
    exit 1
fi 