---
next:
  slug: unified_sso_login
title: Présence en direct
image_url: attendance_in_live.webp
start_date: 2025-03-05
end_date: 2025-07-31
tags:
  - short_name: Redis
  - short_name: Socket.io
  - short_name: TS
    long_name: TypeScript
meta_keywords: |
  legaltech, saas, ubikap, redis, socket.io, architecture stateless, stateless, assemblée générale en ligne, ag en ligne, feuille de présence ag, gestion des connexions ag, gestion multi-appareils, gestion des sessions utilisateurs, logiciel ag, gestion des événements utilisateurs, user events, cache serveur, cache
description: |
  Découvrez comment Ubikap, une LegalTech SaaS, optimise la feuille de présence des AG en ligne avec une architecture stateless, Redis et Socket.io pour une gestion fluide des connexions multi-appareils et une réactivité optimale.
links:
  - url: https://ubikap.com
    title: Ubikap
    icon: EXTERNAL
  - url: https://app.ubikap.com
    title: Ubikap App
    icon: EXTERNAL
---

Je travaille depuis plusieurs années pour une LegalTech, [Ubikap](https://ubikap.com). Si tu n'es pas familier avec ce logiciel **SaaS**, il permet de gérer des Assemblées Générales (AG) en ligne et de dématérialiser les registres de mouvements de titres.

Le sujet sur lequel je me suis penché concernait la **feuille de présence** lors d'une AG. Celle-ci devait être **réactive** aux événements de connexion et de déconnexion des utilisateurs pendant la session.

Les principales contraintes étaient les suivantes :
- Une gestion **stateless** du processus (notre load balancer répartit la charge sur une ferme de serveurs).
- La suppression du **cache** en cas de crash d'un serveur.
- La gestion des **événements** utilisateurs sur plusieurs onglets ou appareils (par exemple, un utilisateur connecté simultanément sur son téléphone et son ordinateur).

Toute la partie **stateless** a été résolue grâce à l'utilisation de **Redis** pour mettre en cache les informations et à la **propagation** des messages via **Socket.io**.

Pour la suppression du **cache** en cas de crash serveur, j'ai mis en place un système de *garbage collector*. Tu pourrais te demander pourquoi ne pas simplement utiliser une **expiration** sur le cache et laisser Redis gérer cette suppression. Bonne question ! J'ai effectivement configuré une expiration, mais avec une durée de quelques heures, correspondant à la durée de l'AG. Si j'avais opté pour une expiration plus courte (en minutes, par exemple), j'aurais dû ajouter un mécanisme pour mettre à jour en **continu** l'expiration des données dans le cache, ce qui aurait complexifié la gestion avec un grand volume de données. J'ai donc choisi une approche inverse : un *garbage collector* qui **nettoie** le cache des serveurs en cas de crash (ne t'inquiète pas, cela n'arrive jamais 😉).

Pour terminer, la dernière contrainte a été rapidement résolue en stockant les identifiants des **sessions actives** pour chaque utilisateur dans le cache. Dès qu'un utilisateur n'a plus de connexions actives, l'interface indique qu'il est déconnecté. Cela garantit une gestion **fluide** des sessions multi-onglets et multi-appareils.
