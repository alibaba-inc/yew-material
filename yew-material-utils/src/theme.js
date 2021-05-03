const font_weight = 400;
const font_family =
	"-apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, 'Noto Sans'";
const appbar_shadow = "0px 2px 4px -1px rgba(0, 0, 0, 0.2), 0px 4px 5px 0px rgba(0, 0, 0, 0.14), 0px 1px 10px 0px rgba(0, 0, 0, 0.12)";
const skeleton_background = "linear-gradient(90deg, hsla(0, 0%, 75%, .2) 25%, hsla(0, 0%, 50%, .25) 37%, hsla(0, 0%, 75%, .2) 63%)";
const linear_buffer_dots = color => `url(\"data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' x='0px' y='0px' xml:space='preserve' viewBox='0 0 5 2' preserveAspectRatio='none slice'%3E%3Ccircle cx='1' cy='1' r='1' fill='${escape(color)}'/%3E%3C/svg%3E\")`;

const color = (type, num) => {
	if (type === "dark") {
		return `rgba(0, 0, 0, .${num})`;
	} else {
		return `rgba(255, 255, 255, .${num})`;
	}
};

//各组件缺省样式值默认由各主题下一级同名样式补值
let schemes = {
	light: {
		font_family,
		font_weight,
		color: color("dark", "88"),
		background: "#fff",
		ripple: color("dark", "28"),
		checked: "#018786",
		unchecked: color("dark", "54"),
		activated: "#6200ee",
		selected: color("dark", "12"),
		disabled: color("dark", "18"),
		hover: color("dark", "06"),
		icon_color: color("dark", "62"),
		divider: {
			color: color("dark", "12"),
		},
		skeleton: {
			background: skeleton_background,
		},
		progress: {
			color: "#6200ee",
			linear_buffer: "#e6e6e6",
			linear_buffer_dots: "#ffcdd2",
		},
		scrollbar: {
			background: color("dark", "21"),
		},
		button: {
			color: "#fff",
			text_button_color: "#6200ee",
			background: "#6200ee",
			ripple: "#fff",
			disabled_color: color("dark", "38"),
			disabled_background: color("dark", "12"),
		},
		icon: {
			color: color("dark", "62"),
		},
		appbar: {
			shadow: appbar_shadow,
			background: "#6200ee",
		},
		switch: {
			unchecked: "#7d7d7d",
			disabled: "#545454",
		},
		slider: {
			color: "#00af5b",
			background: "#b6b6b6",
		},
		dialog: {
			content_color: color("dark", "70"),
		},
		list: {
			ripple: "#666",
		},
		list_item: {
			secondary_color: color("dark", "54"),
		},
		textfield: {
			background: "#f5f5f5",
			error_color: "#b00020",
			label_color: color("dark", "60"),
			label_focus_color: "#6200ee",
			disabled_color: color("dark", "38"),
			disabled_background: "#fafafa",
			radius: "4px",
			border_color: color("dark", "38"),
			border_hover_color: color("dark", "87"),
			border_disabled_color: color("dark", "06"),
		},
	},
	dark: {
		font_family,
		font_weight,
		color: color("light", "92"),
		background: "#303030",
		ripple: color("light", "18"),
		checked: "#009906",
		unchecked: color("light", "54"),
		activated: "#a9a3ff",
		selected: color("light", "12"),
		disabled: color("light", "18"),
		hover: color("light", "06"),
		icon_color: "#73920c",
		// body: {
		// 	background: "#303030",
		// },
		divider: {
			color: color("light", "12"),
		},
		skeleton: {
			background: skeleton_background,
		},
		progress: {
			color: "#468a00",
			linear_buffer: "#9d9d9d",
			linear_buffer_dots: "#999",
		},
		scrollbar: {
			background: color("light", "31"),
		},
		button: {
			color: "#fff",
			text_button_color: "#ac9aff",
			background: "#6200ee",
			ripple: "#fff",
			disabled_color: color("light", "28"),
			disabled_background: color("light", "12"),
		},
		appbar: {
			shadow: appbar_shadow,
			background: "#212121",
		},
		switch: {
			unchecked: "#b9b9b9",
			disabled: "#b3b3b3",
		},
		slider: {
			color: "#5da913",
			background: "#414141",
		},
		dialog: {
			content_color: color("light", "70"),
		},
		list: {
			ripple: "#fff",
		},
		list_item: {
			secondary_color: color("light", "54"),
		},
		tab_item: {
			color: color("light", "72"),
		},
		textfield: {
			background: "#292929",
			error_color: "#ff002e",
			label_color: color("light", "60"),
			label_focus_color: "#8b3aff",
			disabled_color: color("light", "12"),
			disabled_background: "#575757",
			radius: "4px",
			border_color: color("light", "38"),
			border_hover_color: color("light", "87"),
			border_disabled_color: color("light", "06"),
		},
	},
};

let theme_ident_pre_val = "";
let bodyStyle = document.body.style;
const setBodyStyle = (ident, is_iframe) => {
	if (theme_ident_pre_val !== ident) {
		theme_ident_pre_val = ident;
		let scheme = schemes[ident];
		bodyStyle.color =
			scheme.body && scheme.body.color ? scheme.body.color : scheme.color;
		!is_iframe && [bodyStyle.background =
			scheme.body && scheme.body.background
				? scheme.body.background
				: scheme.background];
		bodyStyle["font-family"] = scheme.font_family;
		bodyStyle["font-weight"] = scheme.font_weight;
	}
};


export const change_theme_to_window = (cb) => {
	window.change_theme = cb;
}

export const change_theme_to_iframe = (ident, event) => {
	Array.from(document.getElementsByTagName("iframe")).forEach(element => {
		element.contentWindow.postMessage({
			id: event,
			type: "YewMdc_change_theme",
			value: ident,
		}, '*');
	});
}

export const theme_ident_pre = () => {
	return theme_ident_pre_val;
}

export const set_theme = (_ident = "light", theme) => {
	let defs = { ...schemes };

	//对 theme 进行light、dark两种主题数据补齐
	for (let ident in schemes) {
		if (!theme[ident]) {
			theme[ident] = schemes[ident];
		}
		for (let items in schemes[ident]) {
			if (typeof theme[ident][items] === "undefined") {
				theme[ident][items] = schemes[ident][items];
			}
			if (typeof theme[ident][items] === "object") {
				for (let item in theme[ident][items]) {
					if (typeof theme[ident][items][item] === "undefined") {
						theme[ident][items][item] = schemes[ident][items][item];
					}
				}
			}
		}
	}

	//对 theme 进行自定义数据补齐
	for (let ident in theme) {
		let def = defs[ident];
		if (ident !== "light" && ident !== "dark") {
			def = defs.dark;
		}
		for (let items in def) {
			if (typeof theme[ident][items] === "undefined") {
				theme[ident][items] = def[items];
			}
			if (typeof theme[ident][items] === "object") {
				for (let item in def[items]) {
					if (typeof theme[ident][items][item] === "undefined") {
						theme[ident][items][item] = def[items][item];
					}
				}
			}
		}
	}

	schemes = theme;
}

const themes = {};
export const theme = (_ident, is_iframe) => {
	let ident = _ident || "light";
	let scheme = schemes[ident];

	let fit = (items, item) =>
		scheme[items] && scheme[items][item] ? scheme[items][item] : scheme[item];

	setBodyStyle(ident, is_iframe);

	if (!themes[ident]) {
		themes[ident] = {
			ident,
			font_family: scheme.font_family,
			font_weight: scheme.font_weight,
			color: scheme.color,
			background: scheme.background,
			ripple: scheme.ripple,
			activated: scheme.activated,
			selected: scheme.selected,
			hover: scheme.hover,
			disabled: scheme.disabled,
			body: {
				color: fit("body", "color"),
				background: fit("body", "background"),
			},
			divider: {
				color: fit("divider", "color"),
			},
			skeleton: {
				background: fit("skeleton", "background"),
			},
			progress: {
				color: fit("progress", "color"),
				linear_buffer: fit("progress", "linear_buffer"),
				linear_buffer_dots: linear_buffer_dots(fit("progress", "linear_buffer_dots")),
			},
			scrollbar: {
				background: fit("scrollbar", "background"),
			},
			button: {
				color: fit("button", "color"),
				text_button_color: fit("button", "text_button_color"),
				background: fit("button", "background"),
				ripple: fit("button", "ripple"),
				disabled_color: fit("button", "disabled_color"),
				disabled_background: fit("button", "disabled_background"),
			},
			appbar: {
				shadow: fit("appbar", "shadow"),
				background: fit("appbar", "background"),
			},
			icon: {
				color: fit("icon", "color"),
			},
			radio: {
				checked: fit("radio", "checked"),
				unchecked: fit("radio", "unchecked"),
				disabled: fit("radio", "disabled"),
			},
			checkbox: {
				checked: fit("checkbox", "checked"),
				unchecked: fit("checkbox", "unchecked"),
				disabled: fit("checkbox", "disabled"),
			},
			switch: {
				checked: fit("switch", "checked"),
				unchecked: fit("switch", "unchecked"),
				disabled: fit("switch", "disabled"),
			},
			slider: {
				color: fit("slider", "color"),
				background: fit("slider", "background"),
			},
			list: {
				background: fit("list", "background"),
				ripple: fit("list", "ripple"),
				selected: fit("list", "selected"),
				hover: fit("list", "hover"),
			},
			list_item: {
				secondary_color: fit("list_item", "secondary_color"),
			},
			tab_item: {
				color: fit("tab_item", "color"),
				icon_color: fit("tab_item", "icon_color"),
				activated: fit("tab_item", "activated"),
				ripple: fit("tab_item", "ripple"),
			},
			dialog: {
				heading_color: fit("dialog", "color"),
				content_color: fit("dialog", "content_color"),
				background: fit("dialog", "background"),
			},
			textfield: {
				color: fit("textfield", "color"),
				background: fit("textfield", "background"),
				error_color: fit("textfield", "error_color"),
				label_color: fit("textfield", "label_color"),
				label_focus_color: fit("textfield", "label_focus_color"),
				icon_color: fit("textfield", "icon_color"),
				disabled_color: fit("textfield", "disabled_color"),
				disabled_background: fit("textfield", "disabled_background"),
				radius: fit("textfield", "radius"),
				border_color: fit("textfield", "border_color"),
				border_hover_color: fit("textfield", "border_hover_color"),
				border_disabled_color: fit("textfield", "border_disabled_color"),
			},
			snackbar: {
				color: fit("snackbar", "color"),
				background: fit("snackbar", "background"),
			},
			extra: scheme.extra || {},
		};
	}

	return themes[ident];
}