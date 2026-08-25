use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct HeaderProps {
    pub active_tab: String,
    pub on_tab_change: Callback<String>,
    pub sgx_mode: String,
    pub on_toggle_ai: Callback<()>,
}

#[function_component(Header)]
pub fn header(props: &HeaderProps) -> Html {
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

    let on_ai_click = {
        let cb = props.on_toggle_ai.clone();
        Callback::from(move |_| cb.emit(()))
    };

    html! {
        <header className="border-b border-gray-800 bg-gray-900/90 backdrop-blur sticky top-0 z-50">
            <div className="max-w-7xl mx-auto px-6 py-3 flex items-center justify-between">
                <div className="flex items-center space-x-3">
                    <div className="p-2 bg-indigo-600/20 rounded-xl border border-indigo-500/30 text-indigo-400 font-mono text-sm font-bold">
                        {"SGX"}
                    </div>
                    <div>
                        <h1 className="text-base font-bold tracking-tight text-white flex items-center gap-2">
                            {"traces-sm — SGX Secrets & Key Management Console"}
                        </h1>
                        <p className="text-[11px] text-gray-400">{"100% Rust-Native Multi-OS Console"}</p>
                    </div>
                </div>

                <div className="flex items-center space-x-3 text-xs font-mono">
                    <span className="px-2.5 py-1 rounded bg-gray-800 border border-gray-700 text-gray-300">
                        {"Ubuntu 24.04 (GTK3)"}
                    </span>
                    <span className="px-2.5 py-1 rounded bg-emerald-500/10 border border-emerald-500/30 text-emerald-400 font-bold">
                        {"SGX HW_ACTIVE"}
                    </span>
                    <span className="px-2.5 py-1 rounded bg-indigo-500/10 border border-indigo-500/30 text-indigo-300">
                        {"RA-TLS VERIFIED"}
                    </span>
                    <button 
                        onclick={on_ai_click}
                        className="px-3 py-1.5 rounded-lg bg-indigo-600/30 border border-indigo-500/40 text-indigo-200 font-semibold hover:bg-indigo-600/50 flex items-center space-x-1">
                        <span>{"✦ Traces AI"}</span>
                    </button>
                    <div className="w-7 h-7 rounded-full bg-indigo-600 flex items-center justify-center text-white font-bold text-xs">
                        {"A"}
                    </div>
                </div>
            </div>
        </header>
    }
}
