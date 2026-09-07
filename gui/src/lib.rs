//! `traces-sm` WebAssembly Frontend Application (Yew / Web-Sys).
//!
//! # Architecture & UI Structure
//! Provides a reactive Single-Page Application (SPA) compiled to WebAssembly via `wasm-bindgen`
//! and `yew`.
//!
//! # Components & Routing
//! - [`Header`]: Top navigation bar displaying SGX hardware mode, RA-TLS status, and AI toggle.
//! - [`Dashboard`]: System overview of sealed secrets and key counts.
//! - [`LifecycleView`]: NIST SP 800-57 key lifecycle transition and SP 800-88 crypto-shredding table.
//! - [`TopologyView`]: DKG 2-of-3 threshold peer node cluster status.
//! - [`EntropyView`]: NIST SP 800-90B DRBG continuous health telemetry (APT & RCT).
//! - [`ZkpSandboxView`]: Interactive Schnorr PoK, Bulletproofs, and Paillier PHE cryptographic sandbox.
//! - [`TracesAiPanel`]: Side drawer for querying enclave telemetry and key metadata.

use wasm_bindgen::prelude::*;
use yew::prelude::*;

mod api;
mod components;

use components::dashboard::Dashboard;
use components::entropy::EntropyView;
use components::header::Header;
use components::lifecycle::LifecycleView;
use components::topology::TopologyView;
use components::traces_ai::TracesAiPanel;
use components::zkp_sandbox::ZkpSandboxView;

/// Root component for the `traces-sm` WebAssembly GUI.
///
/// Manages active tab state, AI panel drawer toggle, and routes views dynamically.
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
        (
            "KEYS & SECRETS",
            vec![
                ("dashboard", "• Dashboard", "⌘1"),
                ("lifecycle", "• Key Lifecycle [8]", "⌘2"),
                ("vault", "• Vault [14]", "⌘3"),
            ],
        ),
        ("NETWORK", vec![("topology", "• DKG Topology [3]", "⌘4")]),
        (
            "CRYPTOGRAPHY",
            vec![
                ("entropy", "• Entropy", "⌘5"),
                ("zkp", "• ZKP Sandbox", "⌘6"),
            ],
        ),
        (
            "GOVERNANCE",
            vec![
                ("policy", "• Policy", "⌘7"),
                ("audit", "• Audit Logs", "⌘8"),
            ],
        ),
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

                <main className="flex-1 p-8 overflow-y-auto">
                    {
                        match active_tab.as_str() {
                            "dashboard" => html! { <Dashboard /> },
                            "lifecycle" => html! { <LifecycleView /> },
                            "topology"  => html! { <TopologyView /> },
                            "entropy"   => html! { <EntropyView /> },
                            "zkp"       => html! { <ZkpSandboxView /> },
                            _           => html! { <Dashboard /> },
                        }
                    }
                </main>

                <TracesAiPanel is_open={*show_ai} on_close={on_close_ai} />
            </div>
        </div>
    }
}

/// WebAssembly entry point initializing logger and mounting the [`App`] component to the DOM.
#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::with_root(
        web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("main")
            .unwrap(),
    )
    .render();
}
