---
next:
  slug: gasc
title: LRT Manager
image_url: lrt_manager.png
date: 2023-05-10
tags:
  - name: RxJS
  - name: Socket.io
  - name: Angular
---

LRT Manager est un petit projet de stage qui consistait en la réalisation d'une application permettant la gestion de fichiers spéciaux (fichiers LRT) et leur sauvegarde. Ces fichiers étaient contenus dans des boitiers SBC (Session Border Controller) au sein des centres de données de l'entreprise. Ce boitier est utilisé pour assurer la sécurité des infrastructures de téléphonie SIP.

L'application permettait leur modification via des séquences programmées de traitements (exemple : séquence d'ajouts de plusieurs numéros de téléphone). Ainsi, le but de l'application était de faciliter l'ajout de nouveaux numéros de téléphone dans les fichiers LRT et de donner la possibilité de revenir à une sauvegarde précédente permettant un « rollback », tout ça en utilisant des séquences de traitements.

L'avantage de ce système est qu'il est possible de composer n'importe quel type de séquence avec les traitements qu'on souhaite et de la manière dont on le souhaite. Ce dernier point est important puisque ça veut dire qu'on peut aller jusqu'à programmer des traitements en parallèle. Pour finir, le fonctionnement des séquences utilise les fonctionnalités de RxJS d'où la possibilité de composer ce qu'on souhaite au sein d'une séquence.
