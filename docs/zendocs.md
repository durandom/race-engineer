```md
# 🏎️ Race Engineer

Race Engineer is a desktop application built with Tauri + SvelteKit to visualize Richard Burns Rally (RBR) data from JSON and INI files in a user-friendly interface.

---

## ⚙️ Setup Instructions

### 1. Copy Environment File

```bash
mv <project>/app/src-tauri.env.example <project>/app/src-tauri/.env
```

---

### 2. Select RBR Data Directory

On first launch, you’ll be prompted to select your RBR installation directory.  
Use the included sample data for development:

```bash
<your-project-root>/data/rbr.sample/
```

The selected path will be stored automatically after selection.

---

## 🚀 Running the App

### 3. Launch Dev Server

```bash
cd app
pnpm tauri dev
```

---

## 🛠️ Build for Distribution

### 4. Compile for Your OS

```bash
pnpm run build
```

---

## 🧹 Code Quality

### 5. Format Code

```bash
pnpm run format
```

### 6. Lint Code

```bash
pnpm run lint
```

---

## 📁 Directory Structure

```txt
data/rbr.sample/      # Sample RBR data files (INI/JSON)
app/                  # Tauri + SvelteKit frontend
src-tauri/            # Rust backend logic & Tauri commands
```

---

## 📦 Requirements

- [Rust](https://www.rust-lang.org/tools/install)
- [pnpm](https://pnpm.io/installation)
- Node.js (LTS recommended)

---

## 🧑‍💻 Author

**[@shahidshabbir-se](https://github.com/shahidshabbir-se)**

---
