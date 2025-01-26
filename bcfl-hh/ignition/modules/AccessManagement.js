const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");

module.exports = buildModule("AccessManagement", (m) => {
    const accessManager = m.contract("AccessManagement", []);
  

    return { accessManager };
  });