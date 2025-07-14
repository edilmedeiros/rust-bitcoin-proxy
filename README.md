# rust-bitcoin-proxy

A proxy for the Bitcoin RPC interface.

# How to Use

There are two executables in this project, `roxy-cli` and `roxyd`. The former is the equivalent of 
`bitcoin-cli` and the latter the counterpart for `bitcoind`.

## Testing

This subcommand pings a `bitcoind` node with a `getblockchaininfo` command only using `roxyd`. 
To do a simple test you can in one terminal:
```
$ just bitcoind -daemon
```
That will start a bitcoind node in the background.
In another terminal, you run:
```
$ just roxyd test
```
The expected result is to have the following in the `stdout`:
```
Pinged to Bitcoin Core!
```

## Proxy

This subcommand pings a a `bitcoind` node with `getblockchaininfo` command using both `roxyd` and `roxy-cli`.
```
$ just bitcoind -daemon
```
That will start a bitcoind node in the background.
In another terminal, you run:
```
$ just roxyd proxy
```
In another terminal, you run the following to trigger a response from the Bitcoin node:
```
$ just rcli getblockchaininfo
```
An example of one the possible results for this command is to have the following in the `stdout`:
```
Response { url: "http://localhost:8080/proxy", status: 200, headers: {"content-length": "523", "date": "Mon, 14 Jul 2025 23:25:58 GMT"} }
{"jsonrpc":"2.0","result":{"chain":"regtest","blocks":0,"headers":0,"bestblockhash":"0f9188f13cb7b2c71f2a335e3a4fc328bf5beb436012afca590b1a11466e2206","bits":"207fffff","target":"7fffff0000000000000000000000000000000000000000000000000000000000","difficulty":4.656542373906925e-10,"time":1296688602,"mediantime":1296688602,"verificationprogress":1,"initialblockdownload":true,"chainwork":"0000000000000000000000000000000000000000000000000000000000000002","size_on_disk":293,"pruned":false,"warnings":[]},"error":null,"id":2}
```
