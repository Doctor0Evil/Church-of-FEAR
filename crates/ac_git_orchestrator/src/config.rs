use ac_aln_rt::model::ScriptConfig;

pub fn git_script_config() -> ScriptConfig {
    ScriptConfig {
        session_key_template: "git_session:{user_id}".to_string(),
        bot_id: "git_bot".to_string(),
        virtual_fs: "/alien-vfs/git-commands/invocations-001/".to_string(),
    }
}
