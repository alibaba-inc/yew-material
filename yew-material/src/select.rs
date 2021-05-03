use crate::{
    comp_theme_data,
    form::ValidityRes,
    imports,
    list::index::{Detail, SingleSelectedRes},
    styles::Extra,
    to_option,
};
use web_sys::{Element, HtmlInputElement, ShadowRoot};
use yew::prelude::*;
use yew_material_utils::{
    add_element_listener, del_form_validtrans, form_validtrans, json,
    style::{create_component_style, Item},
    theme::{destroy, reload_with_data, to_theme, DataMsg, Theme},
    WebComponents,
};
use yewtil::NeqAssign;

pub type SelectedRes = (i32, String);

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct Props {
    pub children: Children,
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
    #[prop_or_default]
    pub width: String,
    #[prop_or_else(|| false)]
    pub auto_list_width: bool,
    #[prop_or_else(|| false)]
    pub fixed_list: bool,
    // #[prop_or_else(|| None)]
    // pub label: Option<String>,
    #[prop_or_else(|| None)]
    pub icon: Option<String>,
    #[prop_or_else(|| false)]
    pub disabled: bool,
    #[prop_or_else(|| false)]
    pub outlined: bool,
    #[prop_or_else(|| None)]
    pub helper: Option<String>,
    #[prop_or_else(|| false)]
    pub required: bool,
    #[prop_or_else(|| None)]
    pub validate_message: Option<String>,
    #[prop_or_default]
    pub validate_trans: ValidityRes,
    #[prop_or_else(|| false)]
    pub validate_init_render: bool,
    #[prop_or_else(|| false)]
    pub auto_theme: bool,
    #[prop_or_default]
    pub onselected: Callback<SelectedRes>,
}

#[derive(Debug, Clone)]
pub struct Data {
    pub shadow: Option<ShadowRoot>,
}

pub type This = Select;

comp_theme_data!(
    Select,
    Data { shadow: None },
    |this: &This, first: bool| {
        if first {
            imports("select");
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

            let onselected = props.onselected.to_owned();
            let input = node.cast::<HtmlInputElement>().unwrap();
            add_element_listener(
                &uuid,
                &element,
                "selected",
                Box::new(move |_, e: Event| {
                    let res = e.into_serde::<Detail<SingleSelectedRes>>().unwrap();
                    onselected.emit((res._detail.index, input.value()));
                }),
            );

            //features = ["DomRect"]
            // if props.fixed_list {
            //     let auto_list_width = props.auto_list_width;
            //     add_element_listener(
            //         &uuid,
            //         &element,
            //         "opened",
            //         Box::new(move |ele: Element, _| {
            //             let rect = ele.get_bounding_client_rect();
            //             let menu = find_element(&ele, ".mdc-menu-surface--open");
            //             set_property(&menu, "left", &format!("{}px", rect.x()));
            //             set_property(&menu, "top", &format!("{}px", rect.y() + 56.0));
            //             set_property(
            //                 &menu,
            //                 "width",
            //                 &(match auto_list_width {
            //                     true => "auto".into(),
            //                     _ => format!("{}px", rect.width()),
            //                 }),
            //             );
            //             set_property(&menu, "position", "fixed");
            //         }),
            //     );
            // }

            let this_link = link.to_owned();
            WebComponents::try_rendered(
                element,
                Box::new(move |shadow, _| {
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
                        "--mdc-select-ink-color": theme.textfield.color,
                        "--mdc-select-dropdown-icon-color": theme.textfield.color,
                        "--mdc-select-disabled-dropdown-icon-color": theme.textfield.disabled_color,
                        "--mdc-select-fill-color": theme.textfield.background,
                        "--mdc-theme-error": theme.textfield.error_color,
                        "--mdc-theme-primary": theme.textfield.label_focus_color,
                        "--mdc-select-label-ink-color": theme.textfield.label_color,
                        "--mdc-select-disabled-ink-color": theme.textfield.disabled_color,
                        "--mdc-select-disabled-fill-color": theme.textfield.disabled_background,
                        "--mdc-shape-small": theme.textfield.radius,
                        "--mdc-select-outlined-idle-border-color": theme.textfield.border_color,
                        "--mdc-select-outlined-hover-border-color": theme.textfield.border_hover_color,
                        "--mdc-select-outlined-disabled-border-color": theme.textfield.border_disabled_color,
                    },
                    "width": {
                        "width": width,
                    }
                })
            }),
            "Select",
        );

        match &data.shadow {
            Some(shadow) => {
                let theme = to_theme::<Extra>();
                let style = format!(
                    r#"
                    mwc-menu {{
                        --mdc-theme-surface: {};
                    }}
                    .mdc-select:not(.mdc-select--disabled) .mdc-select__anchor .mdc-select__icon{{
                        transition: color 0.2s;
                        color:{};
                    }}
                    .mdc-select.mdc-select--invalid:not(.mdc-select--disabled) .mdc-select__anchor .mdc-select__icon,
                    .mdc-select.mdc-select--invalid:not(.mdc-select--disabled) + .mdc-select-helper-text{{
                        color:{}!important;
                    }}
                    .mdc-select.mdc-select--disabled .mdc-select__anchor .mdc-select__icon{{
                        color:{};
                    }}
                    "#,
                    theme.list.background,
                    theme.textfield.icon_color,
                    theme.textfield.error_color,
                    theme.textfield.disabled_color
                );
                WebComponents::set_style(&uuid, &shadow, style);
            }
            _ => (),
        };

        html! {
            <mwc-select
                _id=&uuid
                ref=node.to_owned()
                id?=props.id.as_ref()
                class=format!("__YewMdc_input {} {} {}", props.class, style.theme_item("jss"), style.theme_item("width"))
                slot?=props.slot.as_ref()
                name?=props.name.as_ref()
                value?=props.value.as_ref()
                // label?=props.label.as_ref()
                naturalMenuWidth?=to_option(props.auto_list_width)
                fixedMenuPosition?=to_option(props.fixed_list)
                icon?=props.icon.as_ref()
                disabled=props.disabled
                outlined?=to_option(props.outlined)
                helper?=props.helper.as_ref()
                required=props.required
                validationMessage?=props.validate_message.as_ref()
                validateOnInitialRender?=to_option(props.validate_init_render)
            >
                {props.children.to_owned()}
            </mwc-select>
        }
    },
    "./doc/select.md"
);
