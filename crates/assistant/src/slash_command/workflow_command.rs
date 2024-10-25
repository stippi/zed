use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use anyhow::Result;
use assistant_slash_command::{
    ArgumentCompletion, SlashCommand, SlashCommandOutput, SlashCommandOutputSection,
    SlashCommandResult,
};
use gpui::{Task, WeakView};
use language::{BufferSnapshot, LspAdapterDelegate};
use ui::prelude::*;
use workspace::Workspace;

use crate::prompts::PromptBuilder;

pub(crate) struct WorkflowSlashCommand {
    prompt_builder: Arc<PromptBuilder>,
}

impl WorkflowSlashCommand {
    pub const NAME: &'static str = "workflow";

    pub fn new(prompt_builder: Arc<PromptBuilder>) -> Self {
        Self { prompt_builder }
    }
}

impl SlashCommand for WorkflowSlashCommand {
    fn name(&self) -> String {
        Self::NAME.into()
    }

    fn description(&self) -> String {
        "Insert prompt to opt into the edit workflow".into()
    }

    fn menu_text(&self) -> String {
        self.description()
    }

    fn requires_argument(&self) -> bool {
        false
    }

    fn complete_argument(
        self: Arc<Self>,
        _arguments: &[String],
        _cancel: Arc<AtomicBool>,
        _workspace: Option<WeakView<Workspace>>,
        _cx: &mut WindowContext,
    ) -> Task<Result<Vec<ArgumentCompletion>>> {
        Task::ready(Ok(Vec::new()))
    }

    fn run(
        self: Arc<Self>,
        _arguments: &[String],
        _context_slash_command_output_sections: &[SlashCommandOutputSection<language::Anchor>],
        _context_buffer: BufferSnapshot,
        _workspace: WeakView<Workspace>,
        _delegate: Option<Arc<dyn LspAdapterDelegate>>,
        cx: &mut WindowContext,
    ) -> Task<SlashCommandResult> {
        let prompt_builder = self.prompt_builder.clone();
        cx.spawn(|_cx| async move {
            let text = prompt_builder.generate_workflow_prompt()?;
            let range = 0..text.len();

            Ok(SlashCommandOutput {
                text,
                sections: vec![SlashCommandOutputSection {
                    range,
                    icon: IconName::Route,
                    label: "Workflow".into(),
                    metadata: None,
                }],
                run_commands_in_text: false,
            }
            .to_event_stream())
        })
    }
}
