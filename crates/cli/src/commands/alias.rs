use clap::Args;
use proto_core::{is_alias_name, load_tool, Id, ProtoError, UnresolvedVersionSpec};
use starbase::system;
use starbase_styles::color;
use tracing::info;

#[derive(Args, Clone, Debug)]
pub struct AliasArgs {
    #[arg(required = true, help = "ID of tool")]
    id: Id,

    #[arg(required = true, help = "Alias name")]
    alias: String,

    #[arg(required = true, help = "Version or alias to associate with")]
    spec: UnresolvedVersionSpec,
}

#[system]
pub async fn alias(args: ArgsRef<AliasArgs>) {
    if let UnresolvedVersionSpec::Alias(inner_alias) = &args.spec {
        if &args.alias == inner_alias {
            return Err(ProtoError::Message("Cannot map an alias to itself.".into()))?;
        }
    }

    if !is_alias_name(&args.alias) {
        return Err(ProtoError::Message(
            "Versions cannot be aliases. Use alphanumeric words instead.".into(),
        ))?;
    }

    let mut tool = load_tool(&args.id).await?;

    tool.manifest
        .aliases
        .insert(args.alias.clone(), args.spec.clone());
    tool.manifest.save()?;

    info!(
        "Added alias {} ({}) for {}",
        color::id(&args.alias),
        color::muted_light(args.spec.to_string()),
        tool.get_name(),
    );
}
