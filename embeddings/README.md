Projet Rust - Extraction de texte, génération d'embeddings et stockage dans MongoDB
Ce projet permet d'extraire du texte d'un fichier PDF, de générer des embeddings (vecteurs) pour chaque texte extrait à l'aide de BERT, puis de stocker ces embeddings dans une base de données MongoDB.

Le programme prend un argument en ligne de commande : le chemin du fichier PDF à traiter.

Connexion à MongoDB :
Le programme établit une connexion à une base de données MongoDB (en utilisant la fonction connect_to_mongo).

Extraction du texte du PDF :
Le texte et les labels sont extraits du fichier PDF à l'aide de la fonction extract_text_and_labels.

Génération des embeddings :
Le texte extrait est converti en embeddings (vecteurs numériques) via le modèle BERT.

Insertion dans MongoDB :
Les embeddings, associés à leurs labels et textes, sont insérés dans la collection MongoDB spécifiée.

**Prérequis**
Avant d'exécuter le projet, vous devez avoir les éléments suivants installés sur votre machine :

Rust : Le langage de programmation utilisé pour ce projet.

Pour installer Rust, vous pouvez suivre la documentation officielle de Rust.

MongoDB : Une instance MongoDB en fonctionnement. Vous pouvez utiliser Docker pour configurer MongoDB en local.

Docker : Si vous utilisez Docker pour MongoDB, suivez les étapes ci-dessous pour configurer MongoDB dans un conteneur Docker.

Pour installer les dépendances, exécutez la commande suivante dans le répertoire du projet :

```bash
cargo build
```
Cela installera toutes les dépendances nécessaires à l'exécution du programme.

Exécution du programme
**Étape 1** : Lancer MongoDB avec Docker
Si vous n'avez pas déjà MongoDB d'installé, vous pouvez démarrer un conteneur Docker avec MongoDB en utilisant la commande suivante :

```bash
docker run --name mongodb-container -d -p 27017:27017 mongo
```
Cela démarre un conteneur MongoDB avec le port 27017 exposé pour permettre la connexion à la base de données.

**Étape 2** : Exécuter le programme
Une fois MongoDB en place, vous pouvez exécuter le programme Rust en fournissant le chemin vers un fichier PDF à traiter comme argument. Par exemple :

```bash
cargo run -- /path/to/your/pdf-file.pdf
```
Le programme fera ce qui suit :

Lire le fichier PDF spécifié.
Extraire le texte et les labels à l'aide de la fonction extract_text_and_labels.
Générer des embeddings pour chaque texte extrait.
Insérer ces embeddings dans une base de données MongoDB (dans la collection embeddings de la base dataset).
Exemple de commande :
```bash
cargo run -- "/path/to/your/pdf-file.pdf"
```
Assurez-vous que le fichier PDF existe et que MongoDB est en cours d'exécution avant de lancer le programme.

Résultat attendu
Une fois l'exécution terminée, les embeddings générés seront stockés dans la base de données MongoDB, et un message confirmant l'insertion sera affiché dans la console.
Structure du projet
Voici la structure du projet :

text
Copier le code
├── src/
│   ├── main.rs          # Execution principal
│   ├── extract.rs      # Code pour l'extraction du texte et des labels
│   ├── manipulation_db.rs # Code pour la connexion et l'insertion dans MongoDB
|   |__ extract.rs      # Code d'extraction du texte et de labelisation
├── Cargo.toml          # Fichier de configuration des dépendances
└── README.md           # Ce fichier
