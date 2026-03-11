---
next:
  slug: rio_v3
title: Zod vers Proto
image_url: XX
start_date: 2025-12-14
tags:
  - name: gRPC
  - name: NPM
  - name: TS
meta_keywords: |
  gRPC, JavaScript, proto, .proto, génération automatique proto, Zod, schéma Zod, Zod schema, typage TypeScript, inférence de types, type inference, contrôleurs gRPC, @grpc/grpc-js, bibliothèque open source, open source, écosystème JavaScript, développement backend, technologies émergentes, validation de données, génération de code, intégration gRPC, Node.js gRPC, outils pour développeurs, protobuf JavaScript
description: |
 Découvrez une bibliothèque open source pour générer automatiquement des fichiers .proto à partir de schémas Zod et simplifier l'intégration de gRPC dans l'écosystème JavaScript. Idéal pour les développeurs cherchant à optimiser leur workflow backend avec typage strict et validation de données.
links:
  - url: https://github.com/czfabrics/zod-to-proto
    title: GitHub
    icon: GITHUB
  - url: https://www.npmjs.com/package/@czfabrics/zod-to-proto
    title: Package
    icon: NPM
---

Parmi mes hobbies, il n'y a pas seulement mon homelab, mais aussi le développement de libs **open source**. Mon but est d'adresser certains besoins dans de noubelles technos.

L'un des principaux murs auquel on se confronte lorsqu'on commence à utiliser **gRPC**, c'est la rédaction des fichiers **`.proto`**. Ces fichiers reposent sur une **syntaxe** assez différente de ce qu'on trouve en TypeScript. De plus, cette syntaxe est bien plus **stricte** que celle de TS en matière de typage.

Cette lib vise donc à faciliter l'**adoption** de gRPC dans l'**écosystème JavaScript**, en permettant la génération automatique des fichiers `.proto` à partir d'objets JavaScript. Plus précisément, à partir de **schémas [Zod](https://github.com/colinhacks/zod)**.

Pour rappel, **Zod** est une lib qui permet de déclarer des objets JavaScript pour effectuer de la validation, tout en permettant l'inférence de types. **Concrètement**, tu peux utiliser Zod en entrée de ton controller et générer les fichiers `.proto` correspondant au schéma Zod de ton endpoint. Le même principe s'applique si tu veux typer la sortie de ton controller.

La **prochaine étape** sera de développer une autre lib pour déclarer les controllers et les traduire en définitions compatibles avec la bibliothèque historique **[@grpc/grpc-js](https://www.npmjs.com/package/@grpc/grpc-js)**.
