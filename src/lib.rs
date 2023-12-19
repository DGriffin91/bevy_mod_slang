use std::{
    path::{Path, PathBuf},
    process::Command,
};

use bevy::{
    asset::{io::Reader, AssetLoader, AssetPath, LoadContext},
    ecs::system::SystemState,
    prelude::*,
    render::RenderApp,
    utils::{BoxedFuture, HashMap},
};
use serde::{Deserialize, Serialize};

pub struct SlangPlugin;

impl Plugin for SlangPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SlangRegistry>()
            .init_asset::<SlangShader>()
            .register_asset_loader(SlangLoader);
        let Ok(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app.init_resource::<SlangRegistry>();
    }
}

/// Holds Slang shader handles so the file watcher will watch for updates and cause a new spv file to be generated when changes are made.
#[derive(Resource, Default)]
pub struct SlangRegistry(HashMap<PathBuf, Handle<SlangShader>>);

impl SlangRegistry {
    /// Accepted profiles are:
    /// * sm_{4_0,4_1,5_0,5_1,6_0,6_1,6_2,6_3,6_4,6_5,6_6}
    /// * glsl_{110,120,130,140,150,330,400,410,420,430,440,450,460}
    /// Additional profiles that include -stage information:
    /// * {vs,hs,ds,gs,ps}_<version>
    pub fn load<'a>(
        &mut self,
        path: impl Into<AssetPath<'a>> + std::marker::Copy,
        asset_server: &AssetServer,
        profile: &str,
    ) -> Handle<Shader> {
        let p: PathBuf = path.into().into();
        let profile = String::from(profile);
        // TODO skip this if not using "file_watcher" or "asset_processor" features.
        #[cfg(not(target_arch = "wasm32"))]
        {
            let h = asset_server.load_with_settings(path, move |s: &mut SlangSettings| {
                s.profile = profile.clone();
            });
            self.0.insert(p.clone(), h);
        }
        asset_server.load(p.with_extension("spv"))
    }

    /// Accepted profiles are:
    /// * sm_{4_0,4_1,5_0,5_1,6_0,6_1,6_2,6_3,6_4,6_5,6_6}
    /// * glsl_{110,120,130,140,150,330,400,410,420,430,440,450,460}
    /// Additional profiles that include -stage information:
    /// * {vs,hs,ds,gs,ps}_<version>
    pub fn load_from_world<'a>(
        path: impl Into<AssetPath<'a>> + std::marker::Copy,
        world: &mut World,
        profile: &str,
    ) -> Handle<Shader> {
        let mut system_state: SystemState<(Res<AssetServer>, ResMut<SlangRegistry>)> =
            SystemState::new(world);
        let (asset_server, mut slang) = system_state.get_mut(world);
        slang.load(path, &asset_server, profile)
    }
}

#[derive(Asset, TypePath, Debug)]
pub struct SlangShader(PathBuf);

#[derive(Default)]
struct SlangLoader;

#[derive(Default, Serialize, Deserialize)]
struct SlangSettings {
    profile: String,
}

impl AssetLoader for SlangLoader {
    type Asset = SlangShader;
    type Settings = SlangSettings;
    type Error = std::io::Error;
    fn load<'a>(
        &'a self,
        _reader: &'a mut Reader,
        settings: &'a SlangSettings,
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<SlangShader, Self::Error>> {
        let path = Path::new("assets").join(load_context.asset_path().path());
        let mut cmd = Command::new("slangc");
        // TODO allow custom user config
        cmd.arg(path.clone())
            .arg("-profile")
            .arg(&settings.profile)
            .arg("-o")
            .arg(path.with_extension("spv"))
            .arg("-fvk-use-gl-layout");
        if settings.profile.contains("ps_") {
            cmd.arg("-entry").arg("fragment");
            cmd.arg("-fvk-use-entrypoint-name");
        } else if settings.profile.contains("vs_") {
            cmd.arg("-entry").arg("vertex");
            cmd.arg("-fvk-use-entrypoint-name");
        } else {
            cmd.arg("-stage").arg("compute");
        }
        let out = cmd.output().expect("failed to execute process");
        if out.stderr.len() > 1 {
            println!("slang stderr: {}", String::from_utf8_lossy(&out.stderr));
        }

        Box::pin(async move { Ok(SlangShader(path.to_path_buf())) })
    }

    fn extensions(&self) -> &[&str] {
        &["slang"]
    }
}
