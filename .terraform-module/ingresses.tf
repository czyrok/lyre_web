
resource "kubernetes_ingress_v1" "lyre_web_ingress" {
    metadata {
        name = "lyre-web-ingress"
        namespace = var.namespace_name

        annotations = {
            "traefik.ingress.kubernetes.io/router.entrypoints" = "websecure"
            "traefik.ingress.kubernetes.io/router.middlewares" = "${var.namespace_name}-${kubectl_manifest.compress_middleware.name}@kubernetescrd"
            "traefik.ingress.kubernetes.io/router.tls.certresolver" = var.is_development_environment ? null : "letsencrypt"
        }
    }

    spec {
        rule {
            host = var.is_development_environment ? "localhost" : "dylan-valentin.dev"
            http {
                path {
                    path = "/"
                    path_type = "Prefix"
                    backend {
                        service {
                            name = kubernetes_service.app.metadata[0].name
                            port {
                                number = 8507
                            }
                        }
                    }
                }
            }
        }
    }

    depends_on = [
        var.wait_for,
        kubectl_manifest.compress_middleware,
        kubernetes_service.app,
    ]
}

## Can't use the `kubernetes_manifest` resource due to this issue:
## https://github.com/hashicorp/terraform-provider-kubernetes/issues/1367
resource "kubectl_manifest" "compress_middleware" {
    yaml_body  = <<-EOF
        apiVersion: traefik.io/v1alpha1
        kind: Middleware
        metadata:
            name: compress
            namespace: ${var.namespace_name}
        spec:
            compress: {}
    EOF

    depends_on = [
        var.wait_for,
    ]
}
