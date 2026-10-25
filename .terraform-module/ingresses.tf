
resource "kubernetes_ingress_v1" "lyre_web_ingress" {
    metadata {
        name = "lyre-web-ingress"
        namespace = var.namespace_name

        annotations = {
            "traefik.ingress.kubernetes.io/router.entrypoints" = "web,websecure"
            "traefik.ingress.kubernetes.io/router.middlewares" = "${var.namespace_name}-${kubectl_manifest.compress_middleware.name}@kubernetescrd"
            "traefik.ingress.kubernetes.io/router.tls" = var.is_development_environment ? null : "true"
            "cert-manager.io/cluster-issuer" = var.is_development_environment ? null : var.cert_manager_cluster_issuer
        }
    }

    spec {
        dynamic "tls" {
            for_each = var.is_development_environment ? [] : [var.traefik_host]

            content {
                hosts       = [tls.value]
                secret_name = "lyre-web-tls"
            }
        }

        rule {
            host = var.is_development_environment ? "localhost" : var.traefik_host
            http {
                path {
                    path = "/"
                    path_type = "Prefix"
                    backend {
                        service {
                            name = kubernetes_service_v1.app.metadata[0].name
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
        kubernetes_service_v1.app,
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
