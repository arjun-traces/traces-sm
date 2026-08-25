use crate::api::{get_keys, get_secrets};
use yew::prelude::*;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let secrets_count = use_state(|| 0);
    let keys_count = use_state(|| 0);
    let loading = use_state(|| true);

    {
        let secrets_count = secrets_count.clone();
        let keys_count = keys_count.clone();
        let loading = loading.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let s_res = get_secrets().await.map(|v| v.len()).unwrap_or(0);
                let k_res = get_keys().await.map(|v| v.len()).unwrap_or(0);
                secrets_count.set(s_res);
                keys_count.set(k_res);
                loading.set(false);
            });
            || ()
        });
    }

    if *loading {
        return html! { <div class="text-center mt-10 text-gray-400">{"Loading dashboard..."}</div> };
    }

    html! {
        <div>
            <h2 class="text-xl font-semibold mb-4 text-white">{"System Dashboard"}</h2>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                <div class="bg-gray-800 p-6 rounded-lg shadow border border-gray-700">
                    <h3 class="text-lg font-medium text-gray-400 mb-2">{"Secrets Overview"}</h3>
                    <p class="text-4xl font-bold text-white">{*secrets_count}</p>
                    <p class="text-sm text-gray-500 mt-2">{"Total Active Secrets"}</p>
                </div>
                <div class="bg-gray-800 p-6 rounded-lg shadow border border-gray-700">
                    <h3 class="text-lg font-medium text-gray-400 mb-2">{"Keys Overview"}</h3>
                    <p class="text-4xl font-bold text-white">{*keys_count}</p>
                    <p class="text-sm text-gray-500 mt-2">{"Managed Keys"}</p>
                </div>
            </div>
        </div>
    }
}
