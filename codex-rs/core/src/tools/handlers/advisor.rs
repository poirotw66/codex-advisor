use crate::function_tool::FunctionCallError;
use crate::tools::context::FunctionToolOutput;
use crate::tools::context::ToolInvocation;
use crate::tools::context::ToolPayload;
use crate::tools::context::boxed_tool_output;
use crate::tools::handlers::parse_arguments;
use crate::tools::registry::CoreToolRuntime;
use crate::tools::registry::ToolExecutor;
use codex_tools::JsonSchema;
use codex_tools::ResponsesApiTool;
use codex_tools::ToolName;
use codex_tools::ToolSpec;
use serde::Deserialize;
use std::collections::BTreeMap;
use std::time::Duration;

const MAX_BRIEF_BYTES: usize = 16 * 1024;
const CONSULTATION_TIMEOUT: Duration = Duration::from_secs(60);
const TOOL_NAME: &str = "advisor";

pub(crate) struct AdvisorHandler {
    model: String,
}

impl AdvisorHandler {
    pub(crate) fn new(model: String) -> Self {
        Self { model }
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct AdvisorArgs {
    brief: String,
}

impl ToolExecutor<ToolInvocation> for AdvisorHandler {
    fn tool_name(&self) -> ToolName {
        ToolName::plain(TOOL_NAME)
    }

    fn spec(&self) -> ToolSpec {
        ToolSpec::Function(ResponsesApiTool {
            name: TOOL_NAME.to_string(),
            description: format!(
                "Ask {} for a second opinion at a consequential decision, after recurring                  failures, or before declaring complex work complete when uncertainty remains.                  Use selectively. The advisor sees only the brief, has no tools, and returns                  advice for you to evaluate. You remain responsible for execution and results.",
                self.model
            ),
            strict: true,
            defer_loading: None,
            parameters: JsonSchema::object(
                BTreeMap::from([(
                    "brief".to_string(),
                    JsonSchema::string(Some(
                        "State the decision, goal, relevant verified evidence, options, and                          uncertainty. Include only necessary facts. Omit secrets and the full transcript."
                            .to_string(),
                    )),
                )]),
                /*required*/ Some(vec!["brief".to_string()]),
                /*additional_properties*/ Some(false.into()),
            ),
            output_schema: None,
        })
    }

    fn handle<'a>(&'a self, invocation: ToolInvocation) -> codex_tools::ToolExecutorFuture<'a>
    where
        ToolInvocation: 'a,
    {
        Box::pin(async move {
            let ToolPayload::Function { arguments } = &invocation.payload else {
                return Err(FunctionCallError::RespondToModel(
                    "Advisor requires a decision brief.".to_string(),
                ));
            };
            let args: AdvisorArgs = parse_arguments(arguments)?;
            let brief = args.brief.trim();
            if brief.is_empty() || brief.len() > MAX_BRIEF_BYTES {
                return Err(FunctionCallError::RespondToModel(format!(
                    "Advisor requires a nonempty decision brief of at most {MAX_BRIEF_BYTES} bytes."
                )));
            }

            let result = tokio::select! {
                biased;
                () = invocation.cancellation_token.cancelled() => None,
                result = tokio::time::timeout(
                    CONSULTATION_TIMEOUT,
                    invocation.session.consult_advisor(&invocation.step_context, &self.model, brief),
                ) => Some(result),
            };
            let (message, success) = match result {
                Some(Ok(Ok(Some(advice)))) => {
                    (format!("Advisor ({})\n\n{advice}", self.model), true)
                }
                Some(Ok(Ok(None))) => ("Advisor did not return usable advice.".to_string(), false),
                Some(Ok(Err(_))) => {
                    tracing::warn!(model = %self.model, "Advisor consultation failed");
                    (
                        "Advisor unavailable. Continue without advisor evidence.".to_string(),
                        false,
                    )
                }
                Some(Err(_)) => ("Advisor timed out.".to_string(), false),
                None => ("Advisor consultation cancelled.".to_string(), false),
            };
            Ok(boxed_tool_output(FunctionToolOutput::from_text(
                message,
                Some(success),
            )))
        })
    }
}

impl CoreToolRuntime for AdvisorHandler {}
