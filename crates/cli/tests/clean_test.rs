mod utils;

use utils::*;

mod clean {
    use super::*;

    #[test]
    fn purges_tool_inventory() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("tools/node/1.2.3/index.js", "");
        sandbox.create_file("tools/node/4.5.6/index.js", "");

        let mut cmd = create_proto_command(sandbox.path());
        cmd.arg("clean")
            .arg("--yes")
            .arg("--purge")
            .arg("node")
            .assert()
            .success();

        assert!(!sandbox.path().join("tools/node/1.2.3/index.js").exists());
        assert!(!sandbox.path().join("tools/node/4.5.6/index.js").exists());
    }

    #[test]
    fn purges_tool_shims() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("bin/node", "");
        sandbox.create_file("bin/node.cmd", "");
        sandbox.create_file("bin/npx", "");
        sandbox.create_file("bin/npx.cmd", "");

        let mut cmd = create_proto_command(sandbox.path());
        cmd.arg("clean")
            .arg("--yes")
            .arg("--purge")
            .arg("node")
            .assert()
            .success();

        if cfg!(windows) {
            assert!(!sandbox.path().join("bin/node.cmd").exists());
            assert!(!sandbox.path().join("bin/npx.cmd").exists());
        } else {
            assert!(!sandbox.path().join("bin/node").exists());
            assert!(!sandbox.path().join("bin/npx").exists());
        }
    }

    #[test]
    fn purges_plugins() {
        let sandbox = create_empty_sandbox();
        sandbox.create_file("plugins/node_plugin.wasm", "");
        sandbox.create_file("plugins/npm_plugin.wasm", "");

        let mut cmd = create_proto_command(sandbox.path());
        cmd.arg("clean")
            .arg("--yes")
            .arg("--purge-plugins")
            .assert()
            .success();

        assert!(!sandbox.path().join("plugins/node_plugin.wasm").exists());
        assert!(!sandbox.path().join("plugins/npm_plugin.wasm").exists());
    }
}
