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
k8s_yaml(kustomize('infra/k8s/overlays/dev', flags=["--enable-helm"]))
watch_file('./infra')

# Port forwards
k8s_resource(workload='monitoring-grafana', port_forwards=3000)
k8s_resource(workload='db-stateful-set', port_forwards=5432)

# Groups
# App
k8s_resource("nginx-frontend", labels=["app"])
k8s_resource("rust-api", labels=["app"])
k8s_resource("db-stateful-set", labels=["app"])
k8s_resource("todo-cron-job", labels=["app"])
# observability
k8s_resource("monitoring-grafana", labels=["observability"])
k8s_resource("monitoring-kube-prometheus-operator", labels=["observability"])
k8s_resource("monitoring-kube-state-metrics", labels=["observability"])
k8s_resource("loki", labels=["observability"])
k8s_resource("loki-chunks-cache", labels=["observability"])
k8s_resource("loki-results-cache", labels=["observability"])
k8s_resource("alloy", labels=["observability"])
k8s_resource("loki-canary", labels=["observability"])
k8s_resource("monitoring-prometheus-node-exporter", labels=["observability"])
k8s_resource("loki-gateway", labels=["observability"])
k8s_resource("monitoring-kube-prometheus-admission-create", labels=["observability"])
k8s_resource("monitoring-kube-prometheus-admission-patch", labels=["observability"])
k8s_resource("monitoring-kube-prometheus-admission-patch", labels=["observability"])