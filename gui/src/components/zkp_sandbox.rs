use yew::prelude::*;

#[function_component(ZkpSandboxView)]
pub fn zkp_sandbox_view() -> Html {
    let logs = use_state(Vec::new);

    let add_log = {
        let logs = logs.clone();
        move |msg: String| {
            let mut current = (*logs).clone();
            current.push(msg);
            logs.set(current);
        }
    };

    let on_schnorr = {
        let add_log = add_log.clone();
        Callback::from(move |_| {
            add_log("Generated Schnorr PoK for discrete log.".into());
        })
    };

    let on_bulletproof = {
        let add_log = add_log.clone();
        Callback::from(move |_| {
            add_log("Generated Bulletproofs range proof [0, 2^64).".into());
        })
    };

    let on_paillier = {
        let add_log = add_log.clone();
        Callback::from(move |_| {
            add_log("Performed Paillier Homomorphic Addition: E(a) * E(b) = E(a+b).".into());
        })
    };

    let on_clear = {
        let logs = logs.clone();
        Callback::from(move |_| {
            logs.set(Vec::new());
        })
    };

    html! {
        <div>
            <h2 class="text-xl font-semibold mb-4 text-white">{"ZKP & Homomorphic Sandbox"}</h2>
            <div class="grid grid-cols-1 lg:grid-cols-2 gap-6">
                <div class="bg-gray-800 p-6 rounded-lg shadow border border-gray-700 flex flex-col space-y-4">
                    <h3 class="text-lg font-medium text-gray-300">{"Crypto Operations"}</h3>
                    <button onclick={on_schnorr} class="w-full bg-indigo-600 hover:bg-indigo-700 text-white font-medium py-3 px-4 rounded transition-colors shadow-sm">
                        {"Simulate Schnorr PoK"}
                    </button>
                    <button onclick={on_bulletproof} class="w-full bg-purple-600 hover:bg-purple-700 text-white font-medium py-3 px-4 rounded transition-colors shadow-sm">
                        {"Simulate Bulletproof Range Proof"}
                    </button>
                    <button onclick={on_paillier} class="w-full bg-blue-600 hover:bg-blue-700 text-white font-medium py-3 px-4 rounded transition-colors shadow-sm">
                        {"Simulate Paillier PHE Add"}
                    </button>
                </div>
                
                <div class="bg-gray-900 rounded-lg shadow border border-gray-700 flex flex-col h-80">
                    <div class="px-4 py-3 border-b border-gray-800 flex justify-between items-center bg-gray-800 rounded-t-lg">
                        <h3 class="text-gray-400 text-sm font-medium">{"Operation Log"}</h3>
                        <button onclick={on_clear} class="text-xs text-gray-500 hover:text-white transition-colors">{"Clear"}</button>
                    </div>
                    <div class="p-4 flex-grow overflow-y-auto font-mono text-sm text-green-400 space-y-1">
                        {
                            if logs.is_empty() {
                                html! { <div class="text-gray-600 italic">{"No operations executed yet."}</div> }
                            } else {
                                html! {
                                    for logs.iter().map(|log| html! {
                                        <div><span class="text-gray-600 mr-2">{">"}</span>{log}</div>
                                    })
                                }
                            }
                        }
                    </div>
                </div>
            </div>
        </div>
    }
}
