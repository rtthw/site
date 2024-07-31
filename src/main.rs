use yew::prelude::*;



const HOME_HTML: &str = include_str!("home.html");



// ================================================================================================



fn main() {
    yew::Renderer::<Index>::new().render();
}



// ================================================================================================



struct Index;

impl Component for Index {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        Html::from_html_unchecked(HOME_HTML.into())
    }
}



// ================================================================================================
