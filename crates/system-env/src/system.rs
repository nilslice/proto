use crate::deps::{DependencyConfig, SystemDependency};
use crate::env::*;
use crate::error::Error;
use crate::pm::*;
use crate::pm_vendor::*;

pub struct System {
    pub arch: SystemArch,
    pub manager: SystemPackageManager,
    pub os: SystemOS,
}

impl System {
    pub fn new() -> Result<Self, Error> {
        Ok(System::with_manager(SystemPackageManager::detect()?))
    }

    pub fn with_manager(manager: SystemPackageManager) -> Self {
        System {
            arch: SystemArch::from_env(),
            manager,
            os: SystemOS::from_env(),
        }
    }

    pub fn get_install_package_command(
        &self,
        dep_config: &DependencyConfig,
        interactive: bool,
    ) -> Result<Vec<String>, Error> {
        let os = dep_config.os.unwrap_or(self.os);
        let pm = dep_config.manager.unwrap_or(self.manager);
        let pm_config = pm.get_config();
        let mut args = vec![];

        for arg in pm_config
            .commands
            .get(&Command::InstallPackage)
            .cloned()
            .unwrap()
        {
            if arg == "$" {
                for dep in dep_config.get_package_names(&os, &pm)? {
                    if let Some(ver) = &dep_config.version {
                        match &pm_config.version_arg {
                            VersionArgument::None => {
                                args.push(dep);
                            }
                            VersionArgument::Inline(op) => {
                                args.push(format!("{dep}{op}{ver}"));
                            }
                            VersionArgument::Separate(opt) => {
                                args.push(dep);
                                args.push(opt.to_owned());
                                args.push(ver.to_owned());
                            }
                        };
                    } else {
                        args.push(dep);
                    }
                }
            } else {
                args.push(arg);
            }
        }

        self.append_interactive(Command::InstallPackage, &pm_config, &mut args, interactive);

        Ok(args)
    }

    pub fn get_update_index_command(&self, interactive: bool) -> Option<Vec<String>> {
        let pm_config = self.manager.get_config();

        if let Some(args) = pm_config.commands.get(&Command::UpdateIndex) {
            let mut args = args.to_owned();

            self.append_interactive(Command::UpdateIndex, &pm_config, &mut args, interactive);

            return Some(args);
        }

        None
    }

    pub fn resolve_dependencies(&self, deps: Vec<SystemDependency>) -> Vec<DependencyConfig> {
        let mut configs = vec![];

        for dep in deps {
            let config = dep.to_config();

            if config.os.as_ref().is_some_and(|o| o != &self.os) {
                continue;
            }

            if config.arch.as_ref().is_some_and(|a| a != &self.arch) {
                continue;
            }

            configs.push(config);
        }

        configs
    }

    fn append_interactive(
        &self,
        command: Command,
        config: &PackageVendorConfig,
        args: &mut Vec<String>,
        interactive: bool,
    ) {
        if config.prompt_for.contains(&command) {
            match &config.prompt_arg {
                PromptArgument::None => {}
                PromptArgument::Interactive(i) => {
                    if interactive {
                        args.push(i.to_owned());
                    }
                }
                PromptArgument::Skip(y) => {
                    if !interactive {
                        args.push(y.to_owned());
                    }
                }
            };
        }
    }
}
