use leptos::*;
use leptos_meta::*;
use leptos_router::*;



fn main() {
    mount_to_body(|| view! { <App /> })
}



#[component]
fn App() -> impl IntoView {
    view! {
        <Html lang="en" dir="ltr" attr:data-theme="light"/>

        // sets the document title
        <Title text="Welcome to Leptos CSR"/>

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8"/>
        <Meta name="viewport" content="width=device-width, initial-scale=1.0"/>

        <Router>
            <Routes>
                <Route path="/" view=Home/>
                <Route path="/*" view=NotFound/>
            </Routes>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="container">
            <h1>"Testing Leptos"</h1>
            <h2><i>"On Github Pages..."</i></h2>
        </div>
    }
}

/// "404: Not Found" Page
#[component]
pub fn NotFound() -> impl IntoView {
    view! { <h1>"Uh oh!" <br/> "We couldn't find that page!"</h1>  }
}
