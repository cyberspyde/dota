#!/bin/bash

echo "🚀 Dota 2 Helper - Netlify Deployment Script"
echo "=============================================="

# Check if Netlify CLI is installed
if ! command -v netlify &> /dev/null; then
    echo "❌ Netlify CLI is not installed. Installing..."
    npm install -g netlify-cli
fi

# Check if user is logged in to Netlify
if ! netlify status &> /dev/null; then
    echo "🔐 Please log in to Netlify..."
    netlify login
fi

# Build the project
echo "📦 Building the project..."
npm run build

if [ $? -ne 0 ]; then
    echo "❌ Build failed. Please check the errors above."
    exit 1
fi

echo "✅ Build completed successfully!"

# Deploy to Netlify
echo "🚀 Deploying to Netlify..."
netlify deploy --prod --dir=dist

if [ $? -eq 0 ]; then
    echo "🎉 Deployment completed successfully!"
    echo "🌐 Your site is now live on Netlify!"
    echo ""
    echo "📝 Next steps:"
    echo "1. Set up your DATABASE_URL environment variable in Netlify"
    echo "2. Run the database migration if you haven't already"
    echo "3. Test your site functionality"
else
    echo "❌ Deployment failed. Please check the errors above."
    exit 1
fi 