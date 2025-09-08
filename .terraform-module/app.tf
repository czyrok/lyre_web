
resource "kubernetes_pod" "app" {
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
        container {
            image = "${var.local_docker_registry_host}/lyre/lyre_web:latest-arm64"
            name  = "app"

            env {
                # TODO:
                name = "CONTENT_TOTP_URI"
                value = "1234"
            }

            port {
                container_port = 8507
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
      app = kubernetes_pod.app.metadata[0].labels.app
    }

    port {
      port = 8507
      target_port = 8507
    }

    type = "ClusterIP"
  }

  depends_on = [ var.wait_for ]
}
