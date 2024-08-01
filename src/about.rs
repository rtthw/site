use yew::prelude::*;

pub struct About;

impl Component for About {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <section class="hero is-bold is-large">
                <div class="hero-body">
                    <div class="container">
                        <h1 class="title">
                            { "About" }
                        </h1>
                        <h2 class="subtitle">
                            { "This is the about page" }
                        </h2>
                    </div>
                </div>
            </section>
        }
    }
}
