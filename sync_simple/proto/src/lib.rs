#![feature(plugin, use_extern_macros)]
#![plugin(tarpc_plugins)]

#[macro_use]
extern crate tarpc;

service! {
    rpc hello(name: String) -> String;
}