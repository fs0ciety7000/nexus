# Guide : Push du Projet vers GitHub

## 🎯 Objectif
Pousser le projet Nexus Dashboard vers votre repository GitHub : `https://github.com/fs0ciety7000/nexus.git`

## 📋 Méthode Automatique (Recommandée)

### Étape 1 : Naviguer vers le projet
```bash
cd /path/to/nexus-dashboard
```

### Étape 2 : Rendre le script exécutable
```bash
chmod +x scripts/setup-git.sh
```

### Étape 3 : Exécuter le script
```bash
./scripts/setup-git.sh
```

Le script va automatiquement :
- Initialiser le repository Git
- Créer un `.gitignore` approprié
- Ajouter le remote GitHub
- Faire le commit initial
- Pusher vers GitHub

### ⚠️ Authentification GitHub

Vous serez invité à vous authentifier. Deux options :

**Option 1 : Personal Access Token (Recommandé)**
1. Allez sur https://github.com/settings/tokens
2. Cliquez "Generate new token (classic)"
3. Cochez : `repo` (tous les scopes)
4. Générez et copiez le token
5. Utilisez-le comme mot de passe lors du push

**Option 2 : SSH (Plus sécurisé)**
```bash
# Générer une clé SSH
ssh-keygen -t ed25519 -C "your-email@example.com"

# Ajouter à l'agent SSH
eval "$(ssh-agent -s)"
ssh-add ~/.ssh/id_ed25519

# Copier la clé publique
cat ~/.ssh/id_ed25519.pub

# Aller sur GitHub > Settings > SSH keys > Add new
# Coller la clé publique

# Changer le remote pour SSH
git remote set-url origin git@github.com:fs0ciety7000/nexus.git
```

---

## 📋 Méthode Manuelle

Si vous préférez faire les étapes manuellement :

### 1. Initialiser Git
```bash
cd nexus-dashboard
git init
```

### 2. Configurer Git (si nécessaire)
```bash
git config user.name "fs0ciety7000"
git config user.email "votre-email@example.com"
```

### 3. Créer .gitignore
```bash
cat > .gitignore << 'EOF'
# Rust
target/
Cargo.lock
**/*.rs.bk
*.pdb

# Node
node_modules/
dist/
dist-ssr/
*.local

# Environment
.env
.env.local

# IDE
.vscode/
.idea/
*.swp
*.swo
*~

# OS
.DS_Store
Thumbs.db

# Tauri
src-tauri/target/
src-tauri/WixTools/

# Build outputs
*.exe
*.dmg
*.AppImage
*.deb

# Logs
*.log
npm-debug.log*
yarn-debug.log*
EOF
```

### 4. Ajouter le remote GitHub
```bash
git remote add origin https://github.com/fs0ciety7000/nexus.git
```

### 5. Stager tous les fichiers
```bash
git add .
```

### 6. Créer le commit initial
```bash
git commit -m "🚀 Initial commit: Nexus Dashboard v0.1.0

Production-grade desktop command center for Bazzite Linux

Features:
- System Sentinel: Real-time CPU/GPU/RAM monitoring
- Obsidian Viewer: Markdown note browser
- Proton Reactor: GE-Proton version scanner

Tech Stack:
- Backend: Rust + Tauri v2
- Frontend: React 19 + TypeScript + TailwindCSS v4
"
```

### 7. Pusher vers GitHub
```bash
git branch -M main
git push -u origin main
```

---

## 🔧 Résolution des Problèmes

### Erreur : "Repository already exists"
Si le repository GitHub existe déjà avec du contenu :

```bash
# Option 1 : Forcer le push (⚠️ écrase le contenu distant)
git push -u origin main --force

# Option 2 : Pull puis push (recommandé si contenu important)
git pull origin main --allow-unrelated-histories
git push -u origin main
```

### Erreur : "Authentication failed"
```bash
# Vérifier le remote
git remote -v

# Re-configurer avec token
git remote set-url origin https://YOUR_TOKEN@github.com/fs0ciety7000/nexus.git
```

### Erreur : "Permission denied (publickey)"
Vous devez configurer SSH (voir section SSH ci-dessus)

---

## 📝 Configuration Recommandée du Repository GitHub

Après le push, configurez votre repository :

### 1. Description
```
High-performance desktop command center for Bazzite Linux - Real-time system monitoring, Obsidian integration, and Proton management
```

### 2. Topics (tags)
```
rust
tauri
react
typescript
tailwindcss
desktop-app
bazzite
linux-gaming
system-monitor
obsidian
proton
```

### 3. Social Preview
Ajoutez une capture d'écran de l'application comme image social preview

### 4. README Badges
Le README inclut déjà les badges pour :
- Technologies utilisées
- Status du projet
- License

### 5. GitHub Pages (Optionnel)
Pour héberger la documentation :
```bash
Settings > Pages > Source: Deploy from branch > Branch: main / docs
```

---

## 🚀 Commandes Git Utiles Post-Push

### Vérifier le status
```bash
git status
```

### Créer une nouvelle branche pour développement
```bash
git checkout -b develop
git push -u origin develop
```

### Créer un tag pour une release
```bash
git tag -a v0.1.0 -m "Initial release"
git push origin v0.1.0
```

### Mettre à jour après modifications
```bash
git add .
git commit -m "feat: add new feature"
git push
```

---

## 📦 Structure des Commits (Convention)

Utilisez les préfixes suivants pour vos commits :

- `feat:` Nouvelle fonctionnalité
- `fix:` Correction de bug
- `docs:` Documentation
- `style:` Formatage, style
- `refactor:` Refactoring
- `perf:` Amélioration de performance
- `test:` Ajout de tests
- `chore:` Maintenance, configuration

Exemple :
```bash
git commit -m "feat(system): add disk usage monitoring"
git commit -m "fix(obsidian): handle empty vault gracefully"
git commit -m "docs: update installation instructions"
```

---

## ✅ Checklist Finale

Après le push, vérifiez :

- [ ] Le code est sur GitHub
- [ ] Le README s'affiche correctement
- [ ] Les fichiers sensibles ne sont pas présents (tokens, .env)
- [ ] Le .gitignore fonctionne (pas de node_modules, target/)
- [ ] La description du repo est ajoutée
- [ ] Les topics sont configurés
- [ ] La license est visible (MIT)

---

## 🆘 Support

En cas de problème :

1. Vérifiez les logs : `git log --oneline`
2. Vérifiez les remotes : `git remote -v`
3. Consultez le status : `git status`

Pour toute question, créez une issue sur le repository!

---

**Bon push ! 🚀**
