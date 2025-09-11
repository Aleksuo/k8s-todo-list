allow_k8s_contexts('k3d-dev')
default_registry('dev-registry.localhost:5000')

docker_build('rust-api',
             context='backend',
             dockerfile="backend/Dockerfile")
docker_build('nginx-frontend',
             context='frontend',
             target='production',
            # live_update=[sync('./frontend/src', '/usr/src/app/src')],
             dockerfile="frontend/Dockerfile")
k8s_resource(
    workload='nginx-frontend',
    links=[
        link('http://localhost:8081/', 'App')
    ]
)
k8s_yaml(kustomize('infra/k8s/bases'))