use uuid::Uuid;
use yew_material_utils::{
    json,
    style::{create_component_style, Item},
    theme::Theme,
};

#[derive(Deserialize, Serialize)]
pub(crate) struct Extra {}

pub fn style(item: &'static str) -> String {
    create_component_style::<Extra>(
        Box::new(|theme: &Theme<Extra>| {
            json!({
                "scrollbar": {
                    "&::-webkit-scrollbar": {
                        "width": "8px",
                        "height": "8px",
                    },
                    "&::-webkit-scrollbar-thumb": {
                        "background": theme.scrollbar.background,
                    }
                },
                "transition_bg": {
                    "background": theme.skeleton.background,
                    "background-size": "400% 100%",
                    "animation-iteration-count": "infinite",
                },
            })
        }),
        "ComponentStyles",
    )
    .item(item)
}

pub(crate) fn animation<'a>(
    identity: String,
    r#type: &'a str,
    from: &'a str,
    to: &'a str,
    time: f32,
) -> String
where
    'a: 'static,
{
    let ident = match identity.as_str() {
        "" => Uuid::new_v4().to_string(),
        _ => identity,
    };
    let ident = format!("anime-{}", ident);
    let identity = ident.to_owned();
    create_component_style::<Extra>(
        Box::new(move |_| {
            json!({
                ident.to_owned(): {
                    "animationName": "$animate_frames",
                    "animation-duration": format!("{}s", time)
                },
                "@keyframes animate_frames": {
                    "from": {
                        r#type: from
                    },
                    "to": {
                        r#type: to
                    },
                }
            })
        }),
        "AnimateFrames",
    )
    .item(&identity)
}

pub(crate) fn transition_bg() -> String {
    style("transition_bg")
        + " "
        + &animation(
            "skeleton".into(),
            "background-position",
            "100% 50%",
            "0 50%",
            1.5,
        )
}
