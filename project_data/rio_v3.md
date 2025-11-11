---
title: Rio
image_url: rio_v3.webp
date: 2024-07-23
tags:
  - name: Socket.io
  - name: Express.js
  - name: TS
meta_keywords: |
  framework, abstraction, encapsulation, Express.js, Socket.io, décorateurs TypeScript, injection de dépendance, typage, NestJS, choix techniques
description: |
  Rio v3 est un framework expérimental inspiré de NestJS, utilisant des décorateurs TypeScript pour encapsuler Express.js et Socket.io. Ce projet explore l'injection de dépendance et l'abstraction, tout en révélant les défis du typage et en préparant ma transition vers Rust pour des solutions plus robustes.
links:
  - url: https://github.com/czyrok/rio
    title: GitHub
    icon: GITHUB
---

Rio était un mini **framework** composé d'outils et d'éléments **abstraits** permettant l'encapsulation de librairies de communication client-serveur comme Express.js ou Socket.io. Le but était de pouvoir utiliser des décorateurs TypeScript afin de faciliter le développement d'applications utilisant ces technologies tout en rajoutant des fonctionnalités adjacentes comme de l'**injection de dépendance** à l'instar de NestJS. La différence était que Rio devait permettre une bien meilleure intégration des librairies en ayant une base de code totalement indépendante des intégrations.

Bien que ce projet fût une ébauche des possibilités, il m'a confronté au problème des **décorateurs**, à savoir leur manque de typage qui rend l'expérience de développement moins intuitif. Par la suite, j'ai étudié une version améliorée de ce projet qui se passe complètement des décorateurs TypeScript au profit de fonctions hautement typées. Ce n'est jamais allé plus loin car j'ai fini par me tourner vers le Rust pour mes futurs **choix techniques**.