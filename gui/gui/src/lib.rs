use yew::prelude::*;
use wasm_bindgen::prelude::*;

mod api;
mod components;

use components::header::Header;
use components::dashboard::DashboardView;
use components::lifecycle::KeyLifecycleView;
use components::topology::TopologyView;
use components::entropy::EntropyView;
use components::zkp_sandbox::ZkpSandboxView;
use components::traces_ai::TracesAiPanel;

#[function_component(App)]
pub fn app() -> Html {
    let active_tab = use_state(|| "dashboard".to_string());
    let show_ai = use_state(|| true);

    let on_tab_change = {
        let active_tab = active_tab.clone();
        Callback::from(move |tab: String| {
            active_tab.set(tab);
        })
    };

    let on_toggle_ai = {
        let show_ai = show_ai.clone();
        Callback::from(move |_| {
            show_ai.set(!*show_ai);
        })
    };

    let on_close_ai = {
        let show_ai = show_ai.clone();
        Callback::from(move |_| {
            show_ai.set(false);
        })
    };

    let nav_groups = vec![
        ("KEYS & SECRETS", vec![
            ("dashboard", "• Dashboard", "⌘1"),
            ("lifecycle", "• Key Lifecycle [8]", "⌘2"),
            ("vault",     "• Vault [14]", "⌘3"),
        ]),
        ("NETWORK", vec![
            ("topology",  "• DKG Topology [3]", "⌘4"),
        ]),
        ("CRYPTOGRAPHY", vec![
            ("entropy",   "• Entropy", "⌘5"),
            ("zkp",       "• ZKP Sandbox", "⌘6"),
        ]),
        ("GOVERNANCE", vec![
            ("policy",    "• Policy", "⌘7"),
            ("audit",     "• Audit Logs", "⌘8"),
        ]),
    ];

    html! {
        <div className="min-h-screen bg-gray-950 text-gray-100 flex flex-col font-sans">
            <Header 
                active_tab={(*active_tab).clone()} 
                on_tab_change={on_tab_change.clone()} 
                sgx_mode="HW_ACTIVE"
                on_toggle_ai={on_toggle_ai}
            />

            <div className="flex-1 flex overflow-hidden">
                <!-- Left Sidebar Navigation matching PDF screens -->
                <aside className="w-64 border-r border-gray-800/80 bg-gray-900/60 p-4 space-y-6 flex-shrink-0">
                    {
                        nav_groups.into_iter().map(|(group, links)| {
                            html! {
                                <div className="space-y-1">
                                    <h3 className="text-[10px] font-bold font-mono text-gray-500 px-3 uppercase tracking-wider">{group}</h3>
                                    {
                                        links.into_iter().map(|(id, label, shortcut)| {
                                            let is_active = *active_tab == id;
                                            let on_click = {
                                                let cb = on_tab_change.clone();
                                                let id_str = id.to_string();
                                                Callback::from(move |_| cb.emit(id_str.clone()))
                                            };
                                            let cls = if is_active {
                                                "w-full flex items-center justify-between px-3 py-2 text-xs font-semibold rounded-lg bg-indigo-600/20 text-indigo-300 border border-indigo-500/30"
                                            } else {
                                                "w-full flex items-center justify-between px-3 py-2 text-xs text-gray-400 rounded-lg hover:bg-gray-800/60 hover:text-gray-200"
                                            };
                                            html! {
                                                <button onclick={on_click} className={cls}>
                                                    <span>{label}</span>
                                                    <span className="font-mono text-[10px] opacity-40">{shortcut}</span>
                                                </button>
                                            }
                                        }).collect::<Html>()
                                    }
                                </div>
                            }
                        }).collect::<Html>()
                    }

                    <div className="pt-4 border-t border-gray-800 text-[10px] font-mono text-gray-400 space-y-1">
                        <p>{"ENCLAVE DEVICE:"}</p>
                        <p className="text-emerald-400 font-bold">{"/dev/sgx_enclave"}</p>
                        <p>{"EPC 21.2 MB / 64.0 MB"}</p>
                    </div>
                </aside>

                <!-- Center Main Content -->
                <main className="flex-1 p-8 overflow-y-auto">
                    {
                        match active_tab.as_str() {
                            "dashboard" => html! { <DashboardView /> },
                            "lifecycle" => html! { <KeyLifecycleView /> },
                            "topology"  => html! { <TopologyView /> },
                            "entropy"   => html! { <EntropyView /> },
                            "zkp"       => html! { <ZkpSandboxView /> },
                            _           => html! { <DashboardView /> },
                        }
                    }
                </main>

                <!-- Right Traces AI Panel -->
                <TracesAiPanel is_open={*show_ai} on_close={on_close_ai} />
            </div>
        </div>
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("main")
            .unwrap()
    ).render();
}
