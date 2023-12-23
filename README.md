# Studies

## Runnable Projects 

- `yarn nx serve ts-oop`
- `yarn nx serve _rust_oop`

## Monorepo Setup

- npx create-nx-workspace@latest studies --preset=ts
- sets `"packageManager": "yarn@3.6.1"` in package.json and adds a .yarn with it specific release
- run `yarn`
- mkdir apps && mkdir libs

## Generate code for NodeJS runtime

Plugins needed: `yarn add @nx/node -D`

### Apps

- `yarn nx g @nx/node:application ts/oop`

### Libs

- `yarn nx g @nx/js:lib ts/http --bundler=swc`

## Generate code for rust

Plugins needed: `yarn add @monodon/rust -D`

### Apps

- `yarn nx g @monodon/rust:binary oop --directory=/rust`

### Libs

- `yarn nx g @monodon/rust:library cats --directory=/rust`
## Remove grouped folder structure

> Tip: It doesn't matter if it's in apps or libs, you can find the project name in the `project.json` files.

- `yarn nx g rm ts-oop-e2e` which were in `/apps/ts/oop-e2e`. Same for projects inside /libs.

