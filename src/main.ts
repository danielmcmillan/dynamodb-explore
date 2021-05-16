const { app, ipcMain, BrowserWindow } = require("electron");

// Handle creating/removing shortcuts on Windows when installing/uninstalling.
if (require("electron-squirrel-startup")) {
  // eslint-disable-line global-require
  app.quit();
}

const createWindow = () => {
  // @ts-ignore
  const mainEntry: string = MAIN_WINDOW_WEBPACK_ENTRY;
  // @ts-ignore
  const preloadEntry: string = MAIN_WINDOW_PRELOAD_WEBPACK_ENTRY;
  // Create the browser window.
  const mainWindow = new BrowserWindow({
    width: 800,
    height: 600,
    webPreferences: {
      preload: preloadEntry,
      nodeIntegration: false,
      contextIsolation: true,
    },
  });

  mainWindow.loadURL(mainEntry);

  // Open the DevTools.
  mainWindow.webContents.openDevTools();
};

app.on("ready", createWindow);

app.on("window-all-closed", () => {
  if (process.platform !== "darwin") {
    app.quit();
  }
});

app.on("activate", () => {
  if (BrowserWindow.getAllWindows().length === 0) {
    createWindow();
  }
});
