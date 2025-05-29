variable "TAG" {
    default = "latest"
}

group "default" {
    targets = ["rust-api"]
}
target "rust-api" {
    dockerfile = "Dockerfile"
    context = "."
    tags = ["dev-registry.localhost:5000/rust-api:${TAG}"]
}