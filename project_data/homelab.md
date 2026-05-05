---
next:
  slug: lrt_manager
title: HomeLab
image_url: homelab.webp
start_date: 2025-01-24
tags:
  - short_name: K8s
    long_name: Kubernetes
  - short_name: NixOS
  - short_name: Terraform
meta_keywords: |
  homelab, k8s, k3s, kubernetes, traefik, nixos, terraform, orchestration conteneurs, reverse proxy, infrastructure reproductible, configurations nixos, flakes nixos, déploiement, deployment, infrastructure as code, provisioning, provisionnement vm, vm, rotation certificats, certificates, cluster k3s, hyperviseur de type 1, hypervisor type 1, proxmox, devops, infrastructure, reproductibilité, reproducibility
description: |
  Découvrez mon homelab overkill : une infrastructure personnelle basée sur K3s pour l'orchestration de conteneurs, Traefik comme reverse proxy, NixOS pour un OS reproductible, et Terraform pour le provisionnement automatisé.
---

Parlons un peu de mon **homelab** overkill 😂. T'as compris que je ne fais jamais les choses à moitié, et mon homelab n'y échappe pas. C'est d'ailleurs là que [dylan-valentin.dev](/) est hébergé...

Mes choix techniques :
- **K3s** pour l'orchestration de mes conteneurs
- **Traefik** comme reverse proxy
- **NixOS** comme système d'exploitation
- **Terraform** pour le provisionnement

L'idée est de **provisionner** des configurations NixOS sur une VM via Terraform. NixOS est un projet merveilleux : il permet de reconstruire l'OS de manière **reproductible**, à chaque fois de la même façon, grâce aux configurations _flakes_. Pas d'effet de bord si on l'installe trois mois plus tard avec les mêmes configurations — tout est référencé dans les fichiers de configuration. Si ceux-ci ne changent pas, l'OS installé sera **identique**. Cette reproductibilité est un atout majeur et facilite les tests de l'infrastructure en **local**, car on est certain d'obtenir le même résultat en production.

Passons à K3s : puissant, léger et compatible avec l'API Kubernetes, il permet des **déploiements** simplifiés. Enfin, simplifiés une fois qu'on s'est cassé les dents sur les différentes couches (_service_, _deployment_, _pod_, _ingress_) la première fois 😆. Lors de l'installation, Terraform effectue une rotation de tous les certificats pour permettre le déploiement des **ressources Kubernetes** via cet outil. Puis, les services sont **exposés** via Traefik.

Enfin, **Terraform** est donc la colonne vertébrale du projet. Il permet à la fois de configurer l'OS, de configurer K3s et de **provisionner** mes ressources Kubernetes sur mon **nœud** K3s. Le défi a été de créer des modules Terraform pour gérer une VM NixOS et ses configurations, ainsi que la rotation des **certificats** K3s.

Cet ensemble de technologies me permet de reconstruire mon infrastructure en **3:40'**. Bon, il y a quand même deux ou trois configurations à faire avant, mais tu vois le potentiel.

Prochaine étape : un **cluster** K3s et l'utilisation d'un **hyperviseur de type 1** Proxmox 😎.
