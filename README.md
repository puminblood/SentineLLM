# SentineLLM

Ce projet utilise **Rust** et **MongoDB** pour effectuer des opérations de manipulation de données. MongoDB est exécuté dans un conteneur Docker, et le programme Rust interagit avec la base de données pour effectuer des actions d'encryption, d'extraction et de gestion des données.

## Prérequis

Avant de commencer, assure-toi d'avoir les éléments suivants installés sur ta machine :

- **Docker Desktop** installé et en cours d'exécution
- **Rust** installé (suivre [la documentation officielle de Rust](https://www.rust-lang.org/tools/install))
- **Docker CLI** disponible pour gérer les conteneurs Docker

## Étapes pour configurer et exécuter le projet

### 1. **Cloner ce projet**

Si ce n'est pas déjà fait, clone ce projet sur ta machine locale :

```bash
git clone https://github.com/toncompte/tonprojet.git
cd tonprojet
```

### 2. **Configurer Docker pour MongoDB**
```bash
docker network create mongodb-network
docker run --name mongodb --network mongodb-network -d -p 27017:27017 -v mongodb-data:/data/db mongo
docker ps
```

### 3. **Execution du code**
```bash
cd manipdock
cargo run <chemin_du_fichier>
```

