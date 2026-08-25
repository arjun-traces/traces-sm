use crate::api::{get_keys, shred_key, transition_key};
use yew::prelude::*;

#[function_component(LifecycleView)]
pub fn lifecycle_view() -> Html {
    let keys = use_state(Vec::new);
    let loading = use_state(|| true);
    let trigger_reload = use_state(|| 0);

    {
        let keys = keys.clone();
        let loading = loading.clone();
        let dep = *trigger_reload;
        use_effect_with(dep, move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let k_res = get_keys().await.unwrap_or_default();
                keys.set(k_res);
                loading.set(false);
            });
            || ()
        });
    }

    let on_transition = {
        let trigger_reload = trigger_reload.clone();
        Callback::from(move |(id, state): (String, String)| {
            let trigger_reload = trigger_reload.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = transition_key(&id, &state).await;
                trigger_reload.set(*trigger_reload + 1);
            });
        })
    };

    let on_shred = {
        let trigger_reload = trigger_reload.clone();
        Callback::from(move |id: String| {
            let trigger_reload = trigger_reload.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let _ = shred_key(&id).await;
                trigger_reload.set(*trigger_reload + 1);
            });
        })
    };

    if *loading {
        return html! { <div class="text-center mt-10 text-gray-400">{"Loading lifecycle matrix..."}</div> };
    }

    html! {
        <div>
            <h2 class="text-xl font-semibold mb-4 text-white">{"Key Lifecycle Matrix"}</h2>
            <div class="bg-gray-900 rounded-lg shadow overflow-hidden border border-gray-700">
                <table class="min-w-full divide-y divide-gray-800">
                    <thead class="bg-gray-800">
                        <tr>
                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">{"Key ID"}</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">{"Algorithm"}</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">{"State"}</th>
                            <th class="px-6 py-3 text-left text-xs font-medium text-gray-400 uppercase tracking-wider">{"Actions"}</th>
                        </tr>
                    </thead>
                    <tbody class="divide-y divide-gray-800">
                        {
                            for keys.iter().map(|k| {
                                let id = k.id.clone();
                                let id_shred = k.id.clone();
                                let on_rotate = {
                                    let on_transition = on_transition.clone();
                                    let id = id.clone();
                                    Callback::from(move |_| on_transition.emit((id.clone(), "Rotated".to_string())))
                                };
                                let on_shred_cb = {
                                    let on_shred = on_shred.clone();
                                    let id = id_shred.clone();
                                    Callback::from(move |_| on_shred.emit(id.clone()))
                                };
                                html! {
                                    <tr class="hover:bg-gray-800 transition-colors">
                                        <td class="px-6 py-4 whitespace-nowrap text-sm font-medium text-gray-200">{&k.id}</td>
                                        <td class="px-6 py-4 whitespace-nowrap text-sm text-gray-400">{&k.algorithm}</td>
                                        <td class="px-6 py-4 whitespace-nowrap text-sm">
                                            <span class="px-2 inline-flex text-xs leading-5 font-semibold rounded-full bg-blue-900 text-blue-200">
                                                {&k.state}
                                            </span>
                                        </td>
                                        <td class="px-6 py-4 whitespace-nowrap text-sm font-medium space-x-3">
                                            <button onclick={on_rotate} class="text-indigo-400 hover:text-indigo-300">{"Rotate"}</button>
                                            <button onclick={on_shred_cb} class="text-red-500 hover:text-red-400">{"Crypto-Shred"}</button>
                                        </td>
                                    </tr>
                                }
                            })
                        }
                    </tbody>
                </table>
                {
                    if keys.is_empty() {
                        html! { <div class="px-6 py-4 text-sm text-gray-500 text-center">{"No keys found."}</div> }
                    } else {
                        html! {}
                    }
                }
            </div>
        </div>
    }
}
