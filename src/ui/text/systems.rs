use crate::{prelude::*, ui::calc_line_height};

use std::sync::Arc;

use bevy::{
    math::Affine2,
    platform::collections::HashMap,
    render::{Extract, sync_world::TemporaryRenderEntity},
    text::{
        CosmicFontSystem, FontAtlasKey, FontAtlasSet, FontFaceInfo, FontSmoothing, LineHeight,
        PositionedGlyph, RunGeometry, SwashCache, TextBounds, TextLayoutInfo, add_glyph_to_atlas,
        get_glyph_atlas_info, load_font_to_fontdb,
    },
    ui_render::{
        ExtractedGlyph, ExtractedUiItem, ExtractedUiNode, ExtractedUiNodes, UiCameraMap,
        stack_z_offsets,
    },
};
use cosmic_text::{Attrs, AttrsList, BufferLine, Family, LineIter, Metrics, Shaping, Wrap};

#[derive(Debug)]
pub struct GlyphSectionInfo {
    id: AssetId<Font>,
    smoothing: FontSmoothing,
    font_size: f32,
    strikeout_offset: f32,
    stroke_size: f32,
    underline_offset: f32,
    font_weight: u16,
}
impl GlyphSectionInfo {
    pub fn new(
        id: AssetId<Font>,
        smoothing: FontSmoothing,
        font_size: f32,
        strikeout_offset: f32,
        stroke_size: f32,
        underline_offset: f32,
        font_weight: u16,
    ) -> Self {
        Self {
            id,
            smoothing,
            font_size,
            strikeout_offset,
            stroke_size,
            underline_offset,
            font_weight,
        }
    }
}

#[derive(Resource, Default, Debug)]
pub struct ConsoleTextPipeline {
    /// Identifies a font [`ID`](cosmic_text::fontdb::ID) by its [`Font`] [`Asset`](bevy_asset::Asset).
    pub map_handle_to_font_id: HashMap<AssetId<Font>, (cosmic_text::fontdb::ID, Arc<str>)>,
    /// Buffered vec for collecting info for glyph assembly.
    glyph_info: Vec<GlyphSectionInfo>,
}
impl ConsoleTextPipeline {
    /// Utilizes [`cosmic_text::Buffer`] to shape and layout text
    ///
    /// Negative or 0.0 font sizes will not be laid out.
    pub fn update_buffer(
        &mut self,
        fonts: &Assets<Font>,
        linebreak: LineBreak,
        bounds: TextBounds,
        scale_factor: f64,
        computed: &mut ComputedConsoleTextBlock,
        font_system: &mut CosmicFontSystem,
        text_font: &TextFont,
        settings: &ConsoleUiSettings,
        line_height: &LineHeight,
        view: &ConsoleBufferView,
        buffer: &ConsoleBuffer,
    ) -> Result<(), TextError> {
        computed.needs_rerender = false;

        let font_system = &mut font_system.0;

        // Return early if a font is not loaded yet.
        if !fonts.contains(text_font.font.id()) {
            return Err(TextError::NoSuchFont);
        }

        // Load Bevy fonts into cosmic-text's font system.
        let face_info = load_font_to_fontdb(
            text_font,
            font_system,
            &mut self.map_handle_to_font_id,
            fonts,
        );

        let attrs = get_attrs(
            0,
            text_font,
            *line_height,
            settings.font_color,
            &face_info,
            scale_factor,
        );

        // Update the buffer.
        let cosmic_buffer = &mut computed.buffer;
        cosmic_buffer.set_wrap(
            font_system,
            match linebreak {
                LineBreak::WordBoundary => Wrap::Word,
                LineBreak::AnyCharacter => Wrap::Glyph,
                LineBreak::WordOrCharacter => Wrap::WordOrGlyph,
                LineBreak::NoWrap => Wrap::None,
            },
        );

        // Parsing happens here.
        // TODO: Further split these into stylized spans.
        // ANSI text should be escaped into subspans with colors &c
        // Styling will affect the number of characters displayed, so it needs to happen
        // _before_ populating the cosmic_buffer.
        cosmic_buffer.lines.clear();
        cosmic_buffer.set_size(font_system, bounds.width, bounds.height);
        let view_range = view.range; // TEMP
        let buffer = buffer
            .as_lines()
            .into_iter()
            .rev()
            .skip(view.start)
            .take(view_range)
            .map(|v| v.into_iter().collect::<String>())
            .collect::<Vec<String>>();

        let mut count = 0;
        for (i, raw_str) in buffer.iter().enumerate() {
            // todo: cache
            for (range, ending) in LineIter::new(raw_str) {
                cosmic_buffer.lines.push(BufferLine::new(
                    &raw_str[range],
                    ending,
                    AttrsList::new(&attrs),
                    Shaping::Advanced,
                ));
                let layout_lines = cosmic_buffer.line_layout(font_system, i);
                count += layout_lines.map(|v| v.len()).unwrap_or_default();
                if count >= view_range {
                    break;
                }
            }
            if count >= view_range {
                break;
            }
        }
        Ok(())
    }

    // /// Queues text for measurement
    // ///
    // /// Produces a [`TextMeasureInfo`] which can be used by a layout system
    // /// to measure the text area on demand.
    // pub fn _create_text_measure(
    //     &mut self,
    //     entity: Entity,
    //     fonts: &Assets<Font>,
    //     scale_factor: f64,
    //     layout: &ConsoleTextLayout,
    //     computed: &mut ComputedConsoleTextBlock,
    //     font_system: &mut CosmicFontSystem,
    //     text_font: &TextFont,
    //     settings: &ConsoleUiSettings,
    //     line_height: &LineHeight,
    //     buffer: &ConsoleBuffer,
    //     view: &ConsoleBufferView,
    //     prompt: &ConsolePrompt,
    //     console: &Console,
    // ) -> Result<TextMeasureInfo, TextError> {
    //     const MIN_WIDTH_CONTENT_BOUNDS: TextBounds = TextBounds::new_horizontal(0.0);

    //     // Clear this here at the focal point of measured text rendering to ensure the field's lifecycle has
    //     // strong boundaries.
    //     computed.needs_rerender = false;

    //     self.update_buffer(
    //         fonts,
    //         layout.linebreak,
    //         MIN_WIDTH_CONTENT_BOUNDS,
    //         scale_factor,
    //         computed,
    //         font_system,
    //         text_font,
    //         settings,
    //         line_height,
    //         view,
    //         buffer,
    //         prompt,
    //         console,
    //     )?;

    //     let buffer = &mut computed.buffer;
    //     let min_width_content_size = buffer_dimensions(buffer);

    //     let max_width_content_size = {
    //         let font_system = &mut font_system.0;
    //         buffer.set_size(font_system, None, None);
    //         buffer_dimensions(buffer)
    //     };

    //     Ok(TextMeasureInfo {
    //         min: min_width_content_size,
    //         max: max_width_content_size,
    //         entity,
    //     })
    // }

    pub fn update_layout_info(
        &mut self,
        layout_info: &mut TextLayoutInfo,
        computed: &mut ComputedConsoleTextBlock,
        view: &ConsoleBufferView,
        bounds: TextBounds,
        q_font: Query<&TextFont>,
        font_system: &mut CosmicFontSystem,
        scale_factor: f64,
        font_atlas_set: &mut FontAtlasSet,
        texture_atlases: &mut Assets<TextureAtlasLayout>,
        textures: &mut Assets<Image>,
        swash_cache: &mut SwashCache,
        node: &ComputedNode,
    ) -> Result<(), TextError> {
        layout_info.glyphs.clear();
        layout_info.run_geometry.clear();
        layout_info.size = Vec2::default();
        self.glyph_info.clear();
        // NOTE: This originally had an iter_many over the contents of the text node's span children.
        // We don't use children so no need to do that.
        for text_font in q_font {
            let mut section_info = GlyphSectionInfo::new(
                text_font.font.id(),
                text_font.font_smoothing,
                text_font.font_size,
                0.0,
                0.0,
                0.0,
                text_font.weight.clamp().0,
            );

            if let Some((id, _)) = self.map_handle_to_font_id.get(&section_info.id)
                && let Some(font) =
                    font_system.get_font(*id, cosmic_text::Weight(section_info.font_weight))
            {
                let swash = font.as_swash();
                let metrics = swash.metrics(&[]);
                let upem = metrics.units_per_em as f32;
                let scalar = section_info.font_size * scale_factor as f32 / upem;
                section_info.strikeout_offset = (metrics.strikeout_offset * scalar).round();
                section_info.stroke_size = (metrics.stroke_size * scalar).round().max(1.);
                section_info.underline_offset = (metrics.underline_offset * scalar).round();
            }
            self.glyph_info.push(section_info);
        }

        let buffer = &mut computed.buffer;

        buffer.set_size(font_system, bounds.width, bounds.height);
        let mut box_size = Vec2::ZERO;

        // Buffer is stored bottom-up
        let mut res: Result<(), TextError> = Ok(());
        for run in buffer.layout_runs() {
            box_size.x = box_size.x.max(run.line_w);
            box_size.y += run.line_height;
            if box_size.y >= view.range as f32 * run.line_height {
                break;
            }
            let mut current_section: Option<usize> = None;
            let mut start = 0.;
            let mut end = 0.;
            res = run
                .glyphs
                .iter()
                .map(move |layout_glyph| (layout_glyph, run.line_y, run.line_i))
                .try_for_each(|(layout_glyph, line_y, line_i)| {
                    // set start, end, layout info.
                    if let Some(section) = current_section {
                        if section != layout_glyph.metadata {
                            layout_info.run_geometry.push(RunGeometry {
                                span_index: section,
                                bounds: Rect::new(
                                    start,
                                    run.line_top,
                                    end,
                                    run.line_top + run.line_height,
                                ),
                                strikethrough_y: (run.line_y
                                    - self.glyph_info[section].strikeout_offset),
                                strikethrough_thickness: self.glyph_info[section].stroke_size,
                                underline_y: (run.line_y
                                    - self.glyph_info[section].underline_offset)
                                    .round(),
                                underline_thickness: self.glyph_info[section].stroke_size,
                            });
                            start = end.max(layout_glyph.x);
                            current_section = Some(layout_glyph.metadata);
                        }
                        end = layout_glyph.x + layout_glyph.w;
                    } else {
                        current_section = Some(layout_glyph.metadata);
                        start = layout_glyph.x;
                        end = start + layout_glyph.w;
                    }

                    let mut temp_glyph;
                    let span_index = layout_glyph.metadata;
                    let font_id = self.glyph_info[span_index].id;
                    let font_smoothing = self.glyph_info[span_index].smoothing;

                    let layout_glyph = if font_smoothing == FontSmoothing::None {
                        // If font smoothing is disabled, round the glyph positions and sizes,
                        // effectively discarding all subpixel layout.
                        temp_glyph = layout_glyph.clone();
                        temp_glyph.x = temp_glyph.x.round();
                        temp_glyph.y = temp_glyph.y.round();
                        temp_glyph.w = temp_glyph.w.round();
                        temp_glyph.x_offset = temp_glyph.x_offset.round();
                        temp_glyph.y_offset = temp_glyph.y_offset.round();
                        temp_glyph.line_height_opt = temp_glyph.line_height_opt.map(f32::round);

                        &temp_glyph
                    } else {
                        layout_glyph
                    };

                    let physical_glyph = layout_glyph.physical((0., 0.), 1.);
                    let font_atlases = font_atlas_set
                        .entry(FontAtlasKey(
                            font_id,
                            physical_glyph.cache_key.font_size_bits,
                            font_smoothing,
                        ))
                        .or_default();

                    let atlas_info = get_glyph_atlas_info(font_atlases, physical_glyph.cache_key)
                        .map(Ok)
                        .unwrap_or_else(|| {
                            add_glyph_to_atlas(
                                font_atlases,
                                texture_atlases,
                                textures,
                                &mut font_system.0,
                                &mut swash_cache.0,
                                layout_glyph,
                                font_smoothing,
                            )
                        })?;

                    let texture_atlas = texture_atlases.get(atlas_info.texture_atlas).unwrap();
                    let location = atlas_info.location;
                    let glyph_rect = texture_atlas.textures[location.glyph_index];
                    let left = location.offset.x as f32;
                    let top = location.offset.y as f32;
                    let glyph_size = UVec2::new(glyph_rect.width(), glyph_rect.height());

                    // offset by half the size because the origin is center
                    let x = glyph_size.x as f32 / 2.0 + left + physical_glyph.x as f32;
                    let y = (node.size.y.round() - line_y.round()) + physical_glyph.y as f32 - top
                        + glyph_size.y as f32 / 2.0;

                    // invert position - console grows from bottom to top
                    let position = Vec2::new(x, y);

                    let pos_glyph = PositionedGlyph {
                        position,
                        size: glyph_size.as_vec2(),
                        atlas_info,
                        span_index,
                        byte_index: layout_glyph.start,
                        byte_length: layout_glyph.end - layout_glyph.start,
                        line_index: line_i,
                    };
                    layout_info.glyphs.push(pos_glyph);
                    Ok(())
                });

            if let Some(section) = current_section {
                layout_info.run_geometry.push(RunGeometry {
                    span_index: section,
                    bounds: Rect::new(start, run.line_top, end, run.line_top + run.line_height),
                    strikethrough_y: (run.line_y - self.glyph_info[section].strikeout_offset)
                        .round(),
                    strikethrough_thickness: self.glyph_info[section].stroke_size,
                    underline_y: (run.line_y - self.glyph_info[section].underline_offset).round(),
                    underline_thickness: self.glyph_info[section].stroke_size,
                });
            }
        }
        res?;

        layout_info.size = box_size.ceil();
        Ok(())
    }
}

/// Translates [`TextFont`] to [`Attrs`].
fn get_attrs<'a>(
    span_index: usize,
    text_font: &TextFont,
    line_height: LineHeight,
    color: Color,
    face_info: &'a FontFaceInfo,
    scale_factor: f64,
) -> Attrs<'a> {
    Attrs::new()
        .metadata(span_index)
        .family(Family::Name(&face_info.family_name))
        .stretch(face_info.stretch)
        .style(face_info.style)
        .weight(text_font.weight.into())
        .metrics(
            Metrics {
                font_size: text_font.font_size,
                line_height: calc_line_height(&line_height, text_font.font_size),
            }
            .scale(scale_factor as f32),
        )
        .font_features((&text_font.font_features).into())
        .color(cosmic_text::Color(color.to_linear().as_u32()))
}

pub fn update_buffer(
    fonts: Res<Assets<Font>>,
    mut text_query: Query<
        (
            Ref<ConsoleTextLayout>,
            &mut ConsoleBufferFlags,
            &mut ComputedConsoleTextBlock,
            Ref<ComputedUiRenderTargetInfo>,
            &ComputedNode,
            Ref<FontHinting>,
            &TextFont,
            &ConsoleUiSettings,
            &LineHeight,
            &ConsoleBuffer,
            &ConsoleBufferView,
        ),
        With<Node>,
    >,
    mut text_pipeline: ResMut<ConsoleTextPipeline>,
    mut font_system: ResMut<CosmicFontSystem>,
) {
    for (
        layout,
        mut text_flags,
        mut computed,
        computed_target,
        computed_node,
        hinting,
        text_font,
        settings,
        line_height,
        buffer,
        view,
    ) in &mut text_query
    {
        // Note: the ComputedTextBlock::needs_rerender bool is cleared in create_text_measure().
        // 1e-5 epsilon to ignore tiny scale factor float errors
        if !(1e-5
            < (computed_target.scale_factor() - computed_node.inverse_scale_factor.recip()).abs()
            || computed.needs_rerender
            || text_flags.needs_measure_fn
            || hinting.is_changed())
        {
            continue;
        }

        // Clear this here at the focal point of measured text rendering to ensure the field's lifecycle has
        // strong boundaries.
        computed.needs_rerender = false;

        match text_pipeline.update_buffer(
            &fonts,
            layout.linebreak,
            TextBounds {
                width: Some(computed_node.size.x),
                height: Some(computed_node.size.y),
            },
            computed_target.scale_factor().into(),
            &mut computed,
            &mut font_system,
            text_font,
            settings,
            line_height,
            view,
            buffer,
        ) {
            Ok(_) => {
                text_flags.needs_measure_fn = false;
                text_flags.needs_recompute = true;
            }
            Err(TextError::NoSuchFont) => {
                // retry next frame
                computed.needs_rerender = true;
                text_flags.needs_measure_fn = true;
            }
            Err(
                e @ (TextError::FailedToAddGlyph(_)
                | TextError::FailedToGetGlyphImage(_)
                | TextError::MissingAtlasLayout
                | TextError::MissingAtlasTexture
                | TextError::InconsistentAtlasState),
            ) => {
                panic!("Fatal error when processing text: {e}.");
            }
        };

        // match text_pipeline.create_text_measure(
        //     entity,
        //     fonts.as_ref(),
        //     computed_target.scale_factor().into(),
        //     &layout,
        //     computed.as_mut(),
        //     &mut font_system,
        //     text_font,
        //     settings,
        //     line_height,
        //     buffer,
        //     view,
        //     prompt,
        //     console,
        // ) {
        //     Ok(measure) => {
        //         // Text measure func created successfully, so set `TextNodeFlags` to schedule a recompute
        //         text_flags.needs_recompute = true;
        //     }
        //     Err(TextError::NoSuchFont) => {
        //         // Try again next frame
        //     }
        //     Err(
        //         e @ (TextError::FailedToAddGlyph(_)
        //         | TextError::FailedToGetGlyphImage(_)
        //         | TextError::MissingAtlasLayout
        //         | TextError::MissingAtlasTexture
        //         | TextError::InconsistentAtlasState),
        //     ) => {
        //         panic!("Fatal error when processing text: {e}.");
        //     }
        // };
    }
}
pub fn update_console_text_layout(
    mut pipeline: ResMut<ConsoleTextPipeline>,
    console_q: Query<(
        Ref<ComputedNode>,
        &ConsoleTextLayout,
        &ConsoleBufferView,
        &mut TextLayoutInfo,
        &mut ConsoleBufferFlags,
        &mut ComputedConsoleTextBlock,
    )>,
    text_font: Query<&TextFont>,
    mut font_system: ResMut<CosmicFontSystem>,
    mut font_atlas_set: ResMut<FontAtlasSet>,
    mut texture_atlases: ResMut<Assets<TextureAtlasLayout>>,
    mut textures: ResMut<Assets<Image>>,
    mut swash_cache: ResMut<SwashCache>,
) {
    for (node, layout, view, mut layout_info, mut flags, mut computed) in console_q {
        if node.is_changed() || flags.needs_recompute {
            let scale_factor = node.inverse_scale_factor().recip().into();
            let physical_node_size = if layout.linebreak == LineBreak::NoWrap {
                // With `NoWrap` set, no constraints are placed on the width of the text.
                TextBounds::UNBOUNDED
            } else {
                // `scale_factor` is already multiplied by `UiScale`
                TextBounds::new(node.unrounded_size.x, node.unrounded_size.y)
            };
            match pipeline.update_layout_info(
                &mut layout_info,
                &mut computed,
                view,
                physical_node_size,
                text_font,
                &mut font_system,
                scale_factor,
                &mut font_atlas_set,
                &mut texture_atlases,
                &mut textures,
                &mut swash_cache,
                &node,
            ) {
                Ok(()) => {
                    layout_info.scale_factor = scale_factor as f32;
                    layout_info.size *= node.inverse_scale_factor();
                    flags.needs_recompute = false;
                }
                Err(TextError::NoSuchFont) => {
                    // There was an error processing the text layout, try again next frame
                    flags.needs_recompute = true;
                }
                Err(
                    e @ (TextError::FailedToAddGlyph(_)
                    | TextError::FailedToGetGlyphImage(_)
                    | TextError::MissingAtlasLayout
                    | TextError::MissingAtlasTexture
                    | TextError::InconsistentAtlasState),
                ) => {
                    panic!("Fatal error when processing text: {e}.");
                }
            }
        }
    }
}

// If we can use a ComputedTextBlock above, then we won't need this function.
/// Extracts the console glyphs for rendering
pub fn extract_console_text_sections(
    mut commands: Commands,
    mut extracted_uinodes: ResMut<ExtractedUiNodes>,
    texture_atlases: Extract<Res<Assets<TextureAtlasLayout>>>,
    uinode_query: Extract<
        Query<(
            Entity,
            &ComputedNode,
            &UiGlobalTransform,
            &InheritedVisibility,
            Option<&CalculatedClip>,
            &ComputedUiTargetCamera,
            &ComputedConsoleTextBlock,
            &TextColor,
            &TextLayoutInfo,
        )>,
    >,
    text_styles: Extract<Query<&TextColor>>,
    camera_map: Extract<UiCameraMap>,
) {
    let mut start = extracted_uinodes.glyphs.len();
    let mut end = start + 1;

    let mut camera_mapper = camera_map.get_mapper();
    for (
        entity,
        uinode,
        transform,
        inherited_visibility,
        clip,
        camera,
        computed_block,
        text_color,
        text_layout_info,
    ) in &uinode_query
    {
        // Skip if not visible or if size is set to zero (e.g. when a parent is set to `Display::None`)
        if !inherited_visibility.get() || uinode.is_empty() {
            continue;
        }

        let Some(extracted_camera_entity) = camera_mapper.map(camera) else {
            continue;
        };

        let transform = Affine2::from(*transform) * Affine2::from_translation(-0.5 * uinode.size());

        let mut color = text_color.0.to_linear();

        let mut current_span_index = 0;

        for (
            i,
            PositionedGlyph {
                position,
                atlas_info,
                span_index,
                ..
            },
        ) in text_layout_info.glyphs.iter().enumerate()
        {
            if current_span_index != *span_index
                && let Some(span_entity) =
                    computed_block.entities.get(*span_index).map(|t| t.entity)
            {
                color = text_styles
                    .get(span_entity)
                    .map(|text_color| LinearRgba::from(text_color.0))
                    .unwrap_or_default();
                current_span_index = *span_index;
            }

            let rect = texture_atlases
                .get(atlas_info.texture_atlas)
                .unwrap()
                .textures[atlas_info.location.glyph_index]
                .as_rect();
            extracted_uinodes.glyphs.push(ExtractedGlyph {
                color,
                translation: *position,
                rect,
            });

            if text_layout_info
                .glyphs
                .get(i + 1)
                .is_none_or(|info| info.atlas_info.texture != atlas_info.texture)
            {
                extracted_uinodes.uinodes.push(ExtractedUiNode {
                    z_order: uinode.stack_index as f32 + stack_z_offsets::TEXT,
                    render_entity: commands.spawn(TemporaryRenderEntity).id(),
                    image: atlas_info.texture,
                    clip: clip.map(|clip| clip.clip),
                    extracted_camera_entity,
                    item: ExtractedUiItem::Glyphs { range: start..end },
                    main_entity: entity.into(),
                    transform,
                });
                start = end;
            }

            end += 1;
        }
    }
}
