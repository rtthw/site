
use yew::{
    html::Scope,
    prelude::*,
};
use yew_router::prelude::*;
use rtthw::*;



// ================================================================================================



fn main() {
    yew::Renderer::<App>::new().render();
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}



// ================================================================================================



#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/about")]
    About,
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => {
            html! { <Home /> }
        }
        Route::NotFound => {
            html! { <PageNotFound /> }
        }
        Route::About => {
            html! { <About /> }
        }
    }
}



// ================================================================================================
