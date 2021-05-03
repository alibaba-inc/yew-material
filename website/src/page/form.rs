use yew::prelude::*;
use yew_material::form::ValidityRes;
use yew_material::prelude::*;

use crate::route::*;
use crate::styles::*;

#[derive(Debug, Clone, Deserialize, Serialize, FormRes)]
pub struct Data {
    name: String,
    sex: String,
    job: String,
    adult: String,
    like: Vec<String>,
    about: String,
    ext: String,
}

pub enum Msg {
    Submit(Data),
    SetSex(&'static str),
}

pub struct PgForm {
    link: ComponentLink<Self>,
    result: Data,
}

impl Component for PgForm {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            result: Data {
                name: "".into(),
                sex: "".into(),
                job: "".into(),
                adult: "off".into(),
                like: vec![],
                about: "".into(),
                ext: "".into(),
            },
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(res) => {
                self.result = res;
                true
            }
            Msg::SetSex(sex) => {
                self.result.sex = sex.into();
                false
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        let link = self.link.to_owned();
        let sex = self.result.sex.as_str();
        html! {
            <Animate r#type="opacity" time=1.2>
                <div class=style("title")>{"Form 表单"}</div>
                <div>{"用于提取用户录入的数据"}</div>
                {static_des(0)}
                <ul class=style("describe")>
                    <li>{"表单校验：带有表单数据校验功能"}</li>
                </ul>
                {static_des(1)}
                {link_des("#")}
                {code("
//Cargo.toml features配置
default-features = false
features = [
    \"form\",
]

//组件引入
use yew_material::prelude::*;
//或者
use yew_material::form::Form;
                ")}
                {static_des(2)}
                <Flex class=style("exhibition") border_top=true border_bottom=true>
                    <Form<Data>
                        onsubmit=Callback::from(move |res: Data| {
                            link.send_message(Msg::Submit(res));
                        })
                    >
                        <Flex max_width="480px" padding="30px 25px" direction="column" border=true>
                            <Textfield
                                name="name"
                                label="请输入姓名"
                                pattern="^[\\u4e00-\\u9fa5]+$"
                                validate_message="只能输入中文哦~"
                            />
                            <Flex align="center" padding="0 0 19px 0">
                                <Text>{"性别："}</Text>
                                <FormField label="男" margin="0 15px 0 0">
                                    <Radio
                                        name="sex"
                                        value="man"
                                        checked=if sex == "man" {true} else {false}
                                        onchange=&self.link.callback(|sex| Msg::SetSex(if sex {"man"} else {"woman"}))
                                    />
                                </FormField>
                                <FormField label="女">
                                    <Radio
                                        name="sex"
                                        value="woman"
                                        checked=if sex == "woman" {true} else {false}
                                        onchange=&self.link.callback(|sex| Msg::SetSex(if sex {"woman"} else {"man"}))
                                    />
                                </FormField>
                            </Flex>
                            <Select
                                name="job"
                                validate_message="注意：隐士暂时不能选"
                                validate_trans=ValidityRes::from(|v: String| {
                                    match v == "隐士" {
                                        true => false,
                                        _ => true
                                    }
                                })
                            >
                                <ListItem>{"请选择职业"}</ListItem>
                                <ListItem value="道士">{"道士"}</ListItem>
                                <ListItem value="和尚">{"和尚"}</ListItem>
                                <ListItem value="隐士">{"隐士"}</ListItem>
                            </Select>
                            <Flex align="center" padding="9px 0 19px 0">
                                <Text padding="0 13px 0 0">{"成年："}</Text>
                                <Switch name="adult" />
                            </Flex>
                            <Flex align="center" padding="0 0 19px 0">
                                <Text>{"爱好："}</Text>
                                <FormField label="狗" margin="0 15px 0 0">
                                    <Checkbox name="like" value="dog" />
                                </FormField>
                                <FormField label="猫">
                                    <Checkbox name="like" value="cat" />
                                </FormField>
                            </Flex>
                            <Textarea
                                name="about"
                                label="请输入您的简介"
                                validate_message="内容不能含有违规字样哦~"
                                validate_trans=ValidityRes::from(|v: String| {
                                    match v.contains("违规") {
                                        true => false,
                                        _ => true
                                    }
                                })
                            />
                            <Flex align="center" margin="5px 0">
                                <Text>{"原生："}</Text>
                                <input name="ext" />
                            </Flex>
                            <Flex justify="center" border_top=true border_bottom=true margin="15px 0" padding="15px 0">
                                <Button r#type="submit" label="Submit Form" />
                            </Flex>
                            <Text auto_wrap=true>{self.result.to_json()}</Text>
                        </Flex>
                    </Form<Data>>
                </Flex>
                <div class="env_mobile_hide">
                    {code("
use yew_material_macro::FormRes;
use yew_material::form::ValidityRes;

#[derive(Debug, Clone, Deserialize, Serialize, FormRes)]
pub struct Data {
    name: String,
    sex: String,
    job: String,
    adult: String,
    like: Vec<String>,
    about: String,
    ext: String,
}

pub enum Msg {
    Submit(Data),
    SetSex(&'static str),
}

pub struct Page {
    link: ComponentLink<Self>,
    result: Data,
}

impl Component for Page {
    ...
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            result: Data {
                name: \"\".into(),
                sex: \"\".into(),
                job: \"\".into(),
                adult: \"off\".into(),
                like: vec![],
                about: \"\".into(),
                ext: \"\".into(),
            },
        }
    }
    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Submit(res) => {
                self.result = res;
                true
            }
            Msg::SetSex(sex) => {
                self.result.sex = sex.into();
                false
            }
        }
    }
    ...
    fn view(&self) -> Html {
        let link = self.link.to_owned();
        let sex = self.result.sex.as_str();
        html! {
            <Form<Data>
                onsubmit=Callback::from(move |res: Data| {
                    link.send_message(Msg::Submit(res));
                })
            >
                <Flex max_width=\"480px\" padding=\"30px 25px\" direction=\"column\" border=true>
                    <Textfield
                        name=\"name\"
                        label=\"请输入姓名\"
                        pattern=\"^[\\\\u4e00-\\\\u9fa5]+$\"
                        validate_message=\"只能输入中文哦~\"
                    />
                    <Flex align=\"center\" padding=\"0 0 19px 0\">
                        <Text>{\"性别：\"}</Text>
                        <FormField label=\"男\" margin=\"0 15px 0 0\">
                            <Radio
                                name=\"sex\"
                                value=\"man\"
                                checked=if sex == \"man\" {true} else {false}
                                onchange=&self.link.callback(|sex| Msg::SetSex(if sex {\"man\"} else {\"woman\"}))
                            />
                        </FormField>
                        <FormField label=\"女\">
                            <Radio
                                name=\"sex\"
                                value=\"woman\"
                                checked=if sex == \"woman\" {true} else {false}
                                onchange=&self.link.callback(|sex| Msg::SetSex(if sex {\"woman\"} else {\"man\"}))
                            />
                        </FormField>
                    </Flex>
                    <Select
                        name=\"job\"
                        validate_message=\"注意：隐士暂时不能选\"
                        validate_trans=ValidityRes::from(|v: String| {
                            match v == \"隐士\" {
                                true => false,
                                _ => true
                            }
                        })
                    >
                        <ListItem>{\"请选择职业\"}</ListItem>
                        <ListItem value=\"道士\">{\"道士\"}</ListItem>
                        <ListItem value=\"和尚\">{\"和尚\"}</ListItem>
                        <ListItem value=\"隐士\">{\"隐士\"}</ListItem>
                    </Select>
                    <Flex align=\"center\" padding=\"9px 0 19px 0\">
                        <Text padding=\"0 13px 0 0\">{\"成年：\"}</Text>
                        <Switch name=\"adult\" />
                    </Flex>
                    <Flex align=\"center\" padding=\"0 0 19px 0\">
                        <Text>{\"爱好：\"}</Text>
                        <FormField label=\"狗\" margin=\"0 15px 0 0\">
                            <Checkbox name=\"like\" value=\"dog\" />
                        </FormField>
                        <FormField label=\"猫\">
                            <Checkbox name=\"like\" value=\"cat\" />
                        </FormField>
                    </Flex>
                    <Textarea
                        name=\"about\"
                        label=\"请输入您的简介\"
                        validate_message=\"内容不能含有违规字样哦~\"
                        validate_trans=ValidityRes::from(|v: String| {
                            match v.contains(\"违规\") {
                                true => false,
                                _ => true
                            }
                        })
                    />
                    <Flex align=\"center\" margin=\"5px 0\">
                        <Text>{\"原生：\"}</Text>
                        <input name=\"ext\" />
                    </Flex>
                    <Flex justify=\"center\" border_top=true border_bottom=true margin=\"15px 0\" padding=\"15px 0\">
                        <Button r#type=\"submit\" label=\"Submit Form\" />
                    </Flex>
                    <Text auto_wrap=true>{self.result.to_json()}</Text>
                </Flex>
            </Form<Data>>
        }
    }
}
                    ")}
                    {static_des(3)}
                    <Flex direction="column" border_top=true margin="10px 0 0 0">
                        {table_th("属性", "说明", "类型", "默认值", "版本")}
                        {table_th("onsubmit", "提交表单后的回调函数", "Callback", "", "")}
                    </Flex>
                </div>
            {switch_bottom("flex", "Flex 布局", "formfield", "FormField 表单域")}
            </Animate>
        }
    }
}
