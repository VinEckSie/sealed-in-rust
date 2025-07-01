- All chapters are in the `src/` folder
- The book configuration is in `book.toml`

### ✏️ To edit:
1. Open `src/` and write your chapters in `.md` files
2. Use standard Markdown + `mdBook` features (e.g. `{{#include}}`, `{{#playground}}`, etc.)

## 🚀 Preview Locally
run:
mdbook serve

This starts a local dev server at http://localhost:3000
It watches for changes and reloads automatically.


☁️ Publishing (Manual)
Run mdbook build

Switch to the gh-pages branch

Copy contents of book/ into root

Commit and push:

bash
Copy
Edit
git add .
git commit -m "Publish book"
git push origin gh-pages
Tip: Set up GitHub Pages to serve from the gh-pages branch root.

🧠 Notes
Don’t commit the book/ folder to main — it’s in .gitignore
Keep your writing in src/, and version-control your Markdown + book.toml
You can automate deployment later using GitHub Actions

