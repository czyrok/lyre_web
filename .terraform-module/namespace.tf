resource "kubernetes_namespace_v1" "lyre_web" {
  metadata {
    name   = var.namespace_name
    labels = var.namespace_labels
  }

  depends_on = [ var.wait_for ]
}
