require("@nomicfoundation/hardhat-toolbox");

/** @type import('hardhat/config').HardhatUserConfig */
module.exports = {
  solidity: "0.8.20",

  defaultNetwork: "clique",
  networks: {
    clique: {
      url: "http://127.0.0.1:8545",
      gasPrice: 0,
      blockGasLimit: "0x1ffffffffffffe",
      allowUnlimitedContractSize: true,
      accounts: [
        "0x9a9aba58c323191579980ceb2886165a7ace005f1ef38fa0d622864cf1733725"
      ]
    },

    hardhat: {
      allowUnlimitedContractSize: true,
    }
    
  }
};
