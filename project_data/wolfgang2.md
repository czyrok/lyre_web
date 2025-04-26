---
title: Wolfgang2
image_url: # TODO:
date: 2025-10-21 # TODO:
tags:
  - name: TS
  - name: Socket.io
---

Le sujet de ce projet était la transformation du jeu de société Les loups-garous de Thiercelieux en un jeu en ligne multijoueur. C'est lors de celui-ci que j'ai compris l'importance de la phase de conception dans un projet de développement informatique. Le moteur du jeu fut le résultat de plusieurs jours de travail. Mais cette phase était nécessaire vu la complexité non pas du jeu en lui-même mais des interactions possibles en son sein. De plus, à cause du fait que le jeu soit multijoueur, l'architecture a dû être conçue en conséquence.

Pour faire court, nous avions vu les choses en grand, le projet se compose de 3 applications backend. Une des applications sert simplement pour les interactions avec les fonctionnalités basiques du site web (authentification avec JWT etc...). Une autre correspond à l'API pour la gestion du jeu en lui-même (gestion des parties via des workers NodeJS). Et la dernière sert au lien entre les deux premières, car l'API du site web et le serveur de jeu ont été conçus pour être redimensionnés (plusieurs instances dans le Cloud). Par conséquent, cette 3e application backend renvoie la liste des parties créées à l'API du site web et commande la création de nouvelles parties aux serveurs de jeu.

L'architecture et la conception ne sont pas les seuls points qui ont émergé avec ce projet mais c'est aussi le travail en équipe qui fût nécessaire pour arriver à terminer le projet. Avec mon équipe, nous avons appliqué la méthode agile pour une meilleure communication au sein de l'équipe ainsi qu'une meilleure organisation.
