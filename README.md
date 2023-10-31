<p align="center">
  <img src="assets/hero.png">
  <h1 align="center">🗽 NY</h1>
  <h3 align="center">Proxy Package Manager for JavaScript</h3>
  <h3 align="center">Chooses the right package manager based on the lockfile.</h3>
  <p align="center"><i><strong>n</strong></i>ode • <i><strong>y</strong></i>arn • pnpm</p>
</p>

## Features

- <strong>Universal</strong> - Picks the right package manager for you based on the lockfile in your folder. Easy peasy!
- <strong>Versatile</strong> - Handles the basics like installing all your dependencies, adding new packages, and kicking off scripts.
- <strong>Speedy</strong> - Crafted in Rust to give you a quick ride. No extra node processes to slow you down!
- <strong>TypeScript-Ready</strong> - Adding a new dependency? Don't worry! It'll fetch any missing `@types` packages for you if needed.

## Usage

```sh
ny # installs all dependencies eq. to: yarn install
```

```sh
ny add zod # installs zod package eq. to: yarn add zod
```

```sh
ny add react # installs react package and attempts to automatically install missing typings (@types/react)
```

```sh
ny test # executes package.json's test script eq. to: yarn test
```

```sh
ny vitest # executes node_modules/.bin binary eq. to: yarn vitest
```

## Installation

## Tea

[Tea](http://tea.xyz/) is a Homebrew successor and recommended way of installing NY.

```sh
tea ny
```

## Brew

```sh
brew install krzkaczor/tap/ny
```

### Download binary for Linux / Mac Os X

Get the newest release from [releases page](https://github.com/krzkaczor/ny/releases).

## Dive deeper

### Motivation

In the world of JavaScript, there's a fun mix of package managers - npm, yarn, pnpm. Pick your poison. Programmers often switch between projects that use different PMs multiple times a day. Did you just typed `yarn` when the project uses `npm`? Well too bad -- you've wasted couple of seconds again. With 🗽NY, there's no guesswork. Just type `ny` and it picks the right manager for you. Handy, right?

And here's the kicker: 🗽NY is snappier! Especially when running package scripts (like when you type `yarn test`). It's written in Rust and it zips through tasks about ~200ms faster by skipping spawning node process just to parse package.json. Sweet, huh?

Main sources of inspiration were [antfu/ni](https://github.com/antfu/ni) (but it's written in JS) and [egoist/dum](https://github.com/egoist/dum) (but it's only a task runner).

### TypeScript support

If `ny` detects that it's running in TypeScript enabled package, it will attempt to install missing typings when adding new packages. Right now this behaviour can't be turned off.

### Contributing

All contributions are welcomed! [Read contributing guide for more](./contributing.md).

### Logo

Logo should depict a statue of liberty (a symbol of <strong>N</strong>ew <strong>Y</strong>ork ;) ) holding tools. Midjourney prompt goes something like: `simple mascot, statue of liberty with tools, pixelart style`. Later [vectorizer.ai](https://vectorizer.ai) was used to get SVG out of bitmap.

I know it's not perfect and if you figure out a better prompt let me know!

## License

[Kris Kaczor](https://twitter.com/krzkaczor) | MIT
