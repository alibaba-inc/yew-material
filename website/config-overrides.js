const scripts = require("yew-material-scripts");
const plugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = function override(config, _env) {
	return scripts.initialize(plugin, config, "production");
};
