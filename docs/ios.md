# iOS Build Guide

## One-time Setup

### 1. Xcode
Install from the Mac App Store. Open it once to accept the license and install components.

### 2. Rust iOS Target

```bash
rustup target add aarch64-apple-ios
```

### 3. Dioxus CLI

```bash
cargo install dioxus-cli --version "0.7.2" --locked
```

---

## Development (Simulator)

```bash
npm run ios
# = dx serve --platform ios --package mobile
```

Runs a live-reload build targeting the iOS Simulator. The app calls server functions against `https://counted.fr`.

---

## Build Release IPA

### 1. Generate the Xcode project

```bash
npm run ios:build
# = dx bundle --platform ios --package mobile --release
```

This produces an Xcode project under `target/dx/mobile/`.

### 2. Package as IPA

`dx bundle` produces a pre-built `.app` directly — no Xcode project is generated.

```bash
mkdir -p Payload
cp -r target/dx/mobile/release/ios/Mobile.app Payload/Counted.app
zip -r Counted.ipa Payload/
```

---

## Manual Release to AltStore

After building the IPA, upload it to the GitHub Release for the current version tag and update the source:

```bash
VERSION=$(git describe --tags --abbrev=0)

# Upload IPA to the existing release
gh release upload "$VERSION" Counted.ipa --clobber

# Update altstore-source.json
IPA_SIZE=$(wc -c < Counted.ipa | tr -d ' ')
IPA_URL="https://github.com/jbosi/Counted/releases/download/${VERSION}/Counted.ipa"
DATE=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
VER="${VERSION#v}"

python3 - <<EOF
import json, os

with open("altstore-source.json") as f:
    source = json.load(f)

source["apps"][0]["versions"].insert(0, {
    "version": "$VER",
    "buildVersion": "1",
    "date": "$DATE",
    "downloadURL": "$IPA_URL",
    "size": $IPA_SIZE,
    "localizedDescription": "New release"
})

with open("altstore-source.json", "w") as f:
    json.dump(source, f, indent=2)
EOF

git add altstore-source.json
git commit -m "chore: update altstore-source.json for ${VERSION}"
git push
```

---

## AltStore Source URL

Share this URL with users to add the source in AltStore (**Settings → Sources → +**):

```
https://raw.githubusercontent.com/jbosi/Counted/main/altstore-source.json
```

---

## Notes

- **Unsigned IPA**: AltStore Classic re-signs with the user's Apple ID on install — no Apple Developer account needed to distribute.
- **CI automation**: The `ios-build.yml` workflow does all of the above automatically on every push to `main`. Use this manual flow only when you need an out-of-band release.
- **localStorage on iOS**: The app's localStorage state (project/user associations) uses browser APIs that are no-ops on native iOS. This is a known limitation — data won't persist between app launches until a native storage layer is added.
