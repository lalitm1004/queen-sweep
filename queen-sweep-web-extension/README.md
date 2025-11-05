# queen-sweep-web-extension

Chromium extension to inteface with the [QueensGame website](https://queensgame.vercel.app) and inject a solution for the puzzle on screen

## Components

### `content-script`
Handles:
- Detecting when a supported puzzle is present on the page
- Extracting the board data and converting it into a `number[][]` format
- Sending a solve request to the service worker
- Receiving the solution and injecting a UI button that performs the required click sequence on demand


### `service-worker`
Handles:
- Lazy-loading and initializing the WASM solver
- Executing solve requests and returning the result to the content script
- Keeping the solver alive across page reloads when possible

## Installing chromium extension
1. Build the WASM module
```bash
cd queen-sweep-web-extension
npm run wasm:build
```

2. Build the extension
```bash
npm i
npm run build
```

3. Load in Chrome
   - Navigate to `brave://extensions/` **or** your extension manager
   - Enable `Developer mode`
   - Click `Load unpacked`
   - Select the `queen-sweep-web-extension/dist` directory