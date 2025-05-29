variable "TAG" {
    default = "latest"
}

group "default" {
    targets = ["rust-api", "nginx-frontend"]
}


target "rust-api" {
    dockerfile = "Dockerfile"
    context = "backend"
    tags = ["dev-registry.localhost:5000/rust-api:${TAG}"]
}

target "nginx-frontend" {
    dockerfile = "Dockerfile"
    context = "frontend"
    tags = ["dev-registry.localhost:5000/nginx-frontend:${TAG}"]
    target = "production"
}