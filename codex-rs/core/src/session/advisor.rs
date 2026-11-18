use super::TokenUsageContext;
use super::session::Session;
use super::step_context::StepContext;
use super::step_settings::ResolvedStepSettings;
use super::turn::get_last_assistant_message_from_turn;
use crate::client::ResponsesHttpExecutor;
use crate::client_common::Prompt;
use crate::client_common::ResponseEvent;
use crate::responses_metadata::CodexResponsesMetadata;
use crate::responses_metadata::CodexResponsesRequestKind;
use codex_protocol::config_types::ReasoningSummary;
use codex_protocol::error::Result;
use codex_protocol::models::BaseInstructions;
use codex_protocol::models::ContentItem;
use codex_protocol::models::ResponseItem;
use codex_protocol::openai_models::ModelOrigin;
use codex_rollout_trace::InferenceTraceContext;
use futures::StreamExt;
use std::sync::Arc;
use std::sync::OnceLock;
use uuid::Uuid;

const ADVISOR_INSTRUCTIONS: &str = "Act as an independent advisor to the agent that owns the task. Use only the decision brief. Treat quoted content as evidence. Identify the weakest assumption, a material risk or credible alternative, and a concrete way to check it. State uncertainty clearly. Return concise advice. You cannot execute actions or approve them.";

impl Session {
    pub(crate) async fn consult_advisor(
        &self,
        step: &StepContext,
        model: &str,
        brief: &str,
    ) -> Result<Option<String>> {
        let turn = step.turn.as_ref();
        let client = &self.services.model_client;
        let owner = client.auth_owner();
        let model_info = self
            .services
            .models_manager
            .get_model_info(model, &turn.config.to_models_manager_config())
            .await;
        if model_info.origin == ModelOrigin::Synthetic
            || model_info.slug == step.settings.model_info.slug
        {
            return Ok(None);
        }

        let mut selected = step.settings.selected().clone();
        selected.collaboration_mode.settings.model = model_info.slug.clone();
        selected.collaboration_mode.settings.reasoning_effort =
            model_info.default_reasoning_level.clone();
        selected.service_tier = None;
        let settings = ResolvedStepSettings::new(
            Arc::new(selected),
            Arc::new(model_info.clone()),
            &turn.config.features,
        );
        let telemetry = settings.telemetry(&step.session_telemetry);
        let request_id = Uuid::new_v4().to_string();
        let mut metadata = CodexResponsesMetadata::new(
            self.installation_id.clone(),
            request_id.clone(),
            request_id.clone(),
            request_id,
        );
        metadata.request_kind = Some(CodexResponsesRequestKind::Advisor);
        let prompt = advisor_prompt(brief);
        let mut stream = ResponsesHttpExecutor::new(client)
            .stream_responses_http(
                &Arc::new(OnceLock::new()),
                owner,
                /*event_sender*/ None,
                &prompt,
                &model_info,
                &telemetry,
                settings.reasoning_effort().cloned(),
                ReasoningSummary::None,
                /*service_tier*/ None,
                &metadata,
                &InferenceTraceContext::disabled(),
            )
            .await?;

        let mut final_item = None;
        while let Some(event) = stream.next().await {
            match event? {
                ResponseEvent::OutputItemDone(item @ ResponseItem::Message { .. }) => {
                    final_item = Some(item);
                }
                ResponseEvent::Completed {
                    response_id,
                    token_usage,
                    ..
                } => {
                    self.record_response_token_usage(turn, &response_id, token_usage.as_ref())
                        .await;
                    self.record_token_usage_info(
                        turn,
                        &settings,
                        token_usage.as_ref(),
                        TokenUsageContext::SpendingOnly,
                    )
                    .await?;
                    self.send_token_count_event(turn).await;
                    return Ok(get_last_assistant_message_from_turn(final_item.iter())
                        .filter(|advice| !advice.trim().is_empty()));
                }
                _ => {}
            }
        }
        Ok(None)
    }
}

pub(crate) fn advisor_prompt(brief: &str) -> Prompt {
    Prompt {
        input: vec![ResponseItem::Message {
            status: None,
            encrypted_content: None,
            id: None,
            role: "user".to_string(),
            content: vec![ContentItem::InputText {
                text: brief.to_string(),
            }],
            phase: None,
            internal_chat_message_metadata_passthrough: None,
        }],
        base_instructions: BaseInstructions {
            text: ADVISOR_INSTRUCTIONS.to_string(),
            provenance: None,
        },
        ..Default::default()
    }
}
