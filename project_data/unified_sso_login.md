---
next:
  slug: synchro_system
title: Login Multi-SSO
image_url: unified_sso_login.webp
start_date: 2025-07-03
end_date: 2025-12-01
tags:
  - name: OIDC
  - name: Auth
  - name: Redis
meta_keywords: |
  sso, single sign-on, openid connect, oidc, authentification unique, authentication, sécurité des accès, intégration sso, stateless, architecture stateless, redis, gestion des sessions, expiration des tokens, token, authorization code flow, protection des données, scalabilité, résilience, resilience, resiliency, expérience utilisateur, user experience, gestion des erreurs, error handling, stockage éphémère, temporary storage
description: |
  Ubikap simplifie la connexion à son logiciel grâce à une solution multi-SSO basée sur OpenID Connect. La sécurité est au rendez-vous avec courte expiration des tokens, l'utilisation du Authorization Code Flow et une architecture sateless et résiliente.
links:
  - url: https://ubikap.com
    title: Ubikap
    icon: EXTERNAL
  - url: https://app.ubikap.com
    title: Ubikap App
    icon: EXTERNAL
---

La gestion des identifiants et mots de passe peut rapidement devenir un casse-tête pour les utilisateurs, surtout dans un contexte professionnel où les outils se multiplient. Chez [Ubikap](https://ubikap.com), il existe donc un enjeu majeur : **simplifier** cette expérience en permettant à chaque client d'utiliser son propre **SSO** (Single Sign-On).

Pour répondre à cette problématique, la mise en place d'un système multi-SSO compatible avec **OpenID Connect** s'est imposée comme la solution la plus adaptée. Grâce à cette approche, les utilisateurs peuvent se connecter de manière **transparente**, sans avoir à mémoriser de nouveaux identifiants, tout en bénéficiant d'une **intégration** fluide avec leurs infrastructures existantes.

La **sécurité** a été placée au cœur de ce système. Pour la garantir, j'ai mis en œuvre :
- Une **expiration** courte des tokens,
- L'**Authorization Code Flow**, une méthode robuste pour sécuriser les échanges d'autorisation.
- Une architecture **stateless**, rendue possible grâce à **Redis** pour la gestion des sessions et des états temporaires.

Ces mesures réduisent les **risques** liés aux accès non autorisés et renforcent la **protection** des données.

Redis joue un rôle clé dans ce mécanisme : il permet de stocker les **états** d'authentification de manière **éphémère**, tout en assurant des performances optimales et une **scalabilité** horizontale. L'externalisation de ces informations dans Redis, simplifie la montée en charge et améliore la **résilience** des services.

Enfin, pour offrir une **expérience utilisateur** optimale, j'ai intégré une gestion des erreurs **unifiée**. En cas de problème (authentification échouée, session expirée, etc.), l'utilisateur reçoit un message **clair** et **précis** sur la nature de l'erreur, ce qui améliore l'expérience utilisateur.
