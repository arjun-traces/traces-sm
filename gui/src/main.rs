mod api;
mod components;

use components::dashboard::Dashboard;
use components::entropy::EntropyView;
use components::header::Header;
use components::lifecycle::LifecycleView;
use components::topology::TopologyView;
use components::zkp_sandbox::ZkpSandboxView;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Tab {
    Dashboard,
    Lifecycle,
    Topology,
    Entropy,
    ZkpSandbox,
}

#[function_component(App)]
fn app() -> Html {
    let current_tab = use_state(|| Tab::Dashboard);

    let on_tab_select = {
        let current_tab = current_tab.clone();
        Callback::from(move |tab: Tab| {
            current_tab.set(tab);
        })
    };

    let render_tab = match *current_tab {
        Tab::Dashboard => html! { <Dashboard /> },
        Tab::Lifecycle => html! { <LifecycleView /> },
        Tab::Topology => html! { <TopologyView /> },
        Tab::Entropy => html! { <EntropyView /> },
        Tab::ZkpSandbox => html! { <ZkpSandboxView /> },
    };

    html! {
        <div class="min-h-screen bg-gray-950 text-gray-100 flex flex-col">
            <Header on_tab_select={on_tab_select} current_tab={(*current_tab).clone()} />
            <main class="flex-grow p-6">
                <div class="max-w-6xl mx-auto">
                    { render_tab }
                </div>
            </main>
        </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
