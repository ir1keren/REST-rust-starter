# Rust MVC API - Docker Helper Script (PowerShell)
# Usage: .\docker-helper.ps1 [build|run|stop|logs|test|clean]

param(
    [Parameter(Mandatory=$true)]
    [ValidateSet("build", "run", "stop", "logs", "test", "clean", "help")]
    [string]$Command
)

$IMAGE_NAME = "rust-mvc-api"
$CONTAINER_NAME = "rust-api"
$PORT = "8080"

function Show-Help {
    Write-Host "Rust MVC API - Docker Helper" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "Usage: .\docker-helper.ps1 [command]" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "Commands:" -ForegroundColor Green
    Write-Host "  build   - Build the Docker image"
    Write-Host "  run     - Start the container"
    Write-Host "  stop    - Stop and remove the container"
    Write-Host "  logs    - Show container logs"
    Write-Host "  test    - Test API endpoints"
    Write-Host "  clean   - Remove container and image"
    Write-Host ""
    Write-Host "Examples:" -ForegroundColor Magenta
    Write-Host "  .\docker-helper.ps1 build"
    Write-Host "  .\docker-helper.ps1 run"
    Write-Host "  .\docker-helper.ps1 test"
    Write-Host "  .\docker-helper.ps1 stop"
}

function Build-Image {
    Write-Host "🔨 Building Docker image..." -ForegroundColor Yellow
    docker build -t "${IMAGE_NAME}:latest" -f Dockerfile .
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Build complete!" -ForegroundColor Green
    } else {
        Write-Host "❌ Build failed!" -ForegroundColor Red
        exit 1
    }
}

function Start-Container {
    Write-Host "🚀 Starting container..." -ForegroundColor Yellow
    
    docker run -d `
        --name $CONTAINER_NAME `
        -p "${PORT}:${PORT}" `
        -e SERVER_HOST=0.0.0.0 `
        -e SERVER_PORT=$PORT `
        -e RUST_LOG=info `
        "${IMAGE_NAME}:latest"
    
    if ($LASTEXITCODE -eq 0) {
        Write-Host "✅ Container started on http://localhost:$PORT" -ForegroundColor Green
        Write-Host "📚 API docs available at: http://localhost:$PORT/docs" -ForegroundColor Cyan
        Write-Host "❤️  Health check: http://localhost:$PORT/health" -ForegroundColor Cyan
    } else {
        Write-Host "❌ Failed to start container!" -ForegroundColor Red
        exit 1
    }
}

function Stop-Container {
    Write-Host "🛑 Stopping container..." -ForegroundColor Yellow
    
    try {
        docker stop $CONTAINER_NAME 2>$null
        docker rm $CONTAINER_NAME 2>$null
        Write-Host "✅ Container stopped and removed" -ForegroundColor Green
    } catch {
        Write-Host "⚠️  Container might not be running" -ForegroundColor Yellow
    }
}

function Show-Logs {
    Write-Host "📋 Container logs:" -ForegroundColor Cyan
    docker logs -f $CONTAINER_NAME
}

function Test-API {
    Write-Host "🧪 Testing API endpoints..." -ForegroundColor Yellow
    
    # Wait for container to be ready
    Write-Host "⏳ Waiting for API to be ready..." -ForegroundColor Cyan
    $timeout = 30
    $count = 0
    
    while ($count -lt $timeout) {
        try {
            $response = Invoke-WebRequest -Uri "http://localhost:$PORT/health" -UseBasicParsing -TimeoutSec 1
            if ($response.StatusCode -eq 200) {
                break
            }
        } catch {
            # API not ready yet
        }
        Start-Sleep -Seconds 1
        $count++
    }
    
    if ($count -eq $timeout) {
        Write-Host "❌ API failed to start within $timeout seconds" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "✅ API is ready!" -ForegroundColor Green
    
    # Test endpoints
    try {
        Write-Host "🔍 Testing health endpoint..." -ForegroundColor Cyan
        $health = Invoke-WebRequest -Uri "http://localhost:$PORT/health" -UseBasicParsing
        Write-Host " ✅ Health check passed" -ForegroundColor Green
        
        Write-Host "🔍 Testing projects endpoint..." -ForegroundColor Cyan
        $projects = Invoke-WebRequest -Uri "http://localhost:$PORT/api/v1/projects" -UseBasicParsing
        Write-Host " ✅ Projects endpoint working" -ForegroundColor Green
        
        Write-Host "🔍 Testing OpenAPI docs..." -ForegroundColor Cyan
        $openapi = Invoke-WebRequest -Uri "http://localhost:$PORT/openapi.json" -UseBasicParsing
        Write-Host " ✅ OpenAPI spec accessible" -ForegroundColor Green
        
        Write-Host "🎉 All tests passed!" -ForegroundColor Green
    } catch {
        Write-Host "❌ API tests failed: $($_.Exception.Message)" -ForegroundColor Red
        exit 1
    }
}

function Remove-Everything {
    Write-Host "🧹 Cleaning up..." -ForegroundColor Yellow
    
    try {
        docker stop $CONTAINER_NAME 2>$null
        docker rm $CONTAINER_NAME 2>$null
        docker rmi "${IMAGE_NAME}:latest" 2>$null
        Write-Host "✅ Cleanup complete" -ForegroundColor Green
    } catch {
        Write-Host "⚠️  Some cleanup operations failed (items might not exist)" -ForegroundColor Yellow
    }
}

# Main execution
switch ($Command) {
    "build" { Build-Image }
    "run" { Start-Container }
    "stop" { Stop-Container }
    "logs" { Show-Logs }
    "test" { Test-API }
    "clean" { Remove-Everything }
    "help" { Show-Help }
    default { Show-Help }
}