Smart Contract
==================

init contracts

```shell
near deploy nl_abc_a.testnet res/nl_abc.wasm --account_id=nl_abc_a.testnet
near deploy nl_abc_b.testnet res/nl_abc.wasm --account_id=nl_abc_b.testnet
near deploy nl_abc_c.testnet res/nl_abc.wasm --account_id=nl_abc_c.testnet

near call nl_abc_a.testnet new '{"contract_next": "nl_abc_b.testnet", "status": 1}' --account_id=nl_abc_a.testnet

near call nl_abc_b.testnet new '{"contract_next": "nl_abc_c.testnet", "status": 2}' --account_id=nl_abc_b.testnet

near call nl_abc_c.testnet new '{"contract_next": "", "status": 3}' --account_id=nl_abc_c.testnet

```

show status of each contracts
```shell
near view nl_abc_a.testnet get_status ''
near view nl_abc_b.testnet get_status ''
near view nl_abc_c.testnet get_status ''
```

reset status of each contracts
```shell
near call nl_abc_a.testnet set_status '{"status": 1}' --account_id=humeng.testnet
near call nl_abc_b.testnet set_status '{"status": 2}' --account_id=humeng.testnet
near call nl_abc_c.testnet set_status '{"status": 3}' --account_id=humeng.testnet
```

```shell
near call nl_abc_a.testnet call_has_promise '' --gas 60000000000000 --amount=20 --account_id=humeng.testnet
```

# panic at the third contract
---------
caller: humeng.testnet  
formattedAmount: '484.592298222830042580637283'  
called: nl_abc_a/b/c  
formattedAmount: '204.9990073731614979'  
formattedAmount: '204.9985917401582891'  
formattedAmount: '209.99874930775771809999999'  


```shell
near call nl_abc_a.testnet call_has_promise '' --gas 60000000000000 --amount=12 --account_id=humeng.testnet

near view nl_abc_a.testnet get_status
near view nl_abc_b.testnet get_status
near view nl_abc_c.testnet get_status
```
