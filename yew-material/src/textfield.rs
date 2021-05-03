use crate::{comp_theme_data, form::ValidityRes, imports, styles::Extra, to_option};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement, ShadowRoot};
use yew::prelude::*;
use yew_material_utils::{
    del_form_validtrans, find_element, form_validtrans, json,
    style::{create_component_style, Item},
    theme::{destroy, reload_with_data, to_theme, DataMsg, Theme},
    WebComponents,
};
use yewtil::NeqAssign;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_else(|| None)]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: String,
    #[prop_or_else(|| None)]
    pub slot: Option<String>,
    #[prop_or_else(|| None)]
    pub name: Option<String>,
    #[prop_or_else(|| None)]
    pub value: Option<String>,
    #[prop_or_else(|| "text".into())]
    pub r#type: String,
    #[prop_or_default]
    pub width: String,
    #[prop_or_else(|| None)]
    pub label: Option<String>,
    #[prop_or_else(|| None)]
    pub placeholder: Option<String>,
    #[prop_or_else(|| None)]
    pub prefix: Option<String>,
    #[prop_or_else(|| None)]
    pub suffix: Option<String>,
    #[prop_or_else(|| None)]
    pub icon: Option<String>,
    #[prop_or_else(|| None)]
    pub icon_trailing: Option<String>,
    #[prop_or_else(|| false)]
    pub disabled: bool,
    #[prop_or_else(|| false)]
    pub char_counter: bool,
    #[prop_or_else(|| false)]
    pub outlined: bool,
    #[prop_or_else(|| None)]
    pub helper: Option<String>,
    #[prop_or_else(|| false)]
    pub helper_persistent: bool,
    #[prop_or_else(|| false)]
    pub required: bool,
    #[prop_or_else(|| None)]
    pub max_length: Option<u32>,
    #[prop_or_else(|| None)]
    pub pattern: Option<String>,
    #[prop_or_else(|| None)]
    pub min: Option<String>,
    #[prop_or_else(|| None)]
    pub max: Option<String>,
    // #[prop_or_else(|| None)]
    // pub size: Option<u32>, !!!体验糟糕
    // #[prop_or_else(|| None)]
    // pub step: Option<i32>, !!!有校验BUG
    #[prop_or_else(|| None)]
    pub validate_message: Option<String>,
    #[prop_or_default]
    pub validate_trans: ValidityRes,
    #[prop_or_else(|| false)]
    pub validate_init_render: bool,
    #[prop_or_else(|| false)]
    pub auto_validate: bool,
    #[prop_or_else(|| "off".into())]
    pub auto_complete: String,
    #[prop_or_else(|| false)]
    pub focus: bool,
    #[prop_or_else(|| false)]
    pub auto_theme: bool,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub shadow: Option<ShadowRoot>,
}

pub type This = Textfield;

comp_theme_data!(
    Textfield,
    Data { shadow: None },
    |this: &This, first: bool| {
        if first {
            imports("textfield");
            let This {
                props,
                link,
                uuid,
                node,
                ..
            } = &this;
            let element = node.cast::<Element>().unwrap();

            if props.auto_theme {
                reload_with_data::<This>(&uuid, link);
            }

            element.set_attribute("type", &props.r#type).unwrap();

            let focus = props.focus;
            let complete = props.auto_complete.to_string();
            let this_link = link.to_owned();
            WebComponents::try_rendered(
                element,
                Box::new(move |shadow, element| {
                    let input = find_element(&element, ".mdc-text-field__input");
                    input.set_attribute("autocomplete", &complete).unwrap_or(());
                    if focus {
                        match input.dyn_ref::<HtmlElement>() {
                            Some(ele) => ele.focus().unwrap_or(()),
                            _ => (),
                        };
                    }
                    let mwc_rendered: DataMsg<This> =
                        DataMsg::Callback(Box::new(move |this: &mut This| {
                            this.data.shadow = Some(shadow.to_owned());
                            true
                        }));
                    this_link.send_message(mwc_rendered);
                }),
            );
        }
    },
    |uuid: &String, _| {
        del_form_validtrans(&uuid);
        destroy(&uuid)
    },
    |this: &This| {
        let This {
            props,
            uuid,
            data,
            node,
            ..
        } = &this;

        let validity = props.validate_trans.to_owned();
        form_validtrans(&uuid, Box::new(move |v| validity.emit(v)));

        let width = props.width.to_string();
        let style = create_component_style::<Extra>(
            Box::new(move |theme: &Theme<Extra>| {
                json!({
                    "jss": {
                        "--mdc-text-field-ink-color": theme.textfield.color,
                        "--mdc-text-field-fill-color": theme.textfield.background,
                        "--mdc-theme-error": theme.textfield.error_color,
                        "--mdc-theme-primary": theme.textfield.label_focus_color,
                        "--mdc-text-field-label-ink-color": theme.textfield.label_color,
                        "--mdc-text-field-disabled-ink-color": theme.textfield.disabled_color,
                        "--mdc-text-field-disabled-fill-color": theme.textfield.disabled_background,
                        "--mdc-shape-small": theme.textfield.radius,
                        "--mdc-text-field-outlined-idle-border-color": theme.textfield.border_color,
                        "--mdc-text-field-outlined-hover-border-color": theme.textfield.border_hover_color,
                        "--mdc-text-field-outlined-disabled-border-color": theme.textfield.border_disabled_color,
                    },
                    "width": {
                        "width": width,
                    }
                })
            }),
            "Textfield",
        );

        match &data.shadow {
            Some(shadow) => {
                let theme = to_theme::<Extra>();
                let style = format!(
                    r#"
                    .mdc-text-field:not(.mdc-text-field--disabled) .mdc-text-field__affix.mdc-text-field__affix--prefix,
                    .mdc-text-field:not(.mdc-text-field--disabled) .mdc-text-field__affix.mdc-text-field__affix--suffix{{
                        color:inherit;
                    }}
                    .mdc-text-field:not(.mdc-text-field--disabled) .mdc-text-field__icon{{
                        transition: color 0.2s;
                    }}
                    .mdc-text-field:not(.mdc-text-field--disabled) .mdc-text-field__icon.mdc-text-field__icon--leading,
                    .mdc-text-field:not(.mdc-text-field--disabled) .mdc-text-field__icon.mdc-text-field__icon--trailing{{
                        color:{};
                    }}"#,
                    theme.textfield.icon_color
                );
                WebComponents::set_style(&uuid, &shadow, style);
            }
            _ => (),
        };

        html! {
            <mwc-textfield
                //_id:validate_trans锚点
                _id=&uuid
                ref=node.to_owned()
                id?=props.id.as_ref()
                class=format!("__YewMdc_input {} {} {}", props.class, style.theme_item("jss"), style.theme_item("width"))
                slot?=props.slot.as_ref()
                name?=props.name.as_ref()
                value?=props.value.as_ref()
                label?=props.label.as_ref()
                placeholder?=props.placeholder.as_ref()
                prefix?=props.prefix.as_ref()
                suffix?=props.suffix.as_ref()
                icon?=props.icon.as_ref()
                iconTrailing?=props.icon_trailing.as_ref()
                disabled=props.disabled
                charCounter?=to_option(props.char_counter)
                outlined?=to_option(props.outlined)
                helper?=props.helper.as_ref()
                helperPersistent?=to_option(props.helper_persistent)
                required=props.required
                maxLength?=props.max_length.as_ref()
                pattern?=props.pattern.as_ref()
                min?=props.min.as_ref()
                max?=props.max.as_ref()
                // size?=props.size.as_ref()
                // step?=props.step.as_ref()
                validationMessage?=props.validate_message.as_ref()
                validateOnInitialRender?=to_option(props.validate_init_render)
                autoValidate?=to_option(props.auto_validate)
                dialogInitialFocus?=to_option(props.focus)
            />
        }
    },
    "./doc/textfield.md"
);
