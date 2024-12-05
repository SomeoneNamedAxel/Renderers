# Utiliser une image officielle de Rust comme base
FROM rust:latest

# Créer et définir le répertoire de travail dans le conteneur
WORKDIR /usr/src/raycast_candidate_1

# Copier le fichier Cargo.toml et le fichier Cargo.lock (si présent)
# pour que Docker puisse résoudre les dépendances avant de copier tout le code
COPY Cargo.toml Cargo.lock ./

# Télécharger les dépendances de votre application sans copier tout le code
# Cela permet à Docker de ne pas re-télécharger les dépendances si elles n'ont pas changé
RUN cargo build --release

# Copier le reste de votre code source dans le conteneur
COPY . .

# Construire l'application (cargo build) et exécuter le projet
CMD ["cargo", "run"]
