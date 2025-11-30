
resource "kubernetes_deployment" "app" {
  metadata {
    name = "lyre-web-app"
    namespace = var.namespace_name

    labels = {
      app = "app"
    }

    annotations = {
      "keel.sh/policy" = "force",
      "keel.sh/trigger" = "poll"
      "keel.sh/match-tag" = "true"
      "keel.sh/pollSchedule" = "@every 1m"
    }
  }

  spec {
    replicas = 1

    selector {
      match_labels = {
        app = "app"
      }
    }

    strategy {
      type = "RollingUpdate"
      rolling_update {
        max_surge = "2"
        max_unavailable = "0"
      }
    }

    template {
      metadata {
        labels = {
          app = "app"
        }
      }

      spec {
        container {
          image = "${var.local_docker_registry_host}/lyre/lyre_web:latest-amd64"
          image_pull_policy = "Always"
          name  = "app"

          env {
            # TODO: ajouter secret plus tard
            name = "CONTENT_TOTP_URI"
            value = "1234"
          }

          port {
            container_port = 8507
          }
        }
      }
    }
  }

  depends_on = [ var.wait_for ]
}

resource "kubernetes_service" "app" {
  metadata {
    name = "app"
    namespace = var.namespace_name
  }

  spec {
    selector = {
      app = kubernetes_deployment.app.metadata[0].labels.app
    }

    port {
      port = 8507
      target_port = 8507
    }

    type = "ClusterIP"
  }

  depends_on = [ var.wait_for ]
}

resource "kubernetes_pod_disruption_budget_v1" "app" {
  metadata {
    name      = "app"
    namespace = var.namespace_name
  }

  spec {
    min_available = 1
    
    selector {
      match_labels = {
        app = kubernetes_deployment.app.metadata[0].labels.app
      }
    }
  }
}
