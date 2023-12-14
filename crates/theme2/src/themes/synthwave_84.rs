// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

#[allow(unused)]
use crate::{
    Appearance, StatusColorsRefinement, ThemeColorsRefinement, UserFontStyle, UserFontWeight,
    UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn synthwave_84() -> UserThemeFamily {
    UserThemeFamily {
        name: "Synthwave 84".into(),
        author: "Robb Owen (robb0wen)".into(),
        themes: vec![UserTheme {
            name: "Synthwave 84".into(),
            appearance: Appearance::Dark,
            styles: UserThemeStylesRefinement {
                colors: ThemeColorsRefinement {
                    border_focused: Some(rgba(0x1f212bff).into()),
                    elevated_surface_background: Some(rgba(0x232530ff).into()),
                    background: Some(rgba(0x262335ff).into()),
                    element_background: Some(rgba(0x614d85ff).into()),
                    element_hover: Some(rgba(0x37294d99).into()),
                    element_selected: Some(rgba(0xffffff20).into()),
                    drop_target_background: Some(rgba(0x34294f66).into()),
                    ghost_element_hover: Some(rgba(0x37294d99).into()),
                    ghost_element_selected: Some(rgba(0xffffff20).into()),
                    text: Some(rgba(0xffffffff).into()),
                    status_bar_background: Some(rgba(0x241b2fff).into()),
                    title_bar_background: Some(rgba(0x241b2fff).into()),
                    toolbar_background: Some(rgba(0x262335ff).into()),
                    tab_bar_background: Some(rgba(0x241b2fff).into()),
                    tab_inactive_background: Some(rgba(0x262335ff).into()),
                    tab_active_background: Some(rgba(0x262335ff).into()),
                    scrollbar_thumb_background: Some(rgba(0x9d8bca30).into()),
                    scrollbar_thumb_hover_background: Some(rgba(0x9d8bca50).into()),
                    scrollbar_thumb_border: Some(rgba(0x9d8bca30).into()),
                    scrollbar_track_background: Some(rgba(0x262335ff).into()),
                    scrollbar_track_border: Some(rgba(0x34294fb3).into()),
                    editor_foreground: Some(rgba(0xffffffff).into()),
                    editor_background: Some(rgba(0x262335ff).into()),
                    editor_gutter_background: Some(rgba(0x262335ff).into()),
                    editor_line_number: Some(rgba(0xffffff73).into()),
                    terminal_ansi_bright_red: Some(rgba(0xfe4450ff).into()),
                    terminal_ansi_bright_green: Some(rgba(0x72f1b8ff).into()),
                    terminal_ansi_bright_yellow: Some(rgba(0xfede5dff).into()),
                    terminal_ansi_bright_blue: Some(rgba(0x03edf9ff).into()),
                    terminal_ansi_bright_magenta: Some(rgba(0xff7edbff).into()),
                    terminal_ansi_bright_cyan: Some(rgba(0x03edf9ff).into()),
                    terminal_ansi_red: Some(rgba(0xfe4450ff).into()),
                    terminal_ansi_green: Some(rgba(0x72f1b8ff).into()),
                    terminal_ansi_yellow: Some(rgba(0xf3e70fff).into()),
                    terminal_ansi_blue: Some(rgba(0x03edf9ff).into()),
                    terminal_ansi_magenta: Some(rgba(0xff7edbff).into()),
                    terminal_ansi_cyan: Some(rgba(0x03edf9ff).into()),
                    ..Default::default()
                },
                status: StatusColorsRefinement {
                    created: Some(rgba(0x206d4bd6).into()),
                    deleted: Some(rgba(0xfa2e46a4).into()),
                    error: Some(rgba(0xfe4450ff).into()),
                    hint: Some(rgba(0x969696ff).into()),
                    ignored: Some(rgba(0xffffff59).into()),
                    modified: Some(rgba(0xb893ce8f).into()),
                    warning: Some(rgba(0x72f1b8bb).into()),
                    ..Default::default()
                },
                syntax: Some(UserSyntaxTheme {
                    highlights: vec![
                        (
                            "attribute".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfede5dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "boolean".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf97e72ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "comment".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x848bbdff).into()),
                                font_style: Some(UserFontStyle::Italic),
                                ..Default::default()
                            },
                        ),
                        (
                            "comment.doc".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x848bbdff).into()),
                                font_style: Some(UserFontStyle::Italic),
                                ..Default::default()
                            },
                        ),
                        (
                            "constant".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf97e72ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "constructor".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x72f1b8ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "function".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x36f9f6ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "keyword".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfede5dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "label".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfe4450ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_text".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xdd5500ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "link_uri".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xdd5500ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "number".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xf97e72ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "operator".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfede5dff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "property".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xff7edbff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x36f9f6ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.bracket".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x36f9f6ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.delimiter".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x36f9f6ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.list_marker".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x36f9f6ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "punctuation.special".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x36f9f6ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "tag".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0x72f1b8ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "title".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfe4450ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "type".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfe4450ff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variable".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xff7edbff).into()),
                                ..Default::default()
                            },
                        ),
                        (
                            "variable.special".into(),
                            UserHighlightStyle {
                                color: Some(rgba(0xfe4450ff).into()),
                                font_weight: Some(UserFontWeight(700.0)),
                                ..Default::default()
                            },
                        ),
                    ],
                }),
            },
        }],
    }
}
