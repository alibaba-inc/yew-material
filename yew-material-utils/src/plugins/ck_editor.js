export const init_editor = (selector, config, ready) => {
	ClassicEditor.create(document.querySelector(selector), config).then(
		(editor) => {
			ready(editor);
			Array.from(document.getElementsByClassName("ck-editor__main")).forEach(
				(node) => {
					node.style.color = "#000";
				}
			);
		}
	);
}

export const set_data = (editor, text) => {
	editor.setData(text);
}

export const get_data = (editor) => {
	return editor.getData();
}
