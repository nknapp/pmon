---
layout: home
title: pmon - your personal monitor
hero:
  name: pmon
  text: your personal monitor
  tagline: Watch every pipeline with calm, trustworthy signals. pmon keeps your workflows visible without the noise.
  image:
    src: /pmon-icon.svg
    alt: pmon icon
  actions:
    - theme: brand
      text: Download for Linux
      link: /downloads
    - theme: alt
      text: View config sample
      link: /sample-config
features:
  - title: Safe-by-default monitoring
    details: Smart polling and rate-limit awareness keep your providers healthy while you stay informed.
  - title: Clear, calming status views
    details: A focused dashboard highlights what needs attention without overwhelming your team.
  - title: Lightweight and private
    details: Runs locally, stores state on your device, and connects only to your CI providers.
---

## Keep every workflow in view

pmon is a focused desktop monitor for GitHub Actions and other providers. It delivers reliable signals, trusted status colors, and the confidence that your releases are on track.

::: warning Warning!
This project is in a very early development stage. Most of the code has been vibe-coded and still needs to be reviewed.
The tray icon works (i.e. shows the status of the latest pipeline and the one before that).
The UI is still from the template however.
I need to clean up the code a lot (I think) or even read it.

Use at your own risk.
:::

### Built for teams that value safety

- Stable polling with backoff and rate-limit awareness
- Runs locally and keeps your config close to you
- Clear separation between provider plugins and core state

### What you get

- A clean status grid with the latest runs
- Support for branches and pull requests
- Notifications when pipelines fail or recover
