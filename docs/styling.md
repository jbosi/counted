# Styling

## Stack

- **TailwindCSS v4** — utility classes
- **DaisyUI v5** — component classes (`btn`, `modal`, `card`, `navbar`, etc.)
- **Theme**: `cupcake` (light pastel), set as default via `themes: cupcake --default`

## How it works

### React app (`frontend-react/counted/`)

Vite handles CSS via `@tailwindcss/vite`. The entry CSS is `src/App.css`:

```css
@import 'tailwindcss';
@plugin "daisyui";
```

Vite scans source files automatically — no manual rebuild needed.

### Dioxus apps (`packages/web/`, `packages/mobile/`)

`dx serve` has a **built-in Tailwind watcher**: when it finds a `tailwind.css` at the package root, it automatically downloads and runs the Tailwind CLI, writing output to `assets/tailwind.css`.

Each package has a `tailwind.css` input file:

```css
@import "tailwindcss";
@source "./src/**/*.rs";
@source "../ui/src/**/*.rs";
@plugin "daisyui" {
  themes: cupcake --default;
}
```

The `@source` directives are required — Tailwind v4 does not auto-detect `.rs` files, so without them the output only contains DaisyUI component styles and no utility classes.

The generated `assets/tailwind.css` is loaded via `document::Link` (goes to `<head>`):

```rust
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");
// ...
document::Link { rel: "stylesheet", href: TAILWIND_CSS }
```

`data-theme="cupcake"` is set on the root `<main>` element in both `packages/web/src/main.rs` and `packages/mobile/src/main.rs`. Because the input CSS uses `themes: cupcake --default`, the cupcake CSS variables are also applied to `:root` — the explicit `data-theme` attribute is belt-and-suspenders.

### UI package (`packages/ui/`)

Shared Dioxus components. All styling is done with inline Tailwind + DaisyUI class strings in RSX:

```rust
div { class: "card bg-base-100 shadow-sm cursor-pointer hover:shadow-md transition-shadow", ... }
```

Custom CSS assets live in `packages/ui/assets/styling/` (`navbar.css`, `hero.css`, `echo.css`) and are referenced via `asset!()` in the components that need them.

## Adding new classes

Just use them in `.rs` files — `dx serve` will pick them up via the `@source` glob and regenerate `assets/tailwind.css` automatically on the next hot-reload.

## Pitfall: stale `assets/tailwind.css`

`assets/tailwind.css` is a **generated file** and should not be edited by hand. If it looks wrong or missing classes, restart `dx serve` to force a full rebuild.
