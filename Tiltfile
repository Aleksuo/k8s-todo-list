allow_k8s_contexts('k3d-dev')
default_registry('dev-registry.localhost:5000')

docker_build('rust-api',
             context='backend',
             dockerfile="backend/Dockerfile")
docker_build('nginx-frontend',
             context='frontend',
             dockerfile="frontend/Dockerfile")
k8s_yaml(kustomize('infra/k8s/bases'))