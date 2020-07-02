# `latexbuild`

This package has been discontinued in favor of [latextools](https://github.com/Zehua-Chen/latextools)

## Features

- Smart rebuild:
  - Only rebuild when included files change
  - Create additional build when the `.aux` file changes
- `bin` folder: all generated contents are put into a `bin` folder
- Bootstrap new projects

## Commands

- Build if needed

  ```
  latexbuild
  ```

- Create a new project

  ```
  latexbuild new <name>
  ```

- Clean existing build

  ```
  latexbuild clean
  ```
