use eframe::egui;

fn main() -> eframe::Result<()> {
    tracing_subscriber::fmt::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1280.0, 800.0])
            .with_title("traces-sm — SGX Secrets & Key Management Console"),
        ..Default::default()
    };
    
    eframe::run_native(
        "traces-sm",
        options,
        Box::new(|_cc| Ok(Box::new(TracesSmApp::default()))),
    )
}

struct TracesSmApp {
    active_tab: String,
    sgx_mode: String,
    show_ai_panel: bool,
    anthropic_api_key: String,
    ai_input: String,
    ai_messages: Vec<(String, String)>,
}

impl Default for TracesSmApp {
    fn default() -> Self {
        Self {
            active_tab: "dashboard".to_string(),
            sgx_mode: "HW_ACTIVE".to_string(),
            show_ai_panel: true,
            anthropic_api_key: String::new(),
            ai_input: String::new(),
            ai_messages: vec![
                ("assistant".to_string(), "Enclave is HW_ACTIVE and all six security policy invariants are enforcing. Ask me about a key, a policy decision, or an attestation quote.".to_string())
            ],
        }
    }
}

impl eframe::App for TracesSmApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top Header Panel
        egui::TopBottomPanel::top("header_panel").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("🔒 traces-sm").bold().size(16.0));
                ui.label(egui::RichText::new("SGX Secrets & Key Management").size(12.0).color(egui::Color32::GRAY));
                
                ui.separator();
                ui.label(egui::RichText::new("Ubuntu 24.04 (GTK3)").monospace().size(10.0));
                ui.label(egui::RichText::new("SGX HW_ACTIVE").monospace().bold().color(egui::Color32::GREEN));
                ui.label(egui::RichText::new("RA-TLS VERIFIED").monospace().color(egui::Color32::LIGHT_BLUE));

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label("A admin-role");
                    if ui.button(if self.show_ai_panel { "✦ Traces AI (Open)" } else { "✦ Traces AI" }).clicked() {
                        self.show_ai_panel = !self.show_ai_panel;
                    }
                });
            });
        });

        // Bottom Enclave Device Status Bar
        egui::TopBottomPanel::bottom("bottom_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("ENCLAVE DEVICE: /dev/sgx_enclave | HW_ACTIVE | EPC 21.2 MB / 64.0 MB | RA-TLS VERIFIED").monospace().size(11.0).color(egui::Color32::LIGHT_GRAY));
            });
        });

        // Left Sidebar Navigation
        egui::SidePanel::left("left_sidebar").resizable(false).default_width(200.0).show(ctx, |ui| {
            ui.add_space(8.0);
            ui.label(egui::RichText::new("KEYS & SECRETS").bold().size(11.0).color(egui::Color32::GRAY));
            ui.selectable_value(&mut self.active_tab, "dashboard".to_string(), "• Dashboard");
            ui.selectable_value(&mut self.active_tab, "lifecycle".to_string(), "• Key Lifecycle [8]");
            ui.selectable_value(&mut self.active_tab, "vault".to_string(), "• Vault [14]");

            ui.add_space(12.0);
            ui.label(egui::RichText::new("NETWORK").bold().size(11.0).color(egui::Color32::GRAY));
            ui.selectable_value(&mut self.active_tab, "topology".to_string(), "• DKG Topology [3]");

            ui.add_space(12.0);
            ui.label(egui::RichText::new("CRYPTOGRAPHY").bold().size(11.0).color(egui::Color32::GRAY));
            ui.selectable_value(&mut self.active_tab, "entropy".to_string(), "• Entropy");
            ui.selectable_value(&mut self.active_tab, "zkp".to_string(), "• ZKP Sandbox");

            ui.add_space(12.0);
            ui.label(egui::RichText::new("GOVERNANCE").bold().size(11.0).color(egui::Color32::GRAY));
            ui.selectable_value(&mut self.active_tab, "policy".to_string(), "• Policy");
            ui.selectable_value(&mut self.active_tab, "audit".to_string(), "• Audit Logs");
        });

        // Right-hand Traces AI Panel (Anthropic API Ready)
        if self.show_ai_panel {
            egui::SidePanel::right("traces_ai_panel").resizable(true).default_width(280.0).show(ctx, |ui| {
                ui.heading("✦ Traces AI");
                ui.label(egui::RichText::new("Anthropic API Ready - No secrets leave host").size(10.0).color(egui::Color32::GRAY));
                ui.separator();

                ui.label("Anthropic API Key:");
                ui.add(egui::TextEdit::singleline(&mut self.anthropic_api_key).password(true).hint_text("sk-ant-api..."));
                ui.separator();

                egui::ScrollArea::vertical().max_height(450.0).show(ui, |ui| {
                    for (sender, txt) in &self.ai_messages {
                        if sender == "user" {
                            ui.colored_label(egui::Color32::LIGHT_BLUE, format!("YOU: {}", txt));
                        } else {
                            ui.colored_label(egui::Color32::LIGHT_GRAY, format!("TRACES AI: {}", txt));
                        }
                        ui.add_space(4.0);
                    }
                });

                ui.separator();
                ui.horizontal(|ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.ai_input).hint_text("Ask about keys, policy..."));
                    if ui.button("Send").clicked() {
                        if !self.ai_input.trim().is_empty() {
                            let input_copy = self.ai_input.clone();
                            self.ai_messages.push(("user".to_string(), input_copy.clone()));
                            
                            let reply = if input_copy.contains("k-104") {
                                "Key k-104 (RSA-2048) was deactivated because its cryptoperiod volume limit reached 4.2 GB. It is restricted to historical decryption."
                            } else if !self.anthropic_api_key.is_empty() {
                                "Connected to Anthropic API (Claude 3.5 Sonnet). Enclave telemetry: HW_ACTIVE, 6/6 invariants enforcing."
                            } else {
                                "Traces AI: Enclave telemetry healthy. Pass your Anthropic API Key above to activate live Claude intelligence."
                            };
                            self.ai_messages.push(("assistant".to_string(), reply.to_string()));
                            self.ai_input.clear();
                        }
                    }
                });
            });
        }

        // Main Content Area matching exact PDF screens
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.active_tab.as_str() {
                "dashboard" => {
                    ui.heading("Executive Dashboard");
                    ui.label("Enclave health, key counts, cryptoperiod limits and cryptographic throughput.");
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.group(|ui| { ui.label("ACTIVE NIST KEYS"); ui.heading("8"); ui.label("RSA · ECDSA · ML-KEM"); });
                        ui.group(|ui| { ui.label("SEALED SECRETS"); ui.heading("14"); ui.label("AES-256-GCM Envelope"); });
                        ui.group(|ui| { ui.label("DKG PEER QUORUM"); ui.heading("3"); ui.label("2-of-3 RA-TLS Connected"); });
                        ui.group(|ui| { ui.label("SP 800-90B DRBG"); ui.heading("HEALTHY"); ui.label("APT & RCT Passed"); });
                    });
                }
                "lifecycle" => {
                    ui.heading("NIST SP 800-57 Key Lifecycle Matrix");
                    ui.label("NIST SP 800-57 Part 1 Rev. 5 — pre-operational through destroyed.");
                    ui.separator();
                    ui.label("Key table: rsa-master-cert (RSA-4096), ecdsa-token-sign (ECDSA-P256), pqc-kyber-kem (ML-KEM-768), legacy-rsa-2048 (DEACTIVATED).");
                }
                "vault" => {
                    ui.heading("Sealed Secret Vault");
                    ui.label("Envelope-encrypted payloads, tokens, symmetric keys and certificate bundles.");
                }
                "topology" => {
                    ui.heading("Distributed Key Generation & Node Topology");
                    ui.label("2-of-3 threshold quorum with RA-TLS peer attestation.");
                }
                "entropy" => {
                    ui.heading("DRBG & Entropy Health");
                    ui.label("NIST SP 800-90A/B/C continuous health testing on hardware entropy source.");
                }
                "zkp" => {
                    ui.heading("ZKP & Homomorphic Encryption Sandbox");
                    ui.label("Schnorr PoK, Bulletproofs range proofs, and Paillier PHE.");
                }
                "policy" => {
                    ui.heading("Mandatory Security Policy");
                    ui.label("Governance invariants enforced by enclave/src/policy.rs.");
                }
                "audit" => {
                    ui.heading("Audit Logs & Attestation Quotes");
                    ui.label("Non-repudiable audit trail and raw Intel DCAP quote inspection.");
                }
                _ => {}
            }
        });
    }
}
