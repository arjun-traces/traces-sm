use crate::api::get_dkg_nodes;
use yew::prelude::*;

#[function_component(TopologyView)]
pub fn topology_view() -> Html {
    let nodes = use_state(Vec::new);
    let loading = use_state(|| true);

    {
        let nodes = nodes.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = get_dkg_nodes().await.unwrap_or_default();
                nodes.set(res);
                loading.set(false);
            });
            || ()
        });
    }

    if *loading {
        return html! { <div class="text-center mt-10 text-gray-400">{"Loading topology..."}</div> };
    }

    html! {
        <div>
            <h2 class="text-xl font-semibold mb-4 text-white">{"DKG Node Topology"}</h2>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-6">
                {
                    if nodes.is_empty() {
                        html! { <div class="col-span-3 text-gray-500 text-center">{"No DKG nodes registered."}</div> }
                    } else {
                        html! {
                            for nodes.iter().map(|n| html! {
                                <div class="bg-gray-800 p-6 rounded-lg border border-gray-700 flex flex-col items-center shadow-sm">
                                    <div class="w-16 h-16 rounded-full bg-indigo-900 flex items-center justify-center mb-4">
                                        <svg class="w-8 h-8 text-indigo-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                                        </svg>
                                    </div>
                                    <h3 class="text-lg font-medium text-gray-200">{&n.id}</h3>
                                    <p class="text-sm text-gray-400 mt-1 font-mono">{&n.address}</p>
                                    <span class="mt-4 px-3 py-1 rounded-full text-xs font-bold bg-green-900 text-green-200">
                                        {&n.status}
                                    </span>
                                </div>
                            })
                        }
                    }
                }
            </div>
        </div>
    }
}
