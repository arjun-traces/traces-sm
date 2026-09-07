use yew::prelude::*;

#[derive(Clone, PartialEq, Debug)]
pub enum TokenStatus {
    Idle,
    WaitingForTouch,
    Authenticated { key_handle: String, rp_id: String },
    Error(String),
}

#[function_component(WebAuthnView)]
pub fn webauthn_view() -> Html {
    let status = use_state(|| TokenStatus::Idle);
    let rp_name = use_state(|| "traces-sm.internal".to_string());

    let on_start_ceremony = {
        let status = status.clone();
        let rp_name = rp_name.clone();
        Callback::from(move |_| {
            status.set(TokenStatus::WaitingForTouch);
            let status = status.clone();
            let rp_name = (*rp_name).clone();
            // Mock hardware security token / FIDO2 interaction ceremony
            wasm_bindgen_futures::spawn_local(async move {
                // Simulate waiting for hardware token interaction
                status.set(TokenStatus::Authenticated {
                    key_handle: "fido2-cred-secp256r1-9f2b8c".to_string(),
                    rp_id: rp_name,
                });
            });
        })
    };

    let on_reset = {
        let status = status.clone();
        Callback::from(move |_| {
            status.set(TokenStatus::Idle);
        })
    };

    html! {
        <div>
            <h2 class="text-xl font-semibold mb-4 text-white">{"FIDO2 / WebAuthn Hardware Token Unlock"}</h2>
            <div class="bg-gray-800 p-6 rounded-lg shadow border border-gray-700 max-w-2xl">
                <div class="mb-4">
                    <label class="block text-gray-400 text-sm mb-2">{"Relying Party (RP) Identifier"}</label>
                    <input
                        type="text"
                        value={(*rp_name).clone()}
                        disabled=true
                        class="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-gray-200 font-mono text-sm"
                    />
                </div>

                <div class="mb-6">
                    <span class="block text-gray-400 text-sm mb-2">{"Hardware Token Status"}</span>
                    {
                        match &*status {
                            TokenStatus::Idle => html! {
                                <div class="bg-gray-900 p-4 rounded border border-gray-700 text-gray-400 flex items-center justify-between">
                                    <span>{"Ready to authenticate with Security Key (YubiKey / Nitrokey / Apple TouchID)"}</span>
                                    <span class="px-2 py-1 text-xs bg-gray-700 text-gray-300 rounded font-mono">{"IDLE"}</span>
                                </div>
                            },
                            TokenStatus::WaitingForTouch => html! {
                                <div class="bg-amber-900 bg-opacity-30 p-4 rounded border border-amber-700 text-amber-200 flex items-center justify-between animate-pulse">
                                    <span>{"Please touch your hardware security key now..."}</span>
                                    <span class="px-2 py-1 text-xs bg-amber-600 text-white rounded font-mono">{"WAITING_TOUCH"}</span>
                                </div>
                            },
                            TokenStatus::Authenticated { key_handle, rp_id } => html! {
                                <div class="bg-emerald-900 bg-opacity-30 p-4 rounded border border-emerald-700 text-emerald-200">
                                    <div class="flex items-center justify-between mb-2">
                                        <span class="font-semibold text-white">{"✓ Hardware Token Authenticated"}</span>
                                        <span class="px-2 py-1 text-xs bg-emerald-700 text-white rounded font-mono">{"AUTHENTICATED"}</span>
                                    </div>
                                    <div class="text-xs font-mono text-gray-300">
                                        <p>{format!("Credential ID: {}", key_handle)}</p>
                                        <p>{format!("RP Origin: {}", rp_id)}</p>
                                    </div>
                                </div>
                            },
                            TokenStatus::Error(err) => html! {
                                <div class="bg-red-900 bg-opacity-30 p-4 rounded border border-red-700 text-red-200">
                                    <p class="font-semibold">{"Authentication Error"}</p>
                                    <p class="text-xs">{err}</p>
                                </div>
                            }
                        }
                    }
                </div>

                <div class="flex gap-3">
                    <button
                        onclick={on_start_ceremony}
                        class="bg-indigo-600 hover:bg-indigo-700 text-white font-medium px-4 py-2 rounded transition"
                    >
                        {"🔑 Authenticate Hardware Key"}
                    </button>
                    <button
                        onclick={on_reset}
                        class="bg-gray-700 hover:bg-gray-600 text-gray-300 font-medium px-4 py-2 rounded transition"
                    >
                        {"Reset"}
                    </button>
                </div>
            </div>
        </div>
    }
}
