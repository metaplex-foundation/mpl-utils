// @ts-check
'use strict';
const path = require('path');

const localDeployDir = path.join(__dirname, 'test-programs');
const { LOCALHOST, tmpLedgerDir } = require('@metaplex-foundation/amman');

function localDeployPath(programName) {
  return path.join(localDeployDir, `${programName}.so`);
}

// Only keeping core configuration without specific programs
const programs = {};

const validator = {
  killRunningValidators: true,
  programs,
  commitment: 'singleGossip',
  resetLedger: true,
  verifyFees: false,
  jsonRpcUrl: LOCALHOST,
  websocketUrl: '',
  ledgerDir: tmpLedgerDir(),
};

module.exports = {
  programs,
  validator,
  relay: {
    enabled: true,
    killRunningRelay: true,
  },
};
