
variable "namespace_name" {
  type        = string
  description = "Name of the namespace where Lyre Web will be initialized"
}

variable "local_docker_registry_host" {
  type        = string
  description = "Host of the local Docker registry"
}

variable "wait_for" {
  type = list(string)
  default = []
}
