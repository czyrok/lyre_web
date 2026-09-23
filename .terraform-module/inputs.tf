
variable "namespace_name" {
  type        = string
  description = "Name of the namespace where Lyre Web will be initialized"
}

variable "local_docker_registry_host" {
  type        = string
  description = "Host of the local Docker registry"
}

variable "traefik_app_name" {
  type        = string
  description = "Traefik pod's app name"
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

variable "traefik_tls_cert_resolver" {
  type        = string
  description = "Tells to the module the certificate resolver to use for Traefik for the provided host"
}

variable "analytics_tracker_url" {
  type        = string
  default     = null
  description = "Public Liwan tracker script URL; null disables analytics."
}

variable "analytics_api_url" {
  type        = string
  default     = null
  description = "Liwan event API URL used by the browser tracker."
}

variable "analytics_entity" {
  type        = string
  default     = null
  description = "Liwan entity associated with Lyre Web pageviews."
}

variable "wait_for" {
  type = list(string)
  default = []
}
