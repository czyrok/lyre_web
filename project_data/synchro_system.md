---
next:
  slug: lyre_web
title: Système de Synchro
image_url: XX
start_date: 2026-02-02
end_date: 2026-02-24
tags:
  - name: gRPC
  - name: Redis
  - name: TS
meta_keywords: |
  partenariats logiciels, software partnerships, synchronisation logicielle, software synchronization, intégration logicielle, software integration, interopérabilité logicielle, software interoperability, architecture gRPC, gRPC architecture, gRPC-gateway, Redis, TypeScript, tâches de synchronisation, synchronization tasks, API gRPC, gRPC API, service Reflection, test API Postman, Postman API testing, génération de code client, client code generation, API JSON HTTP 1.1, JSON API over HTTP 1.1, développement logiciel, software development, optimisation des partenariats, partnership optimization, intégration technique, technical integration, solutions logicielles, software solutions, Ubikap, synchronisation automatisée, automated synchronization, architecture microservices, microservices architecture, optimisation des flux de données, data flow optimization
description: |
  Ubikap optimise la croissance de son logiciel grâce à des partenariats techniques et une synchronisation automatisée entre systèmes. Notre architecture repose sur des technologies modernes comme gRPC, gRPC-gateway, Redis et TypeScript pour garantir une interopérabilité fluide et une intégration simplifiée.
links:
  - url: https://ubikap.com
    title: Ubikap
    icon: EXTERNAL
  - url: https://app.ubikap.com
    title: Ubikap App
    icon: EXTERNAL
---

Un **vecteur** clé de croissance et de développement pour un logiciel tel que [Ubikap](https://ubikap.com), ce sont les partenariats. Pour nos partenariats actuels, nous avons choisi d'assumer la responsabilité de la **synchronisation** entre le logiciel de nos partenaires et le nôtre.

Plusieurs technologies sont à l'honneur :
- **gRPC**
- **gRPC-gateway**
- **Redis**
- **TypeScript**

L'architecture de ce système se compose de 2 applications : l'une expose un serveur gRPC pour poster les **tâches de synchro** sur Redis, l'autre a pour objectif d'**exécuter** ces tâches selon les paramètres donnés. gRPC nous permet d'exposer l'endpoint avec le **service Reflection**, ce qui facilite le test de notre API via [Postman](https://www.postman.com/), ainsi que l'implémentation de notre système de synchro dans les logiciels des partenaires, grâce à la possibilité de **générer** le code client à partir des fichiers `.proto` du serveur gRPC.

Afin de maximiser l'**interopérabilité** avec notre logiciel, nous avons mis à disposition une 3e application, qui expose cette fois le serveur gRPC sous forme d'API JSON en HTTP 1.1 ([gRPC-gateway](https://github.com/grpc-ecosystem/grpc-gateway)).
