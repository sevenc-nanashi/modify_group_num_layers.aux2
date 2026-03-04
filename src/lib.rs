use aviutl2::{config::translate as tr, tracing};

#[aviutl2::plugin(GenericPlugin)]
struct MyAux2TemplateAux2 {}

impl aviutl2::generic::GenericPlugin for MyAux2TemplateAux2 {
    fn new(_info: aviutl2::AviUtl2Info) -> aviutl2::AnyResult<Self> {
        aviutl2::tracing_subscriber::fmt()
            .with_max_level(if cfg!(debug_assertions) {
                aviutl2::tracing::Level::DEBUG
            } else {
                aviutl2::tracing::Level::INFO
            })
            .event_format(aviutl2::logger::AviUtl2Formatter)
            .with_writer(aviutl2::logger::AviUtl2LogWriter)
            .init();

        Ok(Self {})
    }

    fn plugin_info(&self) -> aviutl2::generic::GenericPluginTable {
        aviutl2::generic::GenericPluginTable {
            name: "my_aux2_template.aux2".into(),
            information: format!(
                "My Plugin Template / v{} / https://github.com/sevenc-nanashi/my_aux2_template.aux2",
                env!("CARGO_PKG_VERSION")
            ),
        }
    }

    fn register(&mut self, registry: &mut aviutl2::generic::HostAppHandle) {}
}

aviutl2::register_generic_plugin!(MyAux2TemplateAux2);
