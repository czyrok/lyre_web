
data "local_file" "traefik_kubernetes_crd_definition" {
  filename = "${path.module}/../.devops/traefik/kubernetes-crd-definition-v1.yaml"
}
