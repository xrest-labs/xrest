import { Builder, WebDriver, Capabilities } from 'selenium-webdriver';
import { expect } from 'chai';
import { fileURLToPath } from 'url';
import path from 'path';
import { spawn, ChildProcess } from 'child_process';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const APP_PATH = path.resolve(__dirname, '../../src-tauri/target/debug/xrest');

describe('Application Load', function () {
    this.timeout(60000); // Tauri apps can take a while to load

    let driver: WebDriver;
    let tauriDriver: ChildProcess;

    before(async function () {
        // Start tauri-driver
        tauriDriver = spawn('tauri-driver', [], {
            stdio: 'inherit',
        });

        // Wait for tauri-driver to start
        await new Promise((resolve) => setTimeout(resolve, 2000));

        const capabilities = new Capabilities();
        capabilities.set('tauri:options', {
            application: APP_PATH,
        });
        capabilities.setBrowserName('wry');

        driver = await new Builder()
            .withCapabilities(capabilities)
            .usingServer('http://127.0.0.1:4444/')
            .build();
    });

    after(async function () {
        if (driver) {
            await driver.quit();
        }
        if (tauriDriver) {
            tauriDriver.kill();
        }
    });

    it('should load the application and show the main window', async function () {
        // Check if we can get the title or some element
        const title = await driver.getTitle();
        console.log('App title:', title);

        // In Tauri, the title might be what's in tauri.conf.json or the HTML title
        // Let's just check if it's not empty
        expect(title).to.not.be.empty;
    });

    it('should have the root element', async function () {
        const root = await driver.executeScript('return document.getElementById("app")');
        expect(root).to.not.be.null;
    });
});
