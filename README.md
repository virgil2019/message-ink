# message_ink

<img width="750" alt="web3 foundation_grants_badge_white" src="https://user-images.githubusercontent.com/83746881/187579523-a40f4fe5-178c-4dae-9482-aa737ff6226b.png">

This is the message defination lib for the protocol stack in ink!.

## Usage

* Put `Payload = { git = "https://github.com/dantenetwork/message_ink", branch = "main" }` in your `Cargo.toml` of your contract project. Or clone it from "https://github.com/dantenetwork/message_ink" and substitute with your related local path.

## Main API
The interaction message between ink! smart contract and others(deployed on other chains) are expressed as [MessagePayload](https://github.com/dantenetwork/message-ink/blob/f7bd6571356c1eb419c67b50cb11ae9506cd351c/payload/message_protocol.rs#L515), which is composited with a vector of [MessageItem](https://github.com/dantenetwork/message-ink/blob/f7bd6571356c1eb419c67b50cb11ae9506cd351c/payload/message_protocol.rs#L497). The core part of `MessageItem` is [MsgDetail](https://github.com/dantenetwork/message-ink/blob/f7bd6571356c1eb419c67b50cb11ae9506cd351c/payload/message_protocol.rs#L26), which defines almost every useful types. 

A convenient way to Build a `MsgDetail` is as follows:  
* The trait [InMsgType](https://github.com/dantenetwork/message-ink/blob/f7bd6571356c1eb419c67b50cb11ae9506cd351c/payload/message_protocol.rs#L99) is defined to create an instance of `MsgDetail` from a normal type and decode out a normal type from `MsgDetail`.
```rust
pub trait InMsgType {
    type MyType;
    fn get_value(type_value: & MsgDetail) -> Option<Self::MyType>;
    fn create_message(msg_detail: Self::MyType) -> MsgDetail;
}
```
* As enum variants are not `types`, we provide an implementation of every type defined in `MsgDetail` separately. 

In addition, with the help of `InMsgType` for `MsgDetail`, `MessageItem` can be directly decoded into a normal type with [x.in_to()](https://github.com/dantenetwork/message-ink/blob/f7bd6571356c1eb419c67b50cb11ae9506cd351c/payload/message_protocol.rs#L505), in which x is an instance of `MessageItem`.

That's the main interface of the message protocol for ink! smart contracts, that is very easy to use.
