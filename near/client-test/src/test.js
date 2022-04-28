require "test-setup";

describe('Token', function () {
  let near;
  let contract;
  let accountId;

  beforeAll(async function () {
    near = await nearlib.connect(nearConfig);
    accountId = nearConfig.contractName;
    contract = await near.loadContract(nearConfig.contractName, {
      viewMethods: ['contract_info'],
      changeMethods: ['contract_info'],
      sender: accountId
    });
  });

  describe('whitelist', function () {
    it('view contract info', async function () {
      const info = await contract.contract_info();
    });
  });
});
