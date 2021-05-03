let styleSheets = {};

const styleMatch = (Pathname, json) => {
	if (!styleSheets[Pathname]) {
		styleSheets[Pathname] = [];
		return {
			status: false,
		};
	}

	if (typeof json !== "string") {
		json = JSON.stringify(json);
	}

	let item = styleSheets[Pathname].filter((item) => item.json === json)[0];

	if (item && item.classes) {
		return {
			status: true,
			classes: item.classes,
		};
	} else {
		return {
			status: false,
		};
	}
};

export const create_style_sheet = (style, meta, json, global) => {
	let Pathname = global ? "_global" : location.pathname.replace(/\//g, "_");

	let { status, classes } = styleMatch(Pathname, style);

	if (status) {
		return classes;
	}

	let styleSheetsItem = {};

	styleSheetsItem.json = JSON.stringify(style);

	if (json) {
		// try {
		// 	styleSheetsItem.json = style;
		// 	style = JSON.parse(style);
		// } catch {
		// 	console.error(
		// 		"json format error, please check if there is a string type value without quotes.\nError content:",
		// 		style
		// 	);
		// }
	} else {
		// styleSheetsItem.json = JSON.stringify(style);
		for (let items in style) {
			for (let item in style[items]) {
				if (item.indexOf("_") > -1) {
					let _item = item.replace(/_/g, "-");
					style[items][_item] = style[items][item];
					delete style[items][item];
				}
			}
		}
	}

	let sheet = Jss.createStyleSheet(style, { meta }).attach();
	let class_key = [];
	let class_val = [];
	for (let i in sheet.classes) {
		if (i) {
			class_key.push(i);
			class_val.push(sheet.classes[i]);
		}
	}
	let _classes = class_key.toString() + "|" + class_val.toString();
	styleSheetsItem.classes = _classes;
	styleSheets[Pathname].push(styleSheetsItem);
	return _classes;
};
