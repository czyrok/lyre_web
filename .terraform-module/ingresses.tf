
# resource "kubernetes_ingress_v1" "lyre_web_ingress" {
#     metadata {
#         name = "lyre-web-ingress"
#         namespace = var.namespace_name

#         annotations = {
#             "kubernetes.io/ingress.class" = "traefik"
#             "traefik.ingress.kubernetes.io/router.entrypoints" = "web,websecure" # j'ai l'impression que ça marche avec web, le issuing du certificat
#             "traefik.ingress.kubernetes.io/router.middlewares" = "${var.namespace_name}-${kubectl_manifest.compress_middleware.name}@kubernetescrd"
#             "traefik.ingress.kubernetes.io/router.tls" = "true"
#             "cert-manager.io/issuer" = "lyre-web-letsencrypt"
#             # "traefik.ingress.kubernetes.io/router.tls.certresolver" = var.is_development_environment ? null : "letsencrypt"
#             # "traefik.ingress.kubernetes.io/router.tls.certificateRefs.name" = var.is_development_environment ? null : "lyre-web-letsencrypt"
#             # "traefik.ingress.kubernetes.io/certificatesresolvers.le.acme.email" = "pro@dylan-valentin.dev"
#             # "traefik.ingress.kubernetes.io/certificatesresolvers.le.acme.storage" = "/data/acme.json"
#             # "traefik.ingress.kubernetes.io/certificatesresolvers.le.acme.httpchallenge.entrypoint" = "web"
#         }
#     }

#     spec {
#         rule {
#             host = var.is_development_environment ? "localhost" : "dylan-valentin.dev"
#             http {
#                 path {
#                     path = "/"
#                     path_type = "Prefix"
#                     backend {
#                         service {
#                             name = kubernetes_service.app.metadata[0].name
#                             port {
#                                 number = 8507
#                             }
#                         }
#                     }
#                 }
#             }
#         }

#         # tls {
#         #     secret_name = "lyre-web-letsencrypt"
#         # }
   
#     }

#     depends_on = [
#         var.wait_for,
#         kubectl_manifest.compress_middleware,
#         kubectl_manifest.certificate_issuer,
#         kubectl_manifest.certificate,
#         kubectl_manifest.gateway_with_tls,
#         kubernetes_service.app,
#     ]
# }

# TODO: var lyre-web-gateway
resource "kubectl_manifest" "http_route" {
    yaml_body  = <<-EOF
        apiVersion: gateway.networking.k8s.io/v1
        kind: HTTPRoute
        metadata:
          name: http-route
          namespace: ${var.namespace_name}
        spec:
            parentRefs:
            - name: lyre-web-gateway
              sectionName: web
            - name: lyre-web-gateway
              sectionName: websecure
            hostnames:
            - "dylan-valentin.dev"
            rules:
            - matches:
              - path:
                  type: PathPrefix
                  value: /
              backendRefs:
              - name: ${kubernetes_service.app.metadata[0].name}
                port: 8507
              filters:
              - type: ExtensionRef
                extensionRef:
                  group: traefik.io
                  kind: Middleware
                  name: compress
    EOF

    depends_on = [
        var.wait_for,
        kubectl_manifest.compress_middleware,
        kubectl_manifest.certificate_issuer,
        kubectl_manifest.certificate,
        kubectl_manifest.gateway_with_tls,
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

# replace to use https://acme-v02.api.letsencrypt.org/directory
resource "kubectl_manifest" "certificate_issuer" {
    yaml_body = <<-EOF
        apiVersion: cert-manager.io/v1
        kind: Issuer
        metadata:
            name: certificate-issuer
            namespace: ${var.namespace_name}
        spec:
            acme:
                email: pro@dylan-valentin.dev
                server: https://acme-staging-v02.api.letsencrypt.org/directory
                privateKeySecretRef:
                    name: lyre-web-letsencrypt
                solvers:
                    - http01:
                        gatewayHTTPRoute:
                            parentRefs:
                                - name: gateway_with_tls
                                  namespace: ${var.namespace_name}
                                  kind: Gateway
    EOF

    depends_on = [
        var.wait_for,
    ]
}
# TODO: use var for lyre-web-ingress
# TODO: use var for gateway_with_tls

resource "kubectl_manifest" "certificate" {
    # TODO: use var for "lyre-web-letsencrypt"
    yaml_body = <<-EOF
        apiVersion: cert-manager.io/v1
        kind: Certificate
        metadata:
            name: certificate
            namespace: ${var.namespace_name}
        spec:
            secretName: lyre-web-letsencrypt
            dnsNames:
                - "dylan-valentin.dev"
            issuerRef:
                name: certificate-issuer
                kind: Issuer
    EOF

    depends_on = [
        var.wait_for,
    ]
}

resource "kubectl_manifest" "gateway_with_tls" {
    # TODO: use var for "lyre-web-letsencrypt"
    yaml_body = <<-EOF
        apiVersion: gateway.networking.k8s.io/v1
        kind: Gateway
        metadata:
            name: lyre-web-gateway
            namespace: ${var.namespace_name}
        spec:
            gatewayClassName: traefik
            listeners:
                - name: web
                  port: 80
                  protocol: HTTP
                  allowedRoutes:
                      namespaces:
                          from: Same
                - name: websecure
                  port: 443
                  protocol: HTTPS
                  allowedRoutes:
                      namespaces:
                          from: Same
            tls:
                certificateRefs:
                    - name: lyre-web-letsencrypt
    EOF

    depends_on = [
        var.wait_for,
    ]
}





# resource "kubectl_manifest" "certificate_resolver" {
#   yaml_body = <<-EOF
#     apiVersion: traefik.io/v1alpha1
#     kind: Ingeress
#     metadata:
#         name: certificate-resolver
#         namespace: ${var.namespace_name}
#     spec:
#         certificatesResolvers:
#             letsencrypt:
#             acme:
#                 email: pro@dylan-valentin.dev
#                 storage: acme.json
#                 httpChallenge:
#                     entryPoint: web
#     EOF

#     depends_on = [
#         var.wait_for,
#     ]
# }
