# Repository instructions

## Project media

- Do not use image-generation tools for this project. Avoid ImageGen and other high-token image generation workflows.
- When a task needs Granblue Fantasy or “星之古战场” visuals, prefer an existing relevant material or a real screenshot, subject to its usage terms.
- README and release screenshots must come from the running application, not generated mockups.
- Keep third-party game materials minimal, preserve attribution where appropriate, and do not imply that this project is official or endorsed by Cygames.
- Optimize image assets for their actual display size before committing them.

## Verification

- Run `npm run check` and `npm run build` after frontend changes.
- Run `cargo test --manifest-path src-tauri/Cargo.toml` after Rust changes.
- For release or packaging changes, verify at least one native Tauri bundle locally when the current host supports it.
