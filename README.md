# bip174.org

## About

[bip174.org](https://bip174.org) is a *Partially Signed Bitcoin Transaction* (aka PSBT) editor and explorer that runs in the browser. It's designed to help developers who work with PSBTs and might need to
either just see what's inside a PSBT or make changes to it.

This repository contains the source code of the website: the web app is built in Rust using the [yew](https://yew.rs) framework.

### Build

When building for the first time, ensure to install dependencies first:

```
yarn install
```

And then build in release mode with optimizations enabled:

```
yarn run build
```

### Develop locally

To work locally you can compile the project in debug mode with a *watch* that automatically reloads the page after the code is changed. This command will do everything for you:

```
yarn run dev
```

