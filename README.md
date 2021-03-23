# scarab

**Tools for working with IOTA Smart Contracts**

---

## Usage

```rust
fn init(ctx: &ScFuncContext) {
  let owner: ScAgentId = ctx.get_required_param("ownerParam");
  let token: ScHash = ctx.get_required_param("tokenParam");

  // ...
}

fn my_func(ctx: &ScFuncContext) {
  let data: ScHash = ctx.get_required_param("data");
  let time: i64 = ctx.timestamp();

  ctx.state().set("$time", &time);
  ctx.state().set("$data", &data);

  ctx.result("data", data);
  ctx.result("time", time);

  // ...
}

fn my_view(ctx: &ScViewContext) {
  let data: ScHash = ctx.state().get_value("$data");

  ctx.result("data", data);

  // ...
}

#[no_mangle]
fn on_load() {
  let exports = ScExports::new();
  exports.add_func("init", init);
  exports.add_func("myFunc", my_func);
  exports.add_view("myView", my_view);
}
```

## References

* [Wasp](https://github.com/iotaledger/wasp)
* [WasmLib](https://github.com/iotaledger/wasp/tree/master/contracts/rust/wasmlib)

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
</sub>
