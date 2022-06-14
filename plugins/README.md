# odilia-plugins

Odilia's plugin architecture.

Part of the [Odilia screen reader project](https://odilia.app).

## Design

This crate implements the plugin architecture for the Odilia screen reader.

Plugins are processes spawned by Odilia that communicate with the main screen reader process over their stdin / stdout
using [json-rpc][json-rpc]. They can be written in any language, but we have plans to provide easy to use bindings for
common languages.

[json-rpc]: <https://www.jsonrpc.org/>

## Contributing

This is a very young project, we appreciate any and all contributions! However, please be aware there is a very llarge
learning curve to helping with this project, particularly due to the lack of documentation, or **complete**
documentation, of many of the libraries and technologies that comprise the Linux accessibility stack. For this reason,
we are currently focused on learning as much as we can, and writing code to take advantage of it, and we don't have lots
of time to mentor new contributors or review pull requests.

Once the ground-work has been layed, accepting contributions should get much easier. We are greatful for your
cooperation in this regard!

## License

All our code is licensed under the [GPL v3](https://www.gnu.org/licenses/gpl-3.0.html).
