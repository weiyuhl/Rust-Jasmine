use crate::modal_provider::service as modal_provider;

#[flutter_rust_bridge::frb]
pub fn modal_provider_resolve_api_model_id(
    config_json: String,
    model_id: String,
) -> Result<String, String> {
    modal_provider::modal_provider_resolve_api_model_id(config_json, model_id)
}

#[flutter_rust_bridge::frb]
pub fn modal_provider_classify_provider_kind(
    provider_id: String,
    explicit_type: Option<String>,
) -> String {
    modal_provider::modal_provider_classify_provider_kind(provider_id, explicit_type)
}

#[flutter_rust_bridge::frb]
pub fn modal_provider_default_base_url(provider_id: String) -> String {
    modal_provider::modal_provider_default_base_url(provider_id)
}

#[flutter_rust_bridge::frb]
pub fn modal_provider_get_provider_default_headers(config_json: String) -> Result<String, String> {
    modal_provider::modal_provider_get_provider_default_headers(config_json)
}

#[flutter_rust_bridge::frb]
pub fn modal_provider_list_models(
    config_json: String,
    siliconflow_fallback_key: Option<String>,
) -> Result<String, String> {
    modal_provider::modal_provider_list_models(config_json, siliconflow_fallback_key)
}

#[flutter_rust_bridge::frb]
pub fn modal_provider_test_connection(
    config_json: String,
    model_id: String,
    use_stream: bool,
    siliconflow_fallback_key: Option<String>,
) -> Result<(), String> {
    modal_provider::modal_provider_test_connection(
        config_json,
        model_id,
        use_stream,
        siliconflow_fallback_key,
    )
}

#[flutter_rust_bridge::frb]
pub fn modal_provider_validate_provider_config(config_json: String) -> Result<(), String> {
    modal_provider::modal_provider_validate_provider_config(config_json)
}

#[flutter_rust_bridge::frb]
pub fn modal_provider_create_default_config(
    provider_id: String,
    display_name: Option<String>,
) -> Result<String, String> {
    modal_provider::modal_provider_create_default_config(provider_id, display_name)
}

#[flutter_rust_bridge::frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}
