locals {
  traefik_configuration = file("${path.module}/../.devops/traefik/kubernetes.traefik.yaml")
  traefik_http_endpoint_external_port = 31429
  traefik_https_endpoint_external_port = 31430
  traefik_dashboard_external_port = 31431
}

locals {
  traefik_kubernetes_crd_definitions = compact(split("---", data.local_file.traefik_kubernetes_crd_definition.content))
}
