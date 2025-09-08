
locals {
  traefik_app_name = "traefik-ingress-controller"
}

resource "kubernetes_service_account" "traefik-ingress-controller" {
  metadata {
    name      = "traefik-ingress-controller"
    namespace = var.namespace_name
    labels = {
      app = local.traefik_app_name
    }
  }

  depends_on = [ var.wait_for ]
}

resource "kubernetes_cluster_role" "traefik-ingress-controller" {
    # Docs: https://raw.githubusercontent.com/traefik/traefik/v3.5/docs/content/reference/dynamic-configuration/kubernetes-crd-rbac.yml

    metadata {
        name = "traefitraefik-ingress-controllerk"
    }

    rule {
        api_groups = [""]
        resources  = ["services", "secrets", "nodes", "configmaps"]
        verbs      = ["get", "list", "watch"]
    }

    rule {
        api_groups = [""]
        resources  = ["pods"]
        verbs      = ["get"]
    }

    rule {
        api_groups = ["discovery.k8s.io"]
        resources  = ["endpointslices"]
        verbs      = ["list", "watch"]
    }

    rule {
        api_groups = ["extensions", "networking.k8s.io"]
        resources  = ["ingresses", "ingressclasses"]
        verbs      = ["get", "list", "watch"]
    }

    rule {
        api_groups = ["extensions", "networking.k8s.io"]
        resources  = ["ingresses/status"]
        verbs      = ["update"]
    }

    rule {
        api_groups = ["traefik.io"]
        resources  = [
            "middlewares",
            "middlewaretcps",
            "ingressroutes",
            "traefikservices",
            "ingressroutetcps",
            "ingressrouteudps",
            "tlsoptions",
            "tlsstores",
            "serverstransports",
            "serverstransporttcps"
        ]
        verbs      = ["get", "list", "watch"]
    }

    depends_on = [ var.wait_for ]
}

resource "kubernetes_cluster_role_binding" "traefik-ingress-controller" {
  metadata {
    name = "traefik-ingress-controller"
  }

  role_ref {
    api_group = "rbac.authorization.k8s.io"
    kind      = "ClusterRole"
    name      = kubernetes_cluster_role.traefik-ingress-controller.metadata[0].name
  }

  subject {
    kind      = "ServiceAccount"
    name      = kubernetes_service_account.traefik-ingress-controller.metadata[0].name
    namespace = var.namespace_name
  }

  depends_on = [ var.wait_for ]
}

resource "kubernetes_pod" "traefik-ingress-controller" {
    metadata {
        name = "traefik-ingress-controller"
        namespace = var.namespace_name
        labels = {
            app = local.traefik_app_name
        }
    }

    spec {
        service_account_name = kubernetes_service_account.traefik-ingress-controller.metadata[0].name

        container {
            image = "traefik:v3.4.4"
            name  = "traefik-ingress-controller"
            command = ["traefik"]
            args = [
                "--providers.file=true",
                ## Debug mode
                # "--log.level=DEBUG"
            ]

            volume_mount {
                mount_path = "/etc/traefik/traefik.yml"
                name = "traefik-config"
                sub_path = "traefik.yml"
                read_only = true
            }
        }

        volume {
            name = "traefik-config"
            config_map {
                name = kubernetes_config_map.traefik-ingress-controller.metadata[0].name
            }
        }
    }

    depends_on = [ var.wait_for, kubernetes_service.app ]
}

resource "kubernetes_config_map" "traefik-ingress-controller" {
    metadata {
        name = "traefik-ingress-controller"
        namespace = var.namespace_name
    }
    data = {
        "traefik.yml" = local.traefik_configuration
    }

    depends_on = [ var.wait_for ]
}

resource "kubernetes_service" "traefik-ingress-controller" {
    metadata {
        name = "traefik-ingress-controller"
        namespace = var.namespace_name
    }

    spec {
        selector = {
            app = kubernetes_pod.traefik-ingress-controller.metadata[0].labels.app
        }

        port {
            name="http"
            port = 80
            target_port = 80
            node_port = local.traefik_http_endpoint_external_port
        }
        port {
            name="https"
            port = 443
            target_port = 443
            node_port = local.traefik_https_endpoint_external_port
        }
        port {
            name="dashboard"
            port = 8080
            target_port = 8080
            node_port = local.traefik_dashboard_external_port
        }

        type = "NodePort"
    }

    depends_on = [ var.wait_for ]
}

resource "kubectl_manifest" "traefik_crd_definition" {
    count = length(local.traefik_kubernetes_crd_definitions)

    yaml_body  = element(local.traefik_kubernetes_crd_definitions, count.index)

    depends_on = [ var.wait_for ]
}
