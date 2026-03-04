use aviutl2::{
    anyhow::{self, Context},
    tracing,
};

#[aviutl2::plugin(GenericPlugin)]
struct ModifyGroupNumLayersAux2 {}

static EDIT_HANDLE: aviutl2::generic::GlobalEditHandle = aviutl2::generic::GlobalEditHandle::new();

impl aviutl2::generic::GenericPlugin for ModifyGroupNumLayersAux2 {
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
            name: "modify_group_num_layers.aux2".into(),
            information: format!(
                "Modify Number of Layers in Group Objects by Shortcut Keys / v{} / https://github.com/sevenc-nanashi/modify_group_num_layers.aux2",
                env!("CARGO_PKG_VERSION")
            ),
        }
    }

    fn register(&mut self, registry: &mut aviutl2::generic::HostAppHandle) {
        registry.register_menus::<Self>();
        EDIT_HANDLE.init(registry.create_edit_handle());
    }
}

impl ModifyGroupNumLayersAux2 {
    fn apply_operation_to_selected_objects<F>(&self, callback: F) -> aviutl2::AnyResult<()>
    where
        F: Fn(
                &mut aviutl2::generic::EditSection,
                aviutl2::generic::ObjectHandle,
            ) -> aviutl2::AnyResult<()>
            + std::marker::Sync,
    {
        EDIT_HANDLE.call_edit_section(|edit| {
            let mut errors = Vec::new();
            let mut selected_objects = edit.get_selected_objects()?;
            if selected_objects.is_empty() {
                selected_objects.push(
                    edit.get_focused_object()?
                        .context("オブジェクトが選択されていません")?,
                );
            }
            tracing::info!(
                "Applying operation to {} selected objects",
                selected_objects.len()
            );
            for object in &selected_objects {
                if let Err(e) = callback(edit, *object) {
                    errors.push((object, e));
                }
            }
            tracing::info!(
                "Applied operation to {} objects, {} of them failed",
                selected_objects.len(),
                errors.len()
            );
            for (object, error) in &errors {
                tracing::error!(
                    "Failed to apply operation to object {:?}: {:?}",
                    object,
                    error
                );
            }
            if errors.len() == selected_objects.len() {
                anyhow::bail!("すべてのオブジェクトに対して操作の適用に失敗しました");
            }
            Ok(())
        })?
    }
}

fn get_num_layers_of_object(
    edit: &mut aviutl2::generic::EditSection,
    object: aviutl2::generic::ObjectHandle,
) -> aviutl2::AnyResult<u32> {
    let alias = edit.object(&object).get_alias_parsed()?;
    let object_0 = alias
        .get_table("Object.0")
        .context("Object.0 テーブルが見つかりません")?;
    let num_layers = object_0
        .parse_value("対象レイヤー数")
        .transpose()?
        .context("対象レイヤー数の値が見つかりません")?;
    Ok(num_layers)
}

fn set_num_layers_of_object(
    edit: &mut aviutl2::generic::EditSection,
    object: aviutl2::generic::ObjectHandle,
    num_layers: u32,
) -> aviutl2::AnyResult<()> {
    let object = edit.object(&object);
    let alias = object.get_alias_parsed()?;
    let object_0 = alias
        .get_table("Object.0")
        .context("Object.0 テーブルが見つかりません")?;
    let effect_name = object_0
        .get_value("effect.name")
        .context("エフェクト名が見つかりません")?;
    object.set_effect_item(effect_name, 0, "対象レイヤー数", &num_layers.to_string())?;
    Ok(())
}

#[aviutl2::generic::menus]
impl ModifyGroupNumLayersAux2 {
    #[edit(name = "modify_group_num_layers.aux2\\対象レイヤー数を増やす")]
    fn increment_layers(&mut self) -> aviutl2::AnyResult<()> {
        return self.apply_operation_to_selected_objects(increment_layers_impl);

        fn increment_layers_impl(
            edit: &mut aviutl2::generic::EditSection,
            object: aviutl2::generic::ObjectHandle,
        ) -> aviutl2::AnyResult<()> {
            let num_layers = get_num_layers_of_object(edit, object)?;
            let to_replace = num_layers
                .checked_add(1)
                .context("対象レイヤー数をこれ以上増やせません")?;
            set_num_layers_of_object(edit, object, to_replace)?;
            Ok(())
        }
    }

    #[edit(name = "modify_group_num_layers.aux2\\対象レイヤー数を減らす")]
    fn decrement_layers(&mut self) -> aviutl2::AnyResult<()> {
        return self.apply_operation_to_selected_objects(decrement_layers_impl);

        fn decrement_layers_impl(
            edit: &mut aviutl2::generic::EditSection,
            object: aviutl2::generic::ObjectHandle,
        ) -> aviutl2::AnyResult<()> {
            let num_layers = get_num_layers_of_object(edit, object)?;
            let to_replace = num_layers
                .checked_sub(1)
                .context("対象レイヤー数をこれ以上減らせません")?;
            set_num_layers_of_object(edit, object, to_replace)?;
            Ok(())
        }
    }

    #[edit(name = "modify_group_num_layers.aux2\\対象レイヤー数を無制限にする")]
    fn set_infinite(&mut self) -> aviutl2::AnyResult<()> {
        return self.apply_operation_to_selected_objects(to_infinite_impl);

        fn to_infinite_impl(
            edit: &mut aviutl2::generic::EditSection,
            object: aviutl2::generic::ObjectHandle,
        ) -> aviutl2::AnyResult<()> {
            get_num_layers_of_object(edit, object)?; // 対象レイヤー数の値が存在することを確認する

            set_num_layers_of_object(edit, object, 0)?;
            Ok(())
        }
    }

    #[edit(name = "modify_group_num_layers.aux2\\対象レイヤー数を1にする")]
    fn set_one(&mut self) -> aviutl2::AnyResult<()> {
        return self.apply_operation_to_selected_objects(to_one_impl);

        fn to_one_impl(
            edit: &mut aviutl2::generic::EditSection,
            object: aviutl2::generic::ObjectHandle,
        ) -> aviutl2::AnyResult<()> {
            get_num_layers_of_object(edit, object)?; // 対象レイヤー数の値が存在することを確認する

            set_num_layers_of_object(edit, object, 1)?;
            Ok(())
        }
    }

    #[object(name = "[modify_group_num_layers.aux2] 対象レイヤー数を増やす")]
    fn increment_layers_object(&mut self) -> aviutl2::AnyResult<()> {
        self.increment_layers()
    }

    #[object(name = "[modify_group_num_layers.aux2] 対象レイヤー数を減らす")]
    fn decrement_layers_object(&mut self) -> aviutl2::AnyResult<()> {
        self.decrement_layers()
    }

    #[object(name = "[modify_group_num_layers.aux2] 対象レイヤー数を無制限にする")]
    fn set_infinite_object(&mut self) -> aviutl2::AnyResult<()> {
        self.set_infinite()
    }

    #[object(name = "[modify_group_num_layers.aux2] 対象レイヤー数を1にする")]
    fn set_one_object(&mut self) -> aviutl2::AnyResult<()> {
        self.set_one()
    }
}

aviutl2::register_generic_plugin!(ModifyGroupNumLayersAux2);
