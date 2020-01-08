# `latexbuild`

## Features

- Smart rebuild:
  - Only rebuild when included files change
  - Create additional build when the `.aux` file changes
- `bin` folder: all generated contents are put into a `bin` folder
- Bootstrap new projects

## Commands

```
latexbuild
```

Create a build

```
latexbuild new <name>
```

Create a new project

```
latexbuild clean
```

Remove the `bin` folder
