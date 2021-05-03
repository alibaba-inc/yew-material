
export const PUBLIC_URL = typeof process === "object" ? process.env.PUBLIC_URL : "";
export const InnerWidth = innerWidth;

const Root = document.getElementById("root");
const YewMdc_iframe_id = query_string("_id");

let iframe_height = 0;
let iframe_post_msg = height => {
	parent.postMessage({
		id: YewMdc_iframe_id,
		type: "YewMdc_root_height",
		value: height,
	}, "*");
};

let observer_timeout = {};
const observer = new MutationObserver(_mutation => {
	if (query_string("_height") === "no") {
		let height = Root.clientHeight;
		if (height && height !== iframe_height) {
			iframe_height = height;
			iframe_post_msg(height);
		}
		//针对web-components组件的临时解决方案
		clearTimeout(observer_timeout);
		observer_timeout = setTimeout(() => {
			if (Root.clientHeight !== height) {
				iframe_height = Root.clientHeight;
				iframe_post_msg(Root.clientHeight);
			}
		}, 50);
	}
});

observer.observe(Root, {
	childList: true,
	attributes: true,
	subtree: true,
	characterData: true
});

addEventListener('message', function (e) {
	let { id, type, value } = e.data;
	let root_style = Root.style;
	let iframe_box = document.getElementById(id);
	let iframe = iframe_box ? iframe_box.children[0] : undefined;
	let iframe_style = iframe ? iframe.style : {};

	if (type === "YewMdc_iframe_open") {
		iframe.contentWindow.postMessage({
			type: "YewMdc_iframe_fix",
			value: iframe_box.getBoundingClientRect(),
		}, '*');
	}

	if (type === "YewMdc_main_frame_fix") {
		iframe_style.position = "fixed";
		iframe_style["z-index"] = 9;
	}

	if (type === "YewMdc_iframe_close") {
		iframe.contentWindow.postMessage({
			type: "YewMdc_iframe_reset",
			value: "",
		}, '*');
	}

	if (type === "YewMdc_main_frame_reset") {
		iframe_style.position = "";
		iframe_style["z-index"] = "inherit";
	}

	if (type === "YewMdc_root_height") {
		iframe_box.style.height = value + "px";
	}

	if (type === "YewMdc_iframe_fix") {
		let { x, y, width, height } = value;
		root_style.position = "fixed";
		root_style.width = width + "px";
		root_style.height = height + "px";
		root_style.left = x + "px";
		root_style.top = y + "px";
		parent.postMessage({
			id: YewMdc_iframe_id,
			type: "YewMdc_main_frame_fix",
		}, "*");
	}

	if (type === "YewMdc_iframe_reset") {
		root_style.position = "";
		root_style.width = "auto";
		root_style.height = "auto";
		root_style.left = "";
		root_style.top = "";
		parent.postMessage({
			id: YewMdc_iframe_id,
			type: "YewMdc_main_frame_reset",
		}, "*");
	}

	if (type === "YewMdc_change_theme") {
		change_theme(value);
		dispatchEvent(new Event(id));
	}
}, false);

export const iframe_open = () => {
	parent.postMessage({
		id: YewMdc_iframe_id,
		type: "YewMdc_iframe_open",
	}, "*");
}

export const iframe_close = () => {
	parent.postMessage({
		id: YewMdc_iframe_id,
		type: "YewMdc_iframe_close",
	}, "*");
}

export const event = (msg) => {
	return new Event(msg);
}

let listener = {};
export const bind_listener = (uuid, type, callback) => {
	let ident = uuid + type;
	let reference = () => callback();
	listener[ident] = reference;
	addEventListener(type, reference, false);
}

export const remove_listener = (uuid, type) => {
	let ident = uuid + type;
	if (listener[ident]) {
		removeEventListener(type, listener[ident], false);
		delete listener[ident];
	}
}

let element_listener = {};
export const bind_element_listener = (uuid, element, type, callback) => {
	let ident = uuid + type;
	let reference = e => {
		e._detail = e.detail || {};
		e._action = e._detail.action || "";
		if (e._detail.index instanceof Set) {
			let index = [];
			e._detail.index.forEach(item => {
				index.push(item);
			});
			e._detail.index = index;
		}
		let target_ident = e.target.getAttribute("_id");
		let element_ident = element.getAttribute("_id");
		//解决同type冒泡问题
		if (target_ident && element_ident && target_ident !== element_ident) {
			return void 0;
		}
		//解决Radio组件在某些情况下切换失效问题
		let { name, nodeName } = e.target;
		if (nodeName === "MWC-RADIO") {
			document.getElementsByName(name).forEach(e => {
				if (target_ident !== e.getAttribute("_id") && e.checked) {
					e.checked = false;
				}
			});
		}
		return callback(element, e);
	};
	element_listener[ident] = reference;
	element.addEventListener(type, reference, false);
}

export const remove_element_listener = (uuid, element, type) => {
	let ident = uuid + type;
	if (element_listener[ident]) {
		element.removeEventListener(type, element_listener[ident], false);
		delete element_listener[ident];
	}
}

let hasClass = (elem, cls = "") => {
	if (cls.replace(/\s/g, "").length == 0) return false;
	return new RegExp(" " + cls + " ").test(" " + elem.className + " ");
};

let match = (element, selector) => {
	if (!element.getAttribute) return false;
	if (element.getAttribute("selectorIgnore") !== null) return false;
	let id = selector.split("#")[1];
	let cls = selector.split(".")[1];
	if (id && element.id && element.id === id) return true;
	if (cls && hasClass(element, cls)) return true;
	if (element.tagName == selector.toUpperCase()) return true;
	return false;
};

let delegate = (element, selector, callback) => {
	return (event) => {
		let parent = event.target;
		while (parent) {
			if (element == parent) return false;
			if (match(parent, selector)) {
				callback.call(element, parent, event);
				return false;
			}
			parent = parent.parentNode;
		}
	};
};

let closest = (element, selector) => {
	let parent = element.parentNode;
	while (parent) {
		if (parent.host) {
			parent = parent.host;
		}
		if (match(parent, selector)) {
			return parent;
		}
		parent = parent.parentNode;
	}
	return parent;
};

let find = (element, selector) => {
	let nodes = [];
	let children = element.children;
	let shadowRootChildren = element.shadowRoot ? element.shadowRoot.children : [];
	let whiles = (children) => {
		for (let i = 0; i < children.length; i++) {
			if (match(children[i], selector)) {
				nodes.push(children[i]);
			} else {
				children[i].children && whiles(children[i].children);
				children[i].shadowRoot && whiles(children[i].shadowRoot.children);
			}
		}
	};
	whiles(children);
	whiles(shadowRootChildren);
	return nodes;
};

export const find_element = (element, selector) => {
	return find(element, selector)[0];
}

export const set_element_attr = (element, attr, value) => {
	element.setAttribute(attr, value);
}

export const remove_children_attr = (element, selector, attr) => {
	let nodes = find(element, selector);
	for (let i in nodes) {
		nodes[i].removeAttribute(attr);
	}
}

let on_listener = {};

export const off = (uuid, element, type) => {
	let ident = uuid + type;
	if (!element || !on_listener[ident]) return false;
	if (element.length >= 0) {
		for (let i = 0; i < element.length; i++) {
			element[i].removeEventListener(type, on_listener[ident], false);
		}
	} else {
		element.removeEventListener(type, on_listener[ident], false);
	}
	delete on_listener[ident];
}

export const bind = (uuid, element, selector, type, callback) => {
	if (!element || !selector) return false;
	let ident = uuid + type;
	let handle = delegate(element, selector, callback);
	if (element.length >= 0) {
		for (let i = 0; i < element.length; i++) {
			element[i].addEventListener(type, handle, false);
		}
	} else {
		element.addEventListener(type, handle, false);
	}
	on_listener[ident] = handle;
}

export const anchor = (element, anchor_id) => {
	element.anchor = document.getElementById(anchor_id);
};

function query_string(name) {
	let reg = new RegExp("(^|&)" + name + "=([^&]*)(&|$)");
	let r = location.search.substr(1).match(reg);
	if (r != null) return unescape(r[2]);
	return "";
};

export const get_query_string = query_string;

const FormItemValidTrans = {};

export const formValidtrans = (id, callback) => {
	FormItemValidTrans[id] = callback;
}

export const del_form_validtrans = (id) => {
	delete FormItemValidTrans[id];
}

export const form_traversing = (id) => {
	let values = {};
	let valid = true;
	let form = document.forms[id];
	if (form) {
		let inputs = find(form, "input");
		let textarea = find(form, "textarea");
		let items = inputs.concat(textarea);
		for (let i = 0; i <= items.length - 1; i++) {
			let item = items[i];
			let field = closest(item, ".__YewMdc_input") || { getAttribute: () => "" };
			let name = item.name || field.getAttribute("name");
			let type = item.type || field.getAttribute("type");
			let value = item.value || field.value || "";
			if (name) {
				let item_val = values[name] || "";
				switch (type) {
					case "radio":
						//处于选中状态才进行存值
						item.checked && [item_val = value];
						break;
					case "checkbox":
						if (field.getAttribute("_type") === "switch") {
							//switch特殊处理
							item_val = item.checked ? value : field.getAttribute("off_value");
						} else {
							//强制checkbox值为数组类型
							!item_val && [item_val = []];
							item.checked && item_val.push(value);
						}
						break;
					default:
						//已有同name存值，且值类型不是数组，则将其值改为数组类型，并将原值插入
						if (item_val && !Array.isArray(item_val)) {
							let _item_val = item_val;
							item_val = [];
							item_val.push(_item_val);
						}
						//已有同name存值，且值类型已是数组，则将其值直接插入
						if (item_val && Array.isArray(item_val)) {
							item_val.push(value);
						}
						//如没有同name存值，则直接创建该存值
						!item_val && [item_val = value];
				}

				//将此次组件存值存入总值
				values[name] = item_val;

				if (field && field.checkValidity) {
					field.validityTransform = (val, native) => {
						//true为回调校验通过
						let transform = FormItemValidTrans[field.getAttribute("_id")] || function () { return true };
						return {
							valid: transform(val) && native.valid,
						};
					}
					valid && [valid = field.checkValidity()];
					!field.disabled && field.reportValidity();
				}
			}
		}
	}
	return {
		valid,
		values
	};
}
