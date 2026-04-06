use aleph_syntax_tree::syntax::AlephTree;
use em_filter::{async_trait, AgentConfig, EmFilterError, Filter, FilterRunner};
use serde_json::{json, Value};

struct ErlangGenAGent;

#[async_trait]
impl Filter for ErlangGenAGent {
    async fn handle(&mut self, body: &str) -> Result<Value, EmFilterError> {
        tracing::info!(len = body.len(), "generating erlang code");

        let embryo: Value = serde_json::from_str(body)?;
        let tree: AlephTree = serde_json::from_value(embryo["properties"]["tree"].clone())?;
        let source_lang = embryo["properties"]["source_language"]
            .as_str()
            .unwrap_or("unknown")
            .to_string();

        let code = erlanggen::generate(tree);

        tracing::info!(source_lang = %source_lang, "generated erlang code");

        Ok(json!([{
            "type": "code",
            "properties": {
                "language": "erlang",
                "source_language": source_lang,
                "content": code
            }
        }]))
    }

    fn capabilities(&self) -> Vec<String> {
        vec!["generator".into(), "erlang".into()]
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    FilterRunner::new("em_erlang_gen", ErlangGenAGent, AgentConfig::default())
        .run()
        .await
        .unwrap();
}
