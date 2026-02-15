---
title: Downloads
outline: false
---

# Downloads

Choose the package that matches your Linux distribution. The list below is automatically populated from the latest CI build.

<DownloadList />

## Install notes

### Debian / Ubuntu (.deb)

```bash
sudo dpkg -i pmon_*.deb
```

### Fedora / RHEL (.rpm)

```bash
sudo rpm -i pmon-*.rpm
```

### Standalone binary

```bash
chmod +x pmon
./pmon
```

## Bonus: private apt repository (optional)

If you host your own updates, you can generate an apt repository from the built `.deb` packages.

1. Run `scripts/build-apt-repo.sh` on a machine with `dpkg-scanpackages` installed.
2. Publish the resulting directory on any static web host.
3. Add it to your systems:

```bash
echo "deb [trusted=yes] https://nknapp.gitlab.io/pmon/apt ./" | sudo tee /etc/apt/sources.list.d/pmon.list
sudo apt update
sudo apt install pmon
```
