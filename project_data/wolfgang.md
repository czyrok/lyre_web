---
next:
  slug: kepler
title: Wolfgang
image_url: wolfgang.webp
date: 2022-11-06
tags:
  - name: Socket.io
  - name: Angular
  - name: TS
meta_keywords: |
  jeu en ligne multijoueur, Les loups-garous de Thiercelieux, architecture, backend, workers NodeJS, instances Cloud, JWT, authentification, méthode agile, travail en équipe, communication
description: |
  Wolfgang transforme le jeu de société 'Les Loups-Garous' en une expérience multijoueur en ligne. Ce projet met en lumière mon expertise en architecture backend (NodeJS, workers, Cloud), conception complexe et travail d'équipe en méthode agile. Un défi technique et organisationnel relevé avec succès.
links:
  - url: https://github.com/czyrok/wolfgang
    title: GitHub
    icon: GITHUB
---

Le sujet de ce projet était la transformation du jeu de société Les loups-garous de Thiercelieux en un jeu en **ligne multijoueur**. C'est lors de celui-ci que j'ai compris l'importance de la phase de **conception** dans un projet de développement informatique. Le moteur du jeu fut le résultat de plusieurs jours de travail. Mais cette phase était nécessaire vu la complexité non pas du jeu en lui-même mais des interactions possibles en son sein. De plus, à cause du fait que le jeu soit multijoueur, l'**architecture** a dû être conçue en conséquence.

Pour faire court, nous avions vu les choses en grand, le projet se compose de 3 applications backend. Une des applications sert simplement pour les interactions avec les fonctionnalités basiques du site web (authentification avec JWT etc...). Une autre correspond à l'API pour la gestion du jeu en lui-même (gestion des parties via des **workers NodeJS**). Et la dernière sert au lien entre les deux premières, car l'API du site web et le serveur de jeu ont été conçus pour être redimensionnés (plusieurs **instances** dans le Cloud). Par conséquent, cette 3e application backend renvoie la liste des parties créées à l'API du site web et commande la création de nouvelles parties aux serveurs de jeu.

L'architecture et la conception ne sont pas les seuls points qui ont émergé avec ce projet mais c'est aussi le **travail en équipe** qui fût nécessaire pour arriver à terminer le projet. Avec mon équipe, nous avons appliqué la **méthode agile** pour une meilleure communication au sein de l'équipe ainsi qu'une meilleure organisation.