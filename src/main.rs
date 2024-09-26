#![allow(dead_code)]
// src/main.rs
use crate::prelude::*;

mod config;
mod diagnostic;
mod error;
mod prelude;
mod tool;
mod utils;
    mod job;

struct LintLsServer {
    client: Client,
    config: Arc<Mutex<LintLsConfig>>,
    jobs: Arc<Mutex<HashMap<JobId, Job>>>,
}

impl LintLsServer {
    pub fn new(client: Client, config: LintLsConfig) -> Self {
        Self {
            client,
            config: Arc::new(Mutex::new(config)),
            jobs: Arc::new(Mutex::new(Default::default())),
        }
    }

    async fn run_diagnostics(&self, job_spec: JobSpec) -> Result<()> {
        let job_id = JobId::from(&job_spec);
        let mut jobs = self.jobs.lock().await;
        if let Some(mut ref job) = jobs.get_mut(&job_id) {
            panic!("TODO: Job for ({job_id}) already exists!")
        }
        // HashMap<JobId, Job>
        let tools = self.config.lock().await.tools.clone();
        let Some(extension) = get_extension_from_url(&job_spec.uri) else {
            return;
        };
        for tool in tools.iter().filter(|t| {
            t.match_extensions
                .iter()
                .any(|&match_extension| match_extension == extension)
        }) {
            let job_spec = job_spec.clone();
            if 
            jobs.entry(job_id).or_insert_with(async move {
                let pid: JobPid = run_tool(client, tool, uri, version, file_path).await?;
                Job {
                    job_spec,
                    job_state: JobState::Running { pid },
                }
            });
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for LintLsServer {
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        log::info!(
            "initialize called [params={:?}, lintls_pid={}]",
            params,
            std::process::id()
        );
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        identifier: None,
                        inter_file_dependencies: false,
                        workspace_diagnostics: false,
                        work_done_progress_options: WorkDoneProgressOptions {
                            work_done_progress: None,
                        },
                    },
                )),
                ..ServerCapabilities::default()
            },
            server_info: Some(ServerInfo {
                name: "lintls".to_string(),
                version: None,
            }),
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "lintls Server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        log::info!("[LintLsServer::did_open] called [params={params:?}]");

        self.run_diagnostics(JobSpec {
            uri: params.text_document.uri,
            version: params.text_document.version,
            language_id: Some(params.text_document.language_id),
            text: params.text_document.text,
        })
        .await;
    }
    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        log::info!("[LintLsServer::did_change] called [params={params:?}]");
        assert!(params.content_changes.len() == 1);
        self.run_diagnostics(JobSpec {
            uri: params.text_document.uri,
            version: params.text_document.version,
            language_id: None,
            text: params.content_changes.remove(0).text,
        })
        .await;
    }

    // Implement other necessary methods like did_change or did_save if needed.
}

#[tokio::main]
async fn main() -> Result<()> {
    simple_logging::log_to_file("lintls.log", log::LevelFilter::Trace).unwrap();

    let config_content: Option<String> = read_to_string("config.toml").ok();
    let config =
        config_content.map_or_else(Default::default, |content| config::parse_config(&content));

    let stdin = io::stdin();
    let stdout = io::stdout();
    let (service, socket) = LspService::build(|client| LintLsServer::new(client, config)).finish();
    Server::new(stdin, stdout, socket).serve(service).await;
    Ok(())
}
