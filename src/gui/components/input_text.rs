use std::cell::{Ref, RefMut};
use yew::{prelude::*, utils::NeqAssign};

use super::CallbackType;
use crate::gui::{components::Helper, RcUi};

pub enum Msg {
    Input(String),
}

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub label: String,
    pub value: RcUi<String>,
    pub helper: Option<&'static str>,
    pub oninput: Option<Callback<CallbackType>>,
}

impl Props {
    fn value(&self) -> Ref<'_, String> {
        self.value.borrow()
    }

    fn value_mut(&mut self) -> RefMut<'_, String> {
        self.value.borrow_mut()
    }
}

pub struct InputText {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for InputText {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        InputText { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Input(value) => {
                if let Some(ref callback) = self.props.oninput {
                    callback.emit(CallbackType::String(value.to_owned()));
                }

                *self.props.value_mut() = value;
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn view(&self) -> Html {
        let helper = self.props.helper.as_ref().map(|&helper| {
            html! {
                <Helper text={helper} />
            }
        });
        let value = self.props.value().to_owned();
        let oninput = self.link.callback(|data: InputData| Msg::Input(data.value));
        html! {
            <label class="flex-auto flex items-center gap-1">
                <input type="text" class="input w-2/3" placeholder="<empty>" {value} {oninput} />
                { &self.props.label }
                { for helper }
            </label>
        }
    }
}