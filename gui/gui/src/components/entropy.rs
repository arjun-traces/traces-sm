use crate::api::{get_entropy_health, EntropyHealth};
use yew::prelude::*;

#[function_component(EntropyView)]
pub fn entropy_view() -> Html {
    let health = use_state(|| None::<EntropyHealth>);
    let loading = use_state(|| true);

    {
        let health = health.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                if let Ok(res) = get_entropy_health().await {
                    health.set(Some(res));
                }
                loading.set(false);
            });
            || ()
        });
    }

    if *loading {
        return html! { <div class="text-center mt-10 text-gray-400">{"Loading entropy health..."}</div> };
    }

    html! {
        <div>
            <h2 class="text-xl font-semibold mb-4 text-white">{"NIST SP 800-90B Entropy Status"}</h2>
            {
                if let Some(h) = (*health).as_ref() {
                    html! {
                        <div class="bg-gray-800 p-6 rounded-lg shadow border border-gray-700 max-w-2xl">
                            <div class="mb-4 flex justify-between items-center">
                                <span class="text-gray-400">{"Entropy Source"}</span>
                                <span class="font-mono text-gray-200 bg-gray-900 px-3 py-1 rounded">{&h.source}</span>
                            </div>
                            <div class="mb-4 flex justify-between items-center">
                                <span class="text-gray-400">{"Adaptive Proportion Test (APT)"}</span>
                                <span class="px-3 py-1 rounded bg-green-900 text-green-200 text-sm font-semibold">{&h.apt_status}</span>
                            </div>
                            <div class="mb-6 flex justify-between items-center">
                                <span class="text-gray-400">{"Repetition Count Test (RCT)"}</span>
                                <span class="px-3 py-1 rounded bg-green-900 text-green-200 text-sm font-semibold">{&h.rct_status}</span>
                            </div>
                            <div class="pt-6 border-t border-gray-700 flex justify-between items-center">
                                <span class="text-gray-400 font-medium">{"Estimated Min-Entropy"}</span>
                                <span class="text-3xl font-bold text-indigo-400">{format!("{:.2} bits/byte", h.min_entropy)}</span>
                            </div>
                        </div>
                    }
                } else {
                    html! { <div class="text-red-400 bg-red-900 bg-opacity-20 p-4 rounded border border-red-800">{"Failed to fetch entropy health data from the server."}</div> }
                }
            }
        </div>
    }
}
