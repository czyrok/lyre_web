output "namespace" {
  value = kubernetes_namespace_v1.lyre_web.metadata[0].name
}

output "health_checks" {
  value = [
    { name = "Website", url = "https://${var.traefik_host}", conditions = ["[STATUS] == 200"], headers = {}, public = true },
  ]
}

output "homepage_cards" {
  value = [
    { name = "Website", href = "https://${var.traefik_host}", icon = "mdi-web", description = "Portfolio", public = true },
  ]
}
