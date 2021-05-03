export const init = () => {
	let Jss = jss.default;
	Jss.setup(jssPresetDefault.default());
	window["Jss"] = Jss;
	window["Imports"] = () => { };
}
