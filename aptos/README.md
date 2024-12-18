# Aptos (WIP)

The endpoint folder was generated with `aptos move init --name Endpoint`.

This module leverages Move 2.0 features and the `aptos` CLI requires the `--move-2` flag as recently as version `4.2.3`.

## Status

- [x] `register`
- [x] `sendMessage`
- [x] `getMessageStatus`
- [x] `recvMessage`
- [x] `execMessage`
- [x] `attestMessage`
- [x] `pickUpMessage`
- [x] `updateAdmin`
- [x] `transferAdmin`
- [x] `claimAdmin`
- [x] `discardAdmin`
- [x] `addAdapter`
- [x] `enableSendAdapter`
- [x] `disableSendAdapter`
- [x] `enableRecvAdapter`
- [x] `disableRecvAdapter`
- [ ] Events
- [x] CI builds and enforces 100% coverage
- [x] Unit test for compute_message_hash matches known output
- [ ] Example adapter
- [ ] Example integrator

## Design

### Integrators

On-chain contracts which integrate with the Endpoint will need to store a resource account capability in order to pass a resource account signer to the `register` and `send_message` functions of the Endpoint. This allows the Endpoint to assign a sequence tracker and integrator configuration directly to that signer, restrict use to that signer, and improve off-chain visibility. See [Resource Accounts](https://aptos.dev/en/build/smart-contracts/resource-accounts) for more info.

### Endpoint

As of this writing, Aptos does not generally support dynamic dispatch (though they do support very specific dynamic dispatch through their [Fungible Asset](https://aptos.dev/en/build/smart-contracts/fungible-asset#dispatchable-fungible-asset-advanced)). This means that the Endpoint cannot call arbitrary Adapters on behalf of the integrator. It therefore must store intermediate message state and rely on a pull model for Adapters. Similar to inbound attestations, the outbound messages will be stored in a Table on the Endpoint.

This limitation means that the most effective way for an integrator to generically post a message and have the Adapters pick it up in a single transaction will be to have a front-end / SDK use an [Aptos Script](https://aptos.dev/en/build/smart-contracts/scripts). In the future, it may be officially supported to use a [Dynamic Transaction Composer](https://github.com/aptos-foundation/AIPs/blob/main/aips/aip-102.md) to achieve this.

### Adapters

Adapters must be associated with exactly one Endpoint instance and must expose a method to pick up a message from the Endpoint by its `source_address` and `sequence`, passing in a signer which uniquely identifies the Adapter (like its resource account signer). This signer must be the same known address used by Integrators when adding the Adapter.

## Development

Style note: this code generally, intentionally avoids the [dot (receiver) function call style](https://aptos.dev/en/build/smart-contracts/book/functions#dot-receiver-function-call-style) as it obscures the mutability of the reference used.

[Move IDE Plugins](https://aptos.dev/en/build/smart-contracts#move-ide-plugins)

### Compile

```bash
aptos move compile --move-2 --named-addresses endpoint=default
```

### Test

```bash
aptos move test --move-2 --named-addresses endpoint=default
```

For coverage, add the `--coverage` flag.

```bash
aptos move test --move-2 --coverage --named-addresses endpoint=default
```

And to view coverage or a module _after_ testing. e.g. for `endpoint::integrator`

```bash
aptos move coverage source --module integrator --move-2 --named-addresses endpoint=default
```
