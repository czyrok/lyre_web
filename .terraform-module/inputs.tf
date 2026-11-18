
variable "namespace_name" {
  type        = string
  default     = "lyre-web"
  description = "Namespace Lyre Web is deployed into. Created by this module."
}

variable "namespace_labels" {
  type        = map(string)
  default     = {}
  description = "Labels set on the namespace this module creates."
}

variable "local_docker_registry_host" {
  type        = string
  description = "Host of the local Docker registry"
}

variable "is_development_environment" {
  type        = bool
  default     = true
  description = "Tells to the module if It should load production things like domain certificates"
}

variable "traefik_host" {
  type        = string
  description = "Tells to the module the host to use for Traefik"
}

variable "cert_manager_cluster_issuer" {
  type        = string
  default     = null
  description = "cert-manager ClusterIssuer that issues the production certificate; unused in development"
}

variable "analytics_tracker_url" {
  type        = string
  default     = null
  description = "Public Umami tracker script URL; null disables analytics."
}

variable "analytics_website_id" {
  type        = string
  default     = null
  description = "Umami website ID associated with Lyre Web pageviews."
}

variable "analytics_domains" {
  type        = string
  default     = null
  description = "Comma-separated hostnames the Umami tracker is allowed to run on; it stays silent anywhere else."
}

variable "wait_for" {
  type = list(string)
  default = []
}
