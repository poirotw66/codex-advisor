use anyhow::Result;
use codex_models_manager::bundled_models_response;
use codex_protocol::openai_models::ModelVisibility;
use codex_protocol::openai_models::ModelsResponse;
use codex_protocol::openai_models::ToolMode;
use core_test_support::responses;
use core_test_support::skip_if_no_network;
use core_test_support::test_codex::test_codex;
use pretty_assertions::assert_eq;
use serde_json::json;

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn advisor_consults_a_different_model_and_returns_advice_to_the_parent() -> Result<()> {
    skip_if_no_network!(Ok(()));
    let server = responses::start_mock_server().await;
    let brief = "Check whether the migration can be rolled back.";
    let advice = "Test the rollback against a copy of the migrated data.";
    let call =
        responses::ev_function_call("consult", "advisor", &json!({ "brief": brief }).to_string());
    let mock = responses::mount_sse_sequence(
        &server,
        vec![
            responses::sse(vec![call, responses::ev_completed("parent-1")]),
            responses::sse(vec![
                responses::ev_assistant_message("advisor-message", advice),
                responses::ev_completed("advisor-1"),
            ]),
            responses::sse_completed("parent-2"),
        ],
    )
    .await;

    let test = test_codex()
        .with_model("executor-example")
        .with_config(|config| {
            let template = bundled_models_response().unwrap().models.remove(0);
            config.model_catalog = Some(ModelsResponse {
                models: ["executor-example", "advisor-example"]
                    .into_iter()
                    .map(|model| {
                        let mut info = template.clone();
                        info.slug = model.to_string();
                        info.visibility = ModelVisibility::List;
                        info.supported_in_api = true;
                        info.use_responses_lite = false;
                        info.tool_mode = Some(ToolMode::Direct);
                        info
                    })
                    .collect(),
            });
            config.advisor_model = Some("advisor-example".to_string());
        })
        .build_with_auto_env(&server)
        .await?;
    let main_task = "Decide whether to migrate with a second opinion.";
    test.submit_turn(main_task).await?;

    let requests = mock.requests();
    assert_eq!(requests.len(), 3);
    let first = requests[0].body_json();
    let advisor = requests[1].body_json();
    let continuation = requests[2].body_json();
    assert_eq!(first["model"], json!("executor-example"));
    assert!(
        first["tools"]
            .as_array()
            .unwrap()
            .iter()
            .any(|tool| tool["name"] == "advisor")
    );
    assert_eq!(advisor["model"], json!("advisor-example"));
    assert_eq!(advisor["tool_choice"], json!("none"));
    assert_eq!(advisor["tools"], json!([]));
    assert!(advisor["input"].to_string().contains(brief));
    assert!(!advisor["input"].to_string().contains(main_task));
    assert_eq!(continuation["model"], json!("executor-example"));
    let (output, _) = requests[2]
        .function_call_output_content_and_success("consult")
        .expect("advisor result should be passed back to the parent");
    assert!(output.unwrap().contains(advice));
    Ok(())
}
