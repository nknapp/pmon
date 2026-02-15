---
title: Sample config
---

Use this as a starting point for `config.yaml`.

```yaml
providers:
  - type: github
    token:
      # Load GITHUB_TOKEN from environment variable
      env: GITHUB_TOKEN
    repos:
      # Show only the main branch of my frontend-testing repository.
      - name: nknapp/frontend-testing
        main_branch: main
        workflow: playwright.yml
```
