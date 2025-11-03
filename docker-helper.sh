#!/bin/bash

# Rust MVC API - Docker Helper Script
# Usage: ./docker-helper.sh [build|run|stop|logs|test|clean]

set -e

IMAGE_NAME="rust-mvc-api"
CONTAINER_NAME="rust-api"
PORT="8080"

case "$1" in
    build)
        echo "🔨 Building Docker image..."
        docker build -t "$IMAGE_NAME:latest" -f Dockerfile .
        echo "✅ Build complete!"
        ;;
    
    run)
        echo "🚀 Starting container..."
        docker run -d \
            --name "$CONTAINER_NAME" \
            -p "$PORT:$PORT" \
            -e SERVER_HOST=0.0.0.0 \
            -e SERVER_PORT=$PORT \
            -e RUST_LOG=info \
            "$IMAGE_NAME:latest"
        
        echo "✅ Container started on http://localhost:$PORT"
        echo "📚 API docs available at: http://localhost:$PORT/docs"
        echo "❤️  Health check: http://localhost:$PORT/health"
        ;;
    
    stop)
        echo "🛑 Stopping container..."
        docker stop "$CONTAINER_NAME" 2>/dev/null || echo "Container not running"
        docker rm "$CONTAINER_NAME" 2>/dev/null || echo "Container not found"
        echo "✅ Container stopped and removed"
        ;;
    
    logs)
        echo "📋 Container logs:"
        docker logs -f "$CONTAINER_NAME"
        ;;
    
    test)
        echo "🧪 Testing API endpoints..."
        
        # Wait for container to be ready
        echo "⏳ Waiting for API to be ready..."
        timeout=30
        count=0
        while [ $count -lt $timeout ]; do
            if curl -s http://localhost:$PORT/health > /dev/null 2>&1; then
                break
            fi
            sleep 1
            count=$((count + 1))
        done
        
        if [ $count -eq $timeout ]; then
            echo "❌ API failed to start within $timeout seconds"
            exit 1
        fi
        
        echo "✅ API is ready!"
        
        # Test endpoints
        echo "🔍 Testing health endpoint..."
        curl -f -s http://localhost:$PORT/health && echo " ✅ Health check passed"
        
        echo "🔍 Testing projects endpoint..."
        curl -f -s http://localhost:$PORT/api/v1/projects && echo " ✅ Projects endpoint working"
        
        echo "🔍 Testing OpenAPI docs..."
        curl -f -s http://localhost:$PORT/openapi.json > /dev/null && echo " ✅ OpenAPI spec accessible"
        
        echo "🎉 All tests passed!"
        ;;
    
    clean)
        echo "🧹 Cleaning up..."
        docker stop "$CONTAINER_NAME" 2>/dev/null || true
        docker rm "$CONTAINER_NAME" 2>/dev/null || true
        docker rmi "$IMAGE_NAME:latest" 2>/dev/null || true
        echo "✅ Cleanup complete"
        ;;
    
    *)
        echo "Rust MVC API - Docker Helper"
        echo ""
        echo "Usage: $0 [command]"
        echo ""
        echo "Commands:"
        echo "  build   - Build the Docker image"
        echo "  run     - Start the container"
        echo "  stop    - Stop and remove the container"
        echo "  logs    - Show container logs"
        echo "  test    - Test API endpoints"
        echo "  clean   - Remove container and image"
        echo ""
        echo "Examples:"
        echo "  $0 build && $0 run     # Build and run"
        echo "  $0 test                # Test the running API"
        echo "  $0 stop                # Stop the container"
        ;;
esac