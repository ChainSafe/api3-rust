const NodeEnvironment = require('jest-environment-node');
const nearAPI = require('near-api-js');
const fs = require('fs');

const { PROJECT_KEY_DIR } = require('near-cli/middleware/key-store');

const INITIAL_BALANCE = '500000000000000000000000000';
const testAccountName = 'mochacha.testnet';

class LocalTestEnvironment extends NodeEnvironment {
    constructor(config) {
        super(config);
    }

    async setup() {
        this.global.nearlib = require('near-api-js');
        this.global.nearAPI = require('near-api-js');
        this.global.window = {};
        let config = require('near-cli/get-config')();
        this.global.testSettings = this.global.nearConfig = config;
        const now = Date.now();
        // create random number with at least 7 digits
        const randomNumber = Math.floor(Math.random() * (9999999 - 1000000) + 1000000);
        config = Object.assign(config, {
            contractName: 'test-account-' + now + '-' + randomNumber,
            accountId: 'test-account-' + now + '-' + randomNumber
        });
        const keyStore = new nearAPI.keyStores.UnencryptedFileSystemKeyStore(PROJECT_KEY_DIR);
        config.deps = Object.assign(config.deps || {}, {
            storage:  this.createFakeStorage(),
            keyStore,
        });
        const near = await nearAPI.connect(config);

        await super.setup();
    }

    async teardown() {
        await super.teardown();
    }

    runScript(script) {
        return super.runScript(script);
    }

    createFakeStorage() {
        let store = {};
        return {
            getItem: function(key) {
                return store[key];
            },
            setItem: function(key, value) {
                store[key] = value.toString();
            },
            clear: function() {
                store = {};
            },
            removeItem: function(key) {
                delete store[key];
            }
        };
    }
}

module.exports = LocalTestEnvironment;
