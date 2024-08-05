use leptos::*;
use leptos_meta::*;
use leptos_router::*;



fn main() {
    mount_to_body(|| view! { <App /> })
}



#[component]
fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Router>
            <nav>
                <A href="about">"About"</A>
            </nav>

            <main>
                <Routes>
                    <Route path="/" view=Home/>
                    <Route path="/about" view=About/>
                    <Route path="/any*" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <Title text="Home"/>
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

#[component]
pub fn About() -> impl IntoView {
    view! {
        <Title text="About"/>
        <div class="container">
            <h1>"About"</h1>
        </div>
    }
}
