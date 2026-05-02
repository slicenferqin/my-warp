use warp_core::{context_flag::ContextFlag, features::FeatureFlag};
use warpui::ViewContext;

use super::{
    ContentItem, ContentSectionData, FeatureItem, FeatureSection, FeatureSectionData,
    ResourceCenterMainView, Section, Tip, TipAction, TipHint,
};

pub fn sections(ctx: &mut ViewContext<ResourceCenterMainView>) -> Vec<Section> {
    let mut sections = vec![Section::Changelog()];

    if FeatureFlag::AvatarInTabBar.is_enabled() {
        return sections;
    }

    let get_started = FeatureSectionData {
        section_name: FeatureSection::GettingStarted,
        items: vec![
            FeatureItem::new(
                warp_i18n::tr("resource-center-create-first-block-title"),
                warp_i18n::tr("resource-center-create-first-block-description"),
                Tip::Hint(TipHint::CreateBlock),
                ctx,
            ),
            FeatureItem::new(
                warp_i18n::tr("resource-center-navigate-blocks-title"),
                warp_i18n::tr("resource-center-navigate-blocks-description"),
                Tip::Hint(TipHint::BlockSelect),
                ctx,
            ),
            FeatureItem::new(
                warp_i18n::tr("resource-center-block-action-title"),
                warp_i18n::tr("resource-center-block-action-description"),
                Tip::Hint(TipHint::BlockAction),
                ctx,
            ),
            FeatureItem::new(
                warp_i18n::tr("resource-center-open-command-palette-title"),
                warp_i18n::tr("resource-center-open-command-palette-description"),
                Tip::Action(TipAction::CommandPalette),
                ctx,
            ),
            FeatureItem::new(
                warp_i18n::tr("resource-center-set-theme-title"),
                warp_i18n::tr("resource-center-set-theme-description"),
                Tip::Action(TipAction::ThemePicker),
                ctx,
            ),
        ],
    };
    sections.push(Section::Feature(get_started));

    let maximize_warp = FeatureSectionData {
        section_name: FeatureSection::MaximizeWarp,
        items: maximize_warp_items(ctx),
    };
    sections.push(Section::Feature(maximize_warp));

    let advanced_setup = ContentSectionData {
        section_name: FeatureSection::AdvancedSetup,
        items: vec![
            ContentItem {
                title: warp_i18n::tr("resource-center-custom-prompt-title"),
                description: warp_i18n::tr("resource-center-custom-prompt-description"),
                url: "https://docs.warp.dev/terminal/appearance/prompt",
                button_label: warp_i18n::tr("resource-center-view-documentation"),
            },
            ContentItem {
                title: warp_i18n::tr("resource-center-ide-integration-title"),
                description: warp_i18n::tr("resource-center-ide-integration-description"),
                url: "https://docs.warp.dev/terminal/integrations-and-plugins",
                button_label: warp_i18n::tr("resource-center-view-documentation"),
            },
            ContentItem {
                title: warp_i18n::tr("resource-center-how-warp-uses-warp-title"),
                description: warp_i18n::tr("resource-center-how-warp-uses-warp-description"),
                url: "https://www.warp.dev/blog/how-warp-uses-warp",
                button_label: warp_i18n::tr("resource-center-read-article"),
            },
        ],
    };
    sections.push(Section::Content(advanced_setup));

    sections
}

fn maximize_warp_items(ctx: &mut ViewContext<ResourceCenterMainView>) -> Vec<FeatureItem> {
    let mut maximize_warp_items = vec![];

    maximize_warp_items.push(FeatureItem::new(
        warp_i18n::tr("resource-center-command-search-title"),
        warp_i18n::tr("resource-center-command-search-description"),
        Tip::Action(TipAction::CommandSearch),
        ctx,
    ));

    maximize_warp_items.push(FeatureItem::new(
        warp_i18n::tr("resource-center-ai-command-search-title"),
        warp_i18n::tr("resource-center-ai-command-search-description"),
        Tip::Action(TipAction::AiCommandSearch),
        ctx,
    ));

    if ContextFlag::CreateNewSession.is_enabled() {
        maximize_warp_items.push(FeatureItem::new(
            warp_i18n::tr("resource-center-split-panes-title"),
            warp_i18n::tr("resource-center-split-panes-description"),
            Tip::Action(TipAction::SplitPane),
            ctx,
        ));
    }

    if ContextFlag::LaunchConfigurations.is_enabled() {
        maximize_warp_items.push(FeatureItem::new(
            warp_i18n::tr("resource-center-launch-config-title"),
            warp_i18n::tr("resource-center-launch-config-description"),
            Tip::Action(TipAction::SaveNewLaunchConfig),
            ctx,
        ));
    }

    maximize_warp_items
}
